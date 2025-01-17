use std::{collections::HashMap, env};

use frame_support::{
	assert_err, assert_noop, assert_ok,
	traits::{
		fungible::{InspectHold, Unbalanced},
		Currency, OnInitialize,
	},
};
use pallet_balances::Event as OwnershipEvent;
use sp_core::{blake2_256, bounded_vec, crypto::AccountId32, ByteArray, H256, U256};
use sp_runtime::{testing::UintAuthorityId, BoundedVec, FixedU128};

use crate::{
	mock::{MiningSlots, Ownership, *},
	pallet::{
		AccountIndexLookup, ActiveMinersByIndex, ActiveMinersCount, AuthorityHashByIndex,
		HistoricalBidsPerSlot, IsNextSlotBiddingOpen, NextSlotCohort, OwnershipBondAmount,
	},
	Error, Event, HoldReason, MiningSlotBid,
};
use argon_primitives::{
	block_seal::{
		MiningAuthority, MiningBidStats, MiningRegistration, RewardDestination, RewardSharing,
	},
	inherents::BlockSealInherent,
	AuthorityProvider, BlockRewardAccountsProvider, BlockVote, MerkleProof,
};

#[test]
fn it_doesnt_add_cohorts_until_time() {
	BlocksBetweenSlots::set(2);

	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		NextSlotCohort::<Test>::set(bounded_vec![MiningRegistration {
			account_id: 1,
			bond_id: None,
			ownership_tokens: 0,
			bond_amount: 0,
			reward_destination: RewardDestination::Owner,
			reward_sharing: None,
			authority_keys: 1.into()
		}]);

		MiningSlots::on_initialize(1);

		assert_eq!(NextSlotCohort::<Test>::get().len(), 1);
		assert_eq!(NextSlotCohort::<Test>::get()[0].reward_destination, RewardDestination::Owner,);
	});
}

#[test]
fn get_validation_window_blocks() {
	MaxCohortSize::set(2);
	MaxMiners::set(10);
	BlocksBetweenSlots::set(1);

	new_test_ext().execute_with(|| {
		assert_eq!(MiningSlots::get_mining_window_blocks(), 5);
	});

	MaxCohortSize::set(5);
	MaxMiners::set(10);
	BlocksBetweenSlots::set(10);

	new_test_ext().execute_with(|| {
		assert_eq!(MiningSlots::get_mining_window_blocks(), 2 * 10);
	});
}

#[test]
fn get_slot_era() {
	MaxCohortSize::set(2);
	MaxMiners::set(10);
	BlocksBetweenSlots::set(140);
	new_test_ext().execute_with(|| {
		System::set_block_number(8);

		let window = 140 * 5;

		assert_eq!(MiningSlots::get_slot_era(), (140, 140 + window));
	});

	MaxCohortSize::set(5);
	MaxMiners::set(10);
	BlocksBetweenSlots::set(10);
	new_test_ext().execute_with(|| {
		System::set_block_number(8);

		assert_eq!(MiningSlots::get_slot_era(), (10, 10 + (10 * 2)));
		// on block, should start showing next period
		System::set_block_number(10);
		assert_eq!(MiningSlots::get_slot_era(), (20, 20 + (10 * 2)));

		System::set_block_number(18);
		assert_eq!(MiningSlots::get_slot_era(), (20, 20 + (10 * 2)));

		System::set_block_number(20);
		assert_eq!(MiningSlots::get_slot_era(), (30, 30 + (10 * 2)));
	});

	MaxCohortSize::set(2);
	MaxMiners::set(6);
	BlocksBetweenSlots::set(4);

	new_test_ext().execute_with(|| {
		System::set_block_number(3);
		assert_eq!(MiningSlots::get_slot_era(), (4, 4 + (4 * 3)));
		System::set_block_number(4);
		assert_eq!(MiningSlots::get_slot_era(), (8, 8 + (4 * 3)));
		System::set_block_number(6);
		assert_eq!(MiningSlots::get_slot_era(), (8, 8 + (4 * 3)));
	})
}

