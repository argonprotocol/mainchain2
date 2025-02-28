#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
extern crate core;

use codec::{Codec, Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, Saturating};

use argon_primitives::{block_seal::BlockPayout, BlockRewardsEventHandler};
pub use pallet::*;
pub use weights::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use core::fmt::Debug;
	use frame_support::{
		pallet_prelude::*,
		traits::fungible::{Inspect, Mutate},
	};
	use frame_system::pallet_prelude::*;
	use log::{trace, warn};
	use sp_arithmetic::FixedI128;
	use sp_core::U256;
	use sp_runtime::{traits::AtLeast32BitUnsigned, FixedPointNumber};

	use argon_primitives::{
		bitcoin::UtxoId, ArgonCPI, BlockRewardAccountsProvider, BurnEventHandler, PriceProvider,
		UtxoLockEvents,
	};

	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		type Currency: Mutate<Self::AccountId, Balance = Self::Balance>;
		type PriceProvider: PriceProvider<Self::Balance>;

		type Balance: AtLeast32BitUnsigned
			+ codec::FullCodec
			+ Copy
			+ MaybeSerializeDeserialize
			+ core::fmt::Debug
			+ Default
			+ From<u128>
			+ TryInto<u128>
			+ TypeInfo
			+ MaxEncodedLen;

		/// The maximum number of UTXOs that can be waiting for minting
		#[pallet::constant]
		type MaxPendingMintUtxos: Get<u32>;

		/// The provider of reward account ids
		type BlockRewardAccountsProvider: BlockRewardAccountsProvider<Self::AccountId>;
	}

	/// Bitcoin UTXOs that have been submitted for minting. This list is FIFO for minting whenever
	/// a) CPI >= 0 and
	/// b) the aggregate minted Bitcoins <= the aggregate minted Argons from mining
	#[pallet::storage]
	pub(super) type PendingMintUtxos<T: Config> = StorageValue<
		_,
		BoundedVec<(UtxoId, T::AccountId, T::Balance), T::MaxPendingMintUtxos>,
		ValueQuery,
	>;

	#[pallet::storage]
	pub(super) type MintedMiningArgons<T: Config> = StorageValue<_, U256, ValueQuery>;

	#[pallet::storage]
	pub(super) type MintedBitcoinArgons<T: Config> = StorageValue<_, U256, ValueQuery>;

	#[pallet::storage]
	pub(super) type BlockMintAction<T> =
		StorageValue<_, (BlockNumberFor<T>, MintAction<<T as Config>::Balance>), ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> {
		ArgonsMinted {
			mint_type: MintType,
			account_id: T::AccountId,
			utxo_id: Option<UtxoId>,
			amount: T::Balance,
		},
		MintError {
			mint_type: MintType,
			account_id: T::AccountId,
			utxo_id: Option<UtxoId>,
			amount: T::Balance,
			error: DispatchError,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyPendingMints,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		<T as Config>::Balance: Into<u128>,
		<T as Config>::Balance: From<u128>,
	{
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			let mut block_mint_action = <BlockMintAction<T>>::mutate(|(b, data)| {
				if *b != n {
					*b = n;
					*data = Default::default();
				}
				data.clone()
			});
			let argon_cpi = T::PriceProvider::get_argon_cpi().unwrap_or_default();
			// only mint when cpi is negative or 0
			if argon_cpi.is_positive() {
				trace!("Argon cpi is not-positive. Nothing to mint.");
				return T::DbWeight::get().reads(2);
			}

			// if there are no miners registered, we can't mint
			let reward_accounts = T::BlockRewardAccountsProvider::get_all_rewards_accounts();

			let argons_to_print_per_miner =
				Self::get_argons_to_print_per_miner(argon_cpi, reward_accounts.len() as u128);

			let mut bitcoin_mint = MintedBitcoinArgons::<T>::get();
			let mut mining_mint = MintedMiningArgons::<T>::get();

			if argons_to_print_per_miner > T::Balance::zero() {
				for miner in reward_accounts {
					let amount = argons_to_print_per_miner;
					match T::Currency::mint_into(&miner, amount) {
						Ok(_) => {
							mining_mint += U256::from(amount.into());
							block_mint_action.argon_minted += amount;
							Self::deposit_event(Event::<T>::ArgonsMinted {
								mint_type: MintType::Mining,
								account_id: miner.clone(),
								utxo_id: None,
								amount,
							});
						},
						Err(e) => {
							warn!(
								"Failed to mint {:?} argons for miner {:?}: {:?}",
								amount, &miner, e
							);
							Self::deposit_event(Event::<T>::MintError {
								mint_type: MintType::Mining,
								account_id: miner.clone(),
								utxo_id: None,
								amount,
								error: e,
							});
						},
					};
				}
				MintedMiningArgons::<T>::put(mining_mint);
			}

			let mut available_bitcoin_to_mint = mining_mint.saturating_sub(bitcoin_mint);
			if available_bitcoin_to_mint > U256::zero() {
				let updated = <PendingMintUtxos<T>>::get().try_mutate(|pending| {
					pending.retain_mut(|(utxo_id, account_id, remaining_account_mint)| {
						if available_bitcoin_to_mint == U256::zero() {
							return true;
						}

						let amount_to_mint = if available_bitcoin_to_mint >=
							U256::from((*remaining_account_mint).into())
						{
							*remaining_account_mint
						} else {
							// an account can't have more than u128 worth of argons
							available_bitcoin_to_mint.as_u128().into()
						};

						match T::Currency::mint_into(account_id, amount_to_mint) {
							Ok(_) => {
								available_bitcoin_to_mint -= U256::from(amount_to_mint.into());
								*remaining_account_mint -= amount_to_mint;
								bitcoin_mint += U256::from(amount_to_mint.into());
								block_mint_action.bitcoin_minted += amount_to_mint;

								Self::deposit_event(Event::<T>::ArgonsMinted {
									mint_type: MintType::Bitcoin,
									account_id: account_id.clone(),
									utxo_id: Some(*utxo_id),
									amount: amount_to_mint,
								});
							},
							Err(e) => {
								warn!(
									"Failed to mint {:?} argons for bitcoin UTXO {:?}: {:?}",
									amount_to_mint, &utxo_id, e
								);
								Self::deposit_event(Event::<T>::MintError {
									mint_type: MintType::Bitcoin,
									account_id: account_id.clone(),
									utxo_id: Some(*utxo_id),
									amount: amount_to_mint,
									error: e,
								});
							},
						};
						*remaining_account_mint > T::Balance::zero()
					});
				});
				PendingMintUtxos::<T>::put(updated.expect("cannot fail, but should be handled"));
			}
			MintedBitcoinArgons::<T>::put(bitcoin_mint);

			BlockMintAction::<T>::put((n, block_mint_action));
			T::DbWeight::get().reads_writes(1, 1)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> Pallet<T>
	where
		<T as Config>::Balance: Into<u128>,
		<T as Config>::Balance: From<u128>,
	{
		pub fn get_argons_to_print_per_miner(
			argon_cpi: ArgonCPI,
			active_miners: u128,
		) -> T::Balance {
			if !argon_cpi.is_negative() || active_miners == 0 {
				return T::Balance::zero();
			}
			let circulation: u128 = T::Currency::total_issuance().into();
			let circulation = FixedI128::saturating_from_integer(circulation);
			let argons_to_print =
				argon_cpi.saturating_abs().saturating_mul(circulation).saturating_mul_int(1u128);
			if argons_to_print == 0 {
				return T::Balance::zero();
			}
			// divide mint over an hour
			const MINT_TIME_SPREAD: u128 = 60;

			let per_miner = argons_to_print
				.checked_div(active_miners.saturating_mul(MINT_TIME_SPREAD))
				.unwrap_or_default();
			trace!(
				"Minting {} microgons. Circulation = {}. Per miner {}",
				argons_to_print,
				circulation.saturating_mul_int(1u128),
				per_miner
			);

			per_miner.into()
		}

		pub fn track_block_mint(amount: T::Balance) {
			BlockMintAction::<T>::mutate(|(b, data)| {
				let block = <frame_system::Pallet<T>>::block_number();
				if *b != block {
					*b = block;
					*data = Default::default();
				}
				data.argon_minted += amount;
			});
			let amount = U256::from(amount.into());
			MintedMiningArgons::<T>::mutate(|mint| *mint += amount);
		}

		pub fn on_argon_burn(amount: T::Balance) {
			let bitcoin_utxos = MintedBitcoinArgons::<T>::get();
			BlockMintAction::<T>::mutate(|(b, data)| {
				let block = <frame_system::Pallet<T>>::block_number();
				if *b != block {
					*b = block;
					*data = Default::default();
				}
				data.argon_burned += amount;
			});

			let mining_mint = MintedMiningArgons::<T>::get();
			let total_minted = mining_mint + bitcoin_utxos;
			let amount = U256::from(amount.into());
			let mining_prorata = (amount * mining_mint).checked_div(total_minted);
			if let Some(microgons) = mining_prorata {
				MintedMiningArgons::<T>::mutate(|mint| *mint -= microgons);
			}

			let bitcoin_prorata = (amount * bitcoin_utxos).checked_div(total_minted);
			if let Some(microgons) = bitcoin_prorata {
				MintedBitcoinArgons::<T>::mutate(|mint| *mint -= microgons);
			}
		}
	}

	impl<T: Config> UtxoLockEvents<T::AccountId, T::Balance> for Pallet<T>
	where
		<T as Config>::Balance: From<u128>,
		<T as Config>::Balance: Into<u128>,
	{
		fn utxo_locked(
			utxo_id: UtxoId,
			account_id: &T::AccountId,
			amount: T::Balance,
		) -> sp_runtime::DispatchResult {
			<PendingMintUtxos<T>>::try_mutate(|x| -> DispatchResult {
				x.try_push((utxo_id, account_id.clone(), amount))
					.map_err(|_| Error::<T>::TooManyPendingMints.into())
			})?;
			Ok(())
		}

		fn utxo_released(
			utxo_id: UtxoId,
			remove_pending_mints: bool,
			amount_burned: T::Balance,
		) -> sp_runtime::DispatchResult {
			if remove_pending_mints {
				<PendingMintUtxos<T>>::mutate(|x| {
					x.retain(|(id, _, _)| id != &utxo_id);
				});
			}

			let amount_burned: u128 = amount_burned.into();

			MintedBitcoinArgons::<T>::mutate(|mint| {
				*mint = mint.saturating_sub(U256::from(amount_burned))
			});
			Ok(())
		}
	}

	impl<T: Config> BurnEventHandler<T::Balance> for Pallet<T>
	where
		<T as Config>::Balance: Into<u128>,
		<T as Config>::Balance: From<u128>,
	{
		fn on_argon_burn(microgons: &T::Balance) {
			Self::on_argon_burn(*microgons);
		}
	}

	#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
	pub enum MintType {
		Bitcoin,
		Mining,
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Default, Decode, TypeInfo)]
pub struct MintAction<Balance: Codec> {
	pub argon_burned: Balance,
	pub argon_minted: Balance,
	pub bitcoin_minted: Balance,
}
impl<T: Config> BlockRewardsEventHandler<T::AccountId, T::Balance> for Pallet<T>
where
	<T as Config>::Balance: Into<u128>,
	<T as Config>::Balance: From<u128>,
{
	fn rewards_created(payout: &[BlockPayout<T::AccountId, T::Balance>]) {
		let mut argons = T::Balance::zero();
		for reward in payout {
			argons = argons.saturating_add(reward.argons);
		}
		if argons != T::Balance::zero() {
			Self::track_block_mint(argons);
		}
	}
}
