use env_logger::{Builder, Env};
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64, Currency, StorageMapShim},
};
use sp_core::{ConstU32, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

use crate as pallet_mining_slot;
use crate::Registration;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		MiningSlots: pallet_mining_slot,
		ArgonBalances: pallet_balances::<Instance1>,
		UlixeeBalances: pallet_balances::<Instance2>,
		Bonds: pallet_bond
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub static BlocksBetweenSlots: u32 = 1;
	pub static MaxCohortSize: u32 = 5;
	pub static MaxMiners: u32 = 10;
	pub static BlocksBufferToStopAcceptingBids: u32 = 0;
	pub const OwnershipPercentDamper: u32 = 80;

	pub static ExistentialDeposit: Balance = 1;
	pub const BondReserveIdentifier:[u8; 8] = *b"bondfund";
	pub const OwnershipReserveIdentifier:[u8; 8] = *b"bondownr";
	pub const MinimumBondAmount:u128 = 1_000;
}

pub type BondId = u64;
pub type Balance = u128;

type ArgonToken = pallet_balances::Instance1;
impl pallet_balances::Config<ArgonToken> for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type MaxHolds = ConstU32<100>;
}

pub fn set_ownership(account_id: u64, amount: Balance) {
	let _ = UlixeeBalances::make_free_balance_be(&account_id, amount);
	drop(UlixeeBalances::issue(amount));
}

pub fn set_argons(account_id: u64, amount: Balance) {
	let _ = ArgonBalances::make_free_balance_be(&account_id, amount);
	drop(ArgonBalances::issue(amount));
}

pub(crate) type UlixeeToken = pallet_balances::Instance2;
impl pallet_balances::Config<UlixeeToken> for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type ReserveIdentifier = [u8; 8];
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = StorageMapShim<
		pallet_balances::Account<Test, UlixeeToken>,
		Self::AccountId,
		pallet_balances::AccountData<Balance>,
	>;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type MaxHolds = ConstU32<100>;
}

impl pallet_bond::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = ArgonBalances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type BondFundId = u32;
	type BondId = BondId;
	type MinimumBondAmount = MinimumBondAmount;
	type MaxConcurrentlyExpiringBonds = ConstU32<10>;
	type MaxConcurrentlyExpiringBondFunds = ConstU32<10>;
	type BlocksPerYear = ConstU64<525_600>;
	type Balance = u128;
}

impl pallet_mining_slot::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type SessionIndicesToKeepInHistory = ConstU32<10>;
	type BlocksBetweenSlots = BlocksBetweenSlots;
	type MaxCohortSize = MaxCohortSize;
	type MaxMiners = MaxMiners;
	type OwnershipCurrency = UlixeeBalances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type OwnershipPercentDamper = OwnershipPercentDamper;
	type BlocksBufferToStopAcceptingBids = BlocksBufferToStopAcceptingBids;
	type Balance = Balance;
	type BondId = BondId;
	type BondProvider = Bonds;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext(miner_zero: Option<Registration<Test>>) -> sp_io::TestExternalities {
	let env = Env::new().default_filter_or("debug");
	let _ = Builder::from_env(env).is_test(true).try_init();

	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into();
	pallet_mining_slot::GenesisConfig::<Test> { miner_zero, _phantom: Default::default() }
		.assimilate_storage(&mut t)
		.unwrap();

	sp_io::TestExternalities::new(t)
}