#[test]
fn starting_cohort_index() {
	let max_cohort_size = 3;
	let max_validators = 12;
	let blocks_between_slots = 5;

	assert_eq!(
		MiningSlots::get_slot_starting_index(
			0,
			blocks_between_slots,
			max_validators,
			max_cohort_size
		),
		0
	);
	assert_eq!(
		MiningSlots::get_slot_starting_index(
			5,
			blocks_between_slots,
			max_validators,
			max_cohort_size
		),
		3
	);
	assert_eq!(
		MiningSlots::get_slot_starting_index(
			10,
			blocks_between_slots,
			max_validators,
			max_cohort_size
		),
		6
	);
	assert_eq!(
		MiningSlots::get_slot_starting_index(
			15,
			blocks_between_slots,
			max_validators,
			max_cohort_size
		),
		9
	);

	assert_eq!(
		MiningSlots::get_slot_starting_index(
			20,
			blocks_between_slots,
			max_validators,
			max_cohort_size
		),
		0
	);
}

#[test]
fn it_adds_new_cohorts_on_block() {
	BlocksBetweenSlots::set(2);
	MaxMiners::set(6);
	MaxCohortSize::set(2);

	new_test_ext().execute_with(|| {
		System::set_block_number(8);

		for i in 0..4u32 {
			let account_id: u64 = (i + 4).into();
			ActiveMinersByIndex::<Test>::insert(
				i,
				MiningRegistration {
					account_id,
					bond_id: None,
					ownership_tokens: 0,
					bond_amount: 0,
					reward_destination: RewardDestination::Owner,
					reward_sharing: None,
					authority_keys: account_id.into(),
				},
			);
			AccountIndexLookup::<Test>::insert(account_id, i);
			AuthorityHashByIndex::<Test>::try_mutate(|index| {
				let hash = blake2_256(&account_id.to_le_bytes());
				index.try_insert(i, U256::from(hash))
			})
			.unwrap();
		}
		ActiveMinersCount::<Test>::put(4);
		// filled indexes are [0, 1, 2, 3, _, _]

		let cohort = BoundedVec::truncate_from(vec![MiningRegistration {
			account_id: 1,
			bond_id: None,
			ownership_tokens: 0,
			bond_amount: 0,
			reward_destination: RewardDestination::Owner,
			reward_sharing: None,
			authority_keys: 1.into(),
		}]);

		NextSlotCohort::<Test>::set(cohort.clone());

		// on 8, we're filling indexes 2 and 3 [0, 1, -> 2, -> 3, _, _]
		MiningSlots::on_initialize(8);

		// re-fetch
		assert_eq!(
			ActiveMinersCount::<Test>::get(),
			3,
			"Should have 3 validators still after insertion"
		);
		let validators = ActiveMinersByIndex::<Test>::iter().collect::<HashMap<_, _>>();
		let authority_hashes = AuthorityHashByIndex::<Test>::get().into_inner();
		assert_eq!(
			NextSlotCohort::<Test>::get().len(),
			0,
			"Queued mining_slot for block 8 should be removed"
		);
		assert_eq!(authority_hashes.len(), 3, "Should have 3 authority hashes after insertion");
		assert_eq!(validators.len(), 3, "Should have 3 validators still after insertion");

		assert!(validators.contains_key(&2), "Should insert validator at index 2");
		assert!(!validators.contains_key(&3), "Should no longer have a validator at index 3");

		assert_eq!(
			AccountIndexLookup::<Test>::get(1),
			Some(2),
			"Should add an index lookup for account 1 at index 2"
		);
		assert!(
			!AccountIndexLookup::<Test>::contains_key(6),
			"Should no longer have account 6 registered"
		);
		assert!(
			!AccountIndexLookup::<Test>::contains_key(7),
			"Should no longer have account 7 registered"
		);
		assert!(!authority_hashes.contains_key(&3), "Should no longer have hash index 3");

		// check what was called from the events
		assert_eq!(
			LastSlotAdded::get().len(),
			1,
			"Should emit a new slot event for the new cohort"
		);
		assert_eq!(LastSlotRemoved::get().len(), 2);

		System::assert_last_event(Event::NewMiners { start_index: 2, new_miners: cohort }.into())
	});
}

#[test]
fn it_unbonds_accounts_when_a_window_closes() {
	BlocksBetweenSlots::set(2);
	MaxMiners::set(6);
	MaxCohortSize::set(2);

	new_test_ext().execute_with(|| {
		OwnershipBondAmount::<Test>::set(1000);

		System::set_block_number(7);

		for i in 0..4u32 {
			let account_id: u64 = (i).into();
			set_ownership(account_id, 1000u32.into());
			set_argons(account_id, 10_000u32.into());

			let bond_amount = (1000u32 + i).into();
			let ownership_tokens =
				MiningSlots::hold_ownership_bond(&account_id, None).ok().unwrap();

			ActiveMinersByIndex::<Test>::insert(
				i,
				MiningRegistration {
					account_id,
					bond_id: None,
					ownership_tokens,
					bond_amount,
					reward_destination: RewardDestination::Owner,
					reward_sharing: None,
					authority_keys: 1.into(),
				},
			);
			AccountIndexLookup::<Test>::insert(account_id, i);
		}
		ActiveMinersCount::<Test>::put(4);
		IsNextSlotBiddingOpen::<Test>::set(true);
		assert_eq!(MiningSlots::get_slot_era(), (8, 8 + (2 * 3)));

		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(2),
			None,
			RewardDestination::Owner,
			1.into()
		));

		System::set_block_number(8);

		MiningSlots::on_initialize(8);

		System::assert_last_event(
			Event::NewMiners {
				start_index: 2,
				new_miners: BoundedVec::truncate_from(vec![MiningRegistration {
					bond_id: None,
					account_id: 2,
					ownership_tokens: 1000u32.into(),
					bond_amount: 0,
					reward_destination: RewardDestination::Owner,
					reward_sharing: None,
					authority_keys: 1.into(),
				}]),
			}
			.into(),
		);

		System::assert_has_event(
			OwnershipEvent::<Test, OwnershipToken>::Endowed {
				account: 3,
				free_balance: 1000u32.into(),
			}
			.into(),
		);

		System::assert_has_event(
			Event::<Test>::UnbondedMiner {
				account_id: 3,
				bond_id: None,
				kept_ownership_bond: false,
			}
			.into(),
		);
		assert_eq!(Ownership::free_balance(2), 0);
		assert_eq!(Ownership::total_balance(&2), 1000);

		assert_eq!(Ownership::free_balance(3), 1000);

		assert!(System::account_exists(&0));
		assert!(System::account_exists(&1));
		assert!(System::account_exists(&2));
		assert!(System::account_exists(&3));
	});
}

#[test]
fn it_holds_ownership_tokens_for_a_slot() {
	BlocksBetweenSlots::set(3);
	MaxMiners::set(6);
	MaxCohortSize::set(2);

	new_test_ext().execute_with(|| {
		System::set_block_number(6);

		assert_err!(
			MiningSlots::bid(RuntimeOrigin::signed(2), None, RewardDestination::Owner, 1.into()),
			Error::<Test>::SlotNotTakingBids
		);

		SlotBiddingStartTick::set(0);
		IsNextSlotBiddingOpen::<Test>::set(true);

		set_ownership(3, 5000u32.into());
		let share_amount = 5000 / 6;
		MiningSlots::on_initialize(6);
		OwnershipBondAmount::<Test>::set(share_amount);

		assert_err!(
			MiningSlots::bid(RuntimeOrigin::signed(1), None, RewardDestination::Owner, 1.into()),
			Error::<Test>::InsufficientOwnershipTokens
		);

		set_ownership(1, 1000u32.into());

		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(1),
			None,
			RewardDestination::Owner,
			1.into()
		));
		System::assert_last_event(
			Event::SlotBidderAdded { account_id: 1, bid_amount: 0u32.into(), index: 0 }.into(),
		);
		assert_eq!(Ownership::free_balance(1), 1000 - share_amount);

		// should be able to re-register
		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(1),
			None,
			RewardDestination::Owner,
			1.into()
		));
		assert_eq!(
			NextSlotCohort::<Test>::get().iter().map(|a| a.account_id).collect::<Vec<_>>(),
			vec![1]
		);
		assert_eq!(
			Ownership::free_balance(1),
			1000 - share_amount,
			"should not alter reserved balance"
		);
		assert_eq!(Ownership::total_balance(&1), 1000, "should still have their full balance");

		assert!(System::account_exists(&1));
		assert!(System::account_exists(&3));
	});
}

#[test]
fn it_wont_accept_bids_until_bidding_starts() {
	BlocksBetweenSlots::set(4);
	MaxMiners::set(6);
	MaxCohortSize::set(2);
	SlotBiddingStartTick::set(12);
	TicksSinceGenesis::set(11);

	new_test_ext().execute_with(|| {
		set_ownership(2, 100u32.into());
		for i in 1..11u64 {
			System::set_block_number(i);

			MiningSlots::on_initialize(i);
			assert_err!(
				MiningSlots::bid(
					RuntimeOrigin::signed(2),
					None,
					RewardDestination::Owner,
					1.into()
				),
				Error::<Test>::SlotNotTakingBids
			);
		}

		System::set_block_number(12);
		TicksSinceGenesis::set(12);
		MiningSlots::on_initialize(12);

		assert!(IsNextSlotBiddingOpen::<Test>::get(), "bidding should now be open");
		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(2),
			None,
			RewardDestination::Owner,
			1.into()
		));
	});
}
#[test]
fn it_wont_let_you_reuse_ownership_tokens_for_two_bids() {
	BlocksBetweenSlots::set(4);
	MaxMiners::set(6);
	MaxCohortSize::set(2);
	SlotBiddingStartTick::set(0);

	new_test_ext().execute_with(|| {
		System::set_block_number(12);

		IsNextSlotBiddingOpen::<Test>::set(true);

		assert_eq!(MiningSlots::get_slot_era(), (16, 16 + (4 * 3)));
		assert_eq!(MiningSlots::get_next_slot_starting_index(), 0);
		set_ownership(2, 100u32.into());
		set_ownership(1, 100u32.into());
		MiningSlots::on_initialize(12);

		let ownership = (200 / 6) as u128;
		OwnershipBondAmount::<Test>::set(ownership);
		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(1),
			None,
			RewardDestination::Owner,
			1.into()
		));
		System::set_block_number(16);
		MiningSlots::on_initialize(16);

		System::assert_last_event(
			Event::NewMiners {
				start_index: 2,
				new_miners: BoundedVec::truncate_from(vec![MiningRegistration {
					account_id: 1,
					bond_id: None,
					ownership_tokens: ownership,
					bond_amount: 0,
					reward_destination: RewardDestination::Owner,
					reward_sharing: None,
					authority_keys: 1.into(),
				}]),
			}
			.into(),
		);
		assert_eq!(MiningSlots::get_slot_era(), (20, 20 + (4 * 3)));
		assert_eq!(MiningSlots::get_next_slot_starting_index(), 2);

		System::set_block_number(20);
		MiningSlots::on_initialize(20);
		assert_eq!(MiningSlots::get_slot_era(), (24, 24 + (4 * 3)));
		assert_eq!(MiningSlots::get_next_slot_starting_index(), 4);

		assert_err!(
			MiningSlots::bid(RuntimeOrigin::signed(1), None, RewardDestination::Owner, 1.into()),
			Error::<Test>::CannotRegisterOverlappingSessions
		);
		assert!(System::account_exists(&1));
		assert!(System::account_exists(&2));
	})
}

#[test]
fn it_will_order_bids_with_argon_bonds() {
	BlocksBetweenSlots::set(3);
	MaxMiners::set(6);
	MaxCohortSize::set(2);

	new_test_ext().execute_with(|| {
		System::set_block_number(6);

		assert_err!(
			MiningSlots::bid(RuntimeOrigin::signed(2), None, RewardDestination::Owner, 1.into()),
			Error::<Test>::SlotNotTakingBids
		);

		SlotBiddingStartTick::set(0);
		IsNextSlotBiddingOpen::<Test>::set(true);

		set_ownership(1, 1000u32.into());
		set_ownership(2, 1000u32.into());
		set_ownership(3, 1000u32.into());

		MiningSlots::on_initialize(6);
		let share_amount = 3000 / 6;
		OwnershipBondAmount::<Test>::set(share_amount);

		// 1. Account 1 bids
		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(1),
			Some(MiningSlotBid { vault_id: 1, amount: 1000u32.into() }),
			RewardDestination::Owner,
			1.into()
		));
		System::assert_last_event(
			Event::SlotBidderAdded { account_id: 1, bid_amount: 1000u32.into(), index: 0 }.into(),
		);
		assert_eq!(Ownership::free_balance(1), 1000 - share_amount);
		let first_bid = HistoricalBidsPerSlot::<Test>::get().into_inner()[0].clone();
		assert_eq!(first_bid.bids_count, 1);
		assert_eq!(first_bid.bid_amount_min, 1000);
		assert_eq!(first_bid.bid_amount_max, 1000);
		assert_eq!(first_bid.bid_amount_sum, 1000);

		assert_eq!(Bonds::get().len(), 1);

		// 2. Account 2 bids highest and takes top slot

		// should be able to re-register
		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(2),
			Some(MiningSlotBid { vault_id: 1, amount: 1001u32.into() }),
			RewardDestination::Owner,
			1.into()
		));
		System::assert_last_event(
			Event::SlotBidderAdded { account_id: 2, bid_amount: 1001u32.into(), index: 0 }.into(),
		);

		let first_bid = HistoricalBidsPerSlot::<Test>::get().into_inner()[0].clone();
		assert_eq!(first_bid.bids_count, 2);
		assert_eq!(first_bid.bid_amount_min, 1000);
		assert_eq!(first_bid.bid_amount_max, 1001);
		assert_eq!(first_bid.bid_amount_sum, 2001);
		assert_eq!(
			NextSlotCohort::<Test>::get().iter().map(|a| a.account_id).collect::<Vec<_>>(),
			vec![2, 1]
		);

		// 3. Account 2 bids above 1

		// should be able to re-register
		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(3),
			Some(MiningSlotBid { vault_id: 1, amount: 1001u32.into() }),
			RewardDestination::Owner,
			1.into()
		));
		System::assert_last_event(
			Event::SlotBidderAdded { account_id: 3, bid_amount: 1001u32.into(), index: 1 }.into(),
		);
		assert_eq!(HistoricalBidsPerSlot::<Test>::get().into_inner()[0].clone().bids_count, 3);

		assert_eq!(
			NextSlotCohort::<Test>::get().iter().map(|a| a.account_id).collect::<Vec<_>>(),
			vec![2, 3]
		);

		let bonds = Bonds::get();
		let acc_3_bond_id = bonds.iter().find(|(_, _, a, _)| *a == 3).map(|a| a.0);
		assert!(!bonds.iter().any(|(_, _, a, _)| *a == 1));
		assert_eq!(bonds.len(), 2);
		{
			let events = frame_system::Pallet::<Test>::events();
			let frame_system::EventRecord { event, .. } = &events[events.len() - 2];
			assert_eq!(
				event,
				&<Test as frame_system::Config>::RuntimeEvent::from(
					Event::<Test>::SlotBidderReplaced {
						bond_id: Some(1u64),
						account_id: 1u64,
						kept_ownership_bond: false,
					}
				)
			);
		}
		assert_eq!(Ownership::free_balance(1), 1000);
		assert!(Ownership::hold_available(&HoldReason::RegisterAsMiner.into(), &1));

		// 4. Account 1 increases bid and resubmits
		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(1),
			Some(MiningSlotBid { vault_id: 1, amount: 1002u32.into() }),
			RewardDestination::Owner,
			1.into()
		));
		let first_bid = HistoricalBidsPerSlot::<Test>::get().into_inner()[0].clone();
		assert_eq!(first_bid.bids_count, 4);
		assert_eq!(first_bid.bid_amount_min, 1000);
		assert_eq!(first_bid.bid_amount_max, 1002);
		assert_eq!(first_bid.bid_amount_sum, 4004);

		let events = frame_system::Pallet::<Test>::events();
		// compare to the last event record
		let frame_system::EventRecord { event, .. } = &events[events.len() - 2];
		assert_eq!(
			event,
			&<Test as frame_system::Config>::RuntimeEvent::from(
				Event::<Test>::SlotBidderReplaced {
					bond_id: acc_3_bond_id,
					account_id: 3u64,
					kept_ownership_bond: false
				}
			)
		);

		System::assert_last_event(
			Event::SlotBidderAdded { account_id: 1, bid_amount: 1002u32.into(), index: 0 }.into(),
		);
		assert_eq!(Ownership::free_balance(3), 1000);

		assert_eq!(
			NextSlotCohort::<Test>::get().iter().map(|a| a.account_id).collect::<Vec<_>>(),
			vec![1, 2]
		);
		assert!(System::account_exists(&1));
		assert!(System::account_exists(&2));
		assert!(System::account_exists(&3));
	});
}

#[test]
fn handles_a_max_of_bids_per_block() {
	BlocksBetweenSlots::set(1);
	MaxMiners::set(4);
	MaxCohortSize::set(2);

	new_test_ext().execute_with(|| {
		System::set_block_number(4);
		MiningSlots::on_initialize(4);
		IsNextSlotBiddingOpen::<Test>::set(true);

		for i in 1..5 {
			set_ownership(i, 1000u32.into());
		}

		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(1),
			None,
			RewardDestination::Owner,
			1.into()
		));
		assert_eq!(HistoricalBidsPerSlot::<Test>::get().into_inner()[0].clone().bids_count, 1);

		System::assert_last_event(
			Event::SlotBidderAdded { account_id: 1, bid_amount: 0u32.into(), index: 0 }.into(),
		);
		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(2),
			None,
			RewardDestination::Owner,
			1.into()
		));
		System::assert_last_event(
			Event::SlotBidderAdded { account_id: 2, bid_amount: 0u32.into(), index: 1 }.into(),
		);
		assert_noop!(
			MiningSlots::bid(RuntimeOrigin::signed(3), None, RewardDestination::Owner, 1.into()),
			Error::<Test>::BidTooLow,
		);
		// should not have changed
		System::assert_last_event(
			Event::SlotBidderAdded { account_id: 2, bid_amount: 0u32.into(), index: 1 }.into(),
		);
		assert_eq!(HistoricalBidsPerSlot::<Test>::get().into_inner()[0].clone().bids_count, 2);
		for i in 1..5 {
			assert!(System::account_exists(&i));
		}
	});
}

#[test]
fn records_profit_sharing_if_applicable() {
	BlocksBetweenSlots::set(1);
	MaxMiners::set(4);
	MaxCohortSize::set(2);
	VaultSharing::set(Some(RewardSharing {
		account_id: 30,
		percent_take: FixedU128::from_rational(90, 100),
	}));

	new_test_ext().execute_with(|| {
		System::set_block_number(4);
		MiningSlots::on_initialize(4);
		IsNextSlotBiddingOpen::<Test>::set(true);

		set_ownership(1, 1000u32.into());

		assert_ok!(MiningSlots::bid(
			RuntimeOrigin::signed(1),
			Some(MiningSlotBid { vault_id: 1, amount: 1000u32.into() }),
			RewardDestination::Account(25),
			1.into()
		));
		assert_eq!(HistoricalBidsPerSlot::<Test>::get().into_inner()[0].bids_count, 1);

		System::assert_last_event(
			Event::SlotBidderAdded { account_id: 1, bid_amount: 1000u32.into(), index: 0 }.into(),
		);
		assert_eq!(
			NextSlotCohort::<Test>::get()[0].reward_sharing,
			Some(RewardSharing { account_id: 30, percent_take: FixedU128::from_rational(90, 100) })
		);

		MiningSlots::on_initialize(6);
		assert_eq!(HistoricalBidsPerSlot::<Test>::get().into_inner()[0].bids_count, 0);
		assert_eq!(NextSlotCohort::<Test>::get().len(), 0);
		let rewards_accounts = MiningSlots::get_all_rewards_accounts();
		assert_eq!(rewards_accounts.len(), 2);
		assert_eq!(rewards_accounts[0], (25, Some(FixedU128::from_rational(10, 100))));
		assert_eq!(rewards_accounts[1], (30, Some(FixedU128::from_rational(90, 100))));

		assert_eq!(
			MiningSlots::get_rewards_account(&1),
			(
				Some(25),
				Some(RewardSharing {
					account_id: 30,
					percent_take: FixedU128::from_rational(90, 100)
				})
			)
		);
	});
}

#[test]
fn it_handles_null_authority() {
	new_test_ext().execute_with(|| {
		assert_eq!(MiningSlots::get_authority(1), None);
	});
}

#[test]
fn it_can_get_closest_authority() {
	MaxMiners::set(100);
	new_test_ext().execute_with(|| {
		System::set_block_number(8);

		for i in 0..100u32 {
			let account_id: u64 = i.into();
			ActiveMinersByIndex::<Test>::insert(
				i,
				MiningRegistration {
					account_id,
					bond_id: None,
					ownership_tokens: 0,
					bond_amount: 0,
					reward_destination: RewardDestination::Owner,
					reward_sharing: None,
					authority_keys: account_id.into(),
				},
			);
			AccountIndexLookup::<Test>::insert(account_id, i);
		}
		AuthorityHashByIndex::<Test>::try_mutate(|a| {
			for i in 0..100u32 {
				// these are normally hashed, but we'll simplify to ease the xor calculation
				let hash = U256::from(i);
				let _ = a.try_insert(i, hash);
			}
			Ok::<(), Error<Test>>(())
		})
		.expect("Didn't insert authorities");

		assert_eq!(
			MiningSlots::xor_closest_authority(U256::from(100)),
			Some(MiningAuthority {
				account_id: 96,
				authority_id: UintAuthorityId(96),
				authority_index: 96,
			})
		);
	});
}

#[test]
fn it_will_end_auctions_if_a_seal_qualifies() {
	BlocksBetweenSlots::set(100);
	MaxMiners::set(6);
	MaxCohortSize::set(2);
	BlocksBeforeBidEndForVrfClose::set(10);
	SlotBiddingStartTick::set(0);

	new_test_ext().execute_with(|| {
		System::set_block_number(89);

		IsNextSlotBiddingOpen::<Test>::set(true);

		let seal = BlockSealInherent::Compute;
		// it's too soon
		assert!(!MiningSlots::check_for_bidding_close(&seal));

		// This seal strength was generated using the commented out loop below
		let seal_strength = U256::from_dec_str(
			"55660301883345363905660969606306034026269601808931936101802154266730817045052",
		)
		.expect("can read seal strength u256");

		let seal = create_block_vote_seal(seal_strength);
		assert!(!MiningSlots::check_for_bidding_close(&seal));

		// now we're the right block
		System::set_block_number(90);
		assert!(MiningSlots::check_for_bidding_close(&seal));
		assert!(!MiningSlots::check_for_bidding_close(&BlockSealInherent::Compute));

		let invalid_strength = U256::from(1);
		let seal = create_block_vote_seal(invalid_strength);
		assert!(!MiningSlots::check_for_bidding_close(&seal));

		if env::var("TEST_DISTRO").unwrap_or("false".to_string()) == "true" {
			let mut valid_seals = vec![];
			for _ in 0..10u32 {
				let seal_strength = U256::from_big_endian(H256::random().as_ref());
				let seal = create_block_vote_seal(seal_strength);

				if MiningSlots::check_for_bidding_close(&seal) {
					valid_seals.push(seal_strength);
				}
			}
			assert!(!valid_seals.is_empty(), "Should have found at least one valid seal");
			println!("Valid seals: {:?}", valid_seals);
		}
	})
}

fn bid_stats(count: u32, amount: Balance) -> MiningBidStats {
	MiningBidStats {
		bids_count: count,
		bid_amount_sum: amount * count as u128,
		bid_amount_min: amount,
		bid_amount_max: amount,
	}
}

#[test]
fn it_adjusts_ownership_bonds() {
	BlocksBetweenSlots::set(10);
	MaxMiners::set(100);
	MaxCohortSize::set(10);
	SlotBiddingStartTick::set(10);
	TargetBidsPerSlot::set(12);
	MinOwnershipBondAmount::set(100_000);

	new_test_ext().execute_with(|| {
		System::set_block_number(10);

		Ownership::set_total_issuance(500_000 * 100);
		OwnershipBondAmount::<Test>::set(120_000);
		// should have 10 per slot, make it 12
		HistoricalBidsPerSlot::<Test>::set(bounded_vec![bid_stats(12, 10), bid_stats(12, 10)]);
		MiningSlots::adjust_ownership_bond_amount();

		// we're targeting 12 bids per slot, so should stay the same
		assert_eq!(OwnershipBondAmount::<Test>::get(), 120_000);

		// simulate bids being past 20%
		HistoricalBidsPerSlot::<Test>::set(bounded_vec![
			bid_stats(20, 100),
			bid_stats(20, 101),
			bid_stats(20, 102)
		]);
		MiningSlots::adjust_ownership_bond_amount();
		// 120k * 1.2
		assert_eq!(OwnershipBondAmount::<Test>::get(), 144000);

		// simulate bids being way past 20%
		// should have 10 per slot, make it 12
		HistoricalBidsPerSlot::<Test>::set(bounded_vec![
			bid_stats(0, 0),
			bid_stats(1, 100),
			bid_stats(0, 0)
		]);
		MiningSlots::adjust_ownership_bond_amount();
		assert_eq!(OwnershipBondAmount::<Test>::get(), (144000.0 * 0.8f64) as u128);

		// simulate bids being way past 20%
		let mut last = OwnershipBondAmount::<Test>::get();
		for _ in 0..100 {
			HistoricalBidsPerSlot::<Test>::set(bounded_vec![
				bid_stats(100, 0),
				bid_stats(100, 100),
				bid_stats(100, 0)
			]);
			MiningSlots::adjust_ownership_bond_amount();
			let next = OwnershipBondAmount::<Test>::get();
			if next == 400_000 {
				break;
			}
			assert_eq!(next, (last as f64 * 1.2) as u128);
			last = next;
		}

		// max increase is to a set amount of the total issuance
		assert_eq!(OwnershipBondAmount::<Test>::get(), (500_000.0 * 0.8) as u128);
	});
}

fn create_block_vote_seal(seal_strength: U256) -> BlockSealInherent {
	BlockSealInherent::Vote {
		seal_strength,
		notary_id: 1,
		block_vote: BlockVote {
			block_hash: System::block_hash(System::block_number().saturating_sub(4)),
			account_id: AccountId32::from_slice(&[2u8; 32]).expect("32 bytes"),
			index: 1,
			power: 500,
			tick: 1,
			block_rewards_account_id: AccountId32::from_slice(&[3u8; 32]).expect("32 bytes"),
			signature: sp_core::sr25519::Signature::from_raw([0u8; 64]).into(),
		},
		source_notebook_number: 1,
		source_notebook_proof: MerkleProof::default(),
	}
}
