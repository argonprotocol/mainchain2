use codec::Encode;
use frame_support::{
	assert_ok,
	traits::{Len, OnFinalize, OnInitialize, OnTimestampSet},
};
use sp_core::{bounded_btree_map, bounded_vec, ed25519::Public, H256};
use sp_runtime::{
	Digest,
	DigestItem::{Consensus, PreRuntime},
};

use ulx_primitives::{
	digests::{
		BlockSealMinimumsDigest, BlockVoteDigest, NotaryNotebookDigest, SealSource,
		BLOCK_VOTES_DIGEST_ID, NEXT_SEAL_MINIMUMS_DIGEST_ID,
	},
	notebook::{BlockVotingKey, NotebookHeader, NotebookNumber},
	BlockSealAuthorityId, NotaryId,
};

use crate::{
	mock::{SealMinimums, System, *},
	pallet::{PastBlockVotes, PastComputeBlockTimes, PreviousBlockTimestamp},
	Event,
};

#[test]
fn it_will_adjust_minimum() {
	new_test_ext(1000, 100).execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_ok!(PastBlockVotes::<Test>::try_mutate(|a| {
			a.try_append(&mut vec![
				(100, 0u128),
				(100, 0u128),
				(100, 0u128),
				(100, 0u128),
				(100, 0u128),
				(100, 0u128),
				(100, 0u128),
				(100, 0u128),
				(100, 0u128),
				(1, 0u128),
			])
		}));
		System::set_block_number(2);

		let start_vote_minimum = SealMinimums::vote_minimum();

		SealMinimums::update_vote_minimum(2, 0);

		assert_eq!(SealMinimums::vote_minimum(), 901);
		assert_eq!(PastBlockVotes::<Test>::get(), vec![(2, 0)]);
		System::assert_last_event(
			Event::VoteMinimumAdjusted {
				start_vote_minimum,
				actual_block_votes: 901,
				expected_block_votes: 1000,
				new_vote_minimum: 901,
			}
			.into(),
		);
	});
}

#[test]
fn it_creates_a_block_digest() {
	new_test_ext(500, 100).execute_with(|| {
		System::set_block_number(1);
		let mut book1 = create_default_notebook(1, 1);
		book1.block_votes_count = 1;
		book1.block_voting_power = 20_000;
		let mut book2 = create_default_notebook(2, 1);
		book2.block_votes_count = 3;
		book2.block_voting_power = 10_000;
		let digest = SealMinimums::create_block_vote_digest(bounded_btree_map!(
			1 => book1,
			2 => book2,
		));
		assert_eq!(
			digest,
			BlockVoteDigest {
				parent_voting_key: None,
				notebook_numbers: bounded_vec![
					NotaryNotebookDigest { notary_id: 1, notebook_number: 1 },
					NotaryNotebookDigest { notary_id: 2, notebook_number: 1 },
				],
				voting_power: 30_000,
				votes_count: 4,
			}
		);
	});
}

#[test]
fn it_creates_the_next_parent_key() {
	new_test_ext(500, 100).execute_with(|| {
		System::set_block_number(3);
		let mut book1 = create_default_notebook(1, 3);
		let book1_secret = H256::from_slice(&[1u8; 32]);
		book1.parent_secret = Some(book1_secret.clone());
		let old_root1 = H256::random();

		let mut book2 = create_default_notebook(2, 3);
		let book2_secret = H256::from_slice(&[2u8; 32]);
		book2.parent_secret = Some(book2_secret.clone());
		let old_root2 = H256::random();

		VotingRoots::mutate(|a| {
			a.insert((1, 2), (old_root1, 1));
			a.insert((2, 2), (old_root2, 1));
		});

		let digest = SealMinimums::create_block_vote_digest(bounded_btree_map!(
			1 => book1.clone(),
			2 => book2.clone(),
		));

		let parent_key = BlockVotingKey::create_key(vec![
			BlockVotingKey { parent_secret: book1_secret.clone(), parent_vote_root: old_root1 },
			BlockVotingKey { parent_secret: book2_secret.clone(), parent_vote_root: old_root2 },
		]);
		let mut expected_digest = BlockVoteDigest {
			parent_voting_key: Some(parent_key),
			notebook_numbers: bounded_vec![
				NotaryNotebookDigest { notary_id: 1, notebook_number: 3 },
				NotaryNotebookDigest { notary_id: 2, notebook_number: 3 },
			],
			voting_power: 2,
			votes_count: 2,
		};
		assert_eq!(digest, expected_digest);

		// if a parent root is not available, a key will be ignored
		VotingRoots::mutate(|a| {
			a.remove(&(1, 2));
		});
		let digest = SealMinimums::create_block_vote_digest(bounded_btree_map!(
			1 => book1,
			2 => book2,
		));
		expected_digest.parent_voting_key =
			Some(BlockVotingKey::create_key(vec![BlockVotingKey {
				parent_secret: book2_secret,
				parent_vote_root: old_root2,
			}]));
		assert_eq!(digest, expected_digest);
	});
}

#[test]
#[should_panic(expected = "Calculated seal minimums do not match included digest")]
fn it_errors_if_the_seal_minimums_digest_is_wrong() {
	MiningSlotsInitiatingTaxProof::set(1);
	new_test_ext(500, 100).execute_with(|| {
		AuthorityList::set(vec![(1, create_seal_id())]);
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		System::initialize(
			&1,
			&System::parent_hash(),
			&Digest {
				logs: vec![
					PreRuntime(
						BLOCK_VOTES_DIGEST_ID,
						BlockVoteDigest {
							parent_voting_key: None,
							voting_power: 0,
							notebook_numbers: bounded_vec![],
							votes_count: 0,
						}
						.encode(),
					),
					Consensus(
						NEXT_SEAL_MINIMUMS_DIGEST_ID,
						BlockSealMinimumsDigest { compute_difficulty: 100, vote_minimum: 1000 }
							.encode(),
					),
				],
			},
		);
		SealMinimums::on_timestamp_set(1);
		SealMinimums::on_initialize(1);
		SealMinimums::on_finalize(1);
	});
}

#[test]
fn it_calculates_next_vote_minimum() {
	// clamped
	assert_next_minimum(100, 0, 25);
	assert_next_minimum(25, 0, 6);
	assert_next_minimum(1, 0, 1);
	assert_next_minimum(100, 1, 25);
	assert_next_minimum(100, 25, 25);
	assert_next_minimum(100, 26, 26);
	assert_next_minimum(100, 50, 50);
	assert_next_minimum(100, 100, 100);
	assert_next_minimum(100, 200, 200);
	// clamped
	assert_next_minimum(100, 5_000, 400);
	assert_next_minimum(100, 10_000, 400);
}

#[test]
fn it_handles_overflowing_minimum() {
	new_test_ext(1, 0);
	let actual =
		SealMinimums::calculate_next_vote_minimum(u128::MAX - 500, 1000, 4000, 1, u128::MAX);
	assert_eq!(u128::MAX, actual, "Failed to overflow seal_minimums");
}

// assume that the current seal_minimums is 100 and the target window time is 100
fn assert_next_minimum(start_minimum: u64, actual_votes: u64, next_minimum: u64) {
	let next_minimum: u128 = next_minimum.into();
	let actual = SealMinimums::calculate_next_vote_minimum(
		start_minimum.into(),
		100,
		actual_votes.into(),
		1,
		10_000,
	);
	assert_eq!(next_minimum, actual, "Failed for actual votes {}", actual_votes);
}

fn create_seal_id() -> BlockSealAuthorityId {
	BlockSealAuthorityId::from(Public::from_raw([0u8; 32]))
}

fn create_default_notebook(notary_id: NotaryId, notebook_number: NotebookNumber) -> NotebookHeader {
	NotebookHeader {
		version: 1,
		notary_id,
		notebook_number,
		block_number: 1,
		finalized_block_number: 1,
		start_time: 0,
		changed_accounts_root: Default::default(),
		chain_transfers: Default::default(),
		changed_account_origins: Default::default(),
		tax: 0,
		end_time: 0,
		// Block Votes
		parent_secret: None,
		secret_hash: H256::from_slice(&[0u8; 32]),
		block_voting_power: 1,
		block_votes_root: H256::from_slice(&[0u8; 32]),
		block_votes_count: 1,
		best_block_nonces: Default::default(),
		blocks_with_votes: Default::default(),
	}
}

////////// DIFFICULTY TESTS //////////////////////////////////////////////////////

#[test]
fn it_doesnt_adjust_difficulty_until_time() {
	new_test_ext(100, 1000).execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let start_difficulty = SealMinimums::compute_difficulty();

		SealMinimums::on_timestamp_set(1);
		SealMinimums::on_initialize(1);
		SealMinimums::on_finalize(1);

		assert_eq!(SealMinimums::compute_difficulty(), start_difficulty);
		assert_eq!(PastComputeBlockTimes::<Test>::get().len(), 1);
	});
}

#[test]
fn it_doesnt_adjust_difficulty_if_tax_block() {
	new_test_ext(100, 1000).execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		CurrentSealType::set(SealSource::Vote);

		assert_ok!(PastComputeBlockTimes::<Test>::try_mutate(|a| {
			a.try_append(&mut vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 1])
		}));
		let start_difficulty = SealMinimums::compute_difficulty();

		SealMinimums::on_timestamp_set(1);
		SealMinimums::on_initialize(1);
		SealMinimums::on_finalize(1);

		assert_eq!(SealMinimums::compute_difficulty(), start_difficulty);
		assert_eq!(PastComputeBlockTimes::<Test>::get().len(), 10);
	});
}

#[test]
fn it_tracks_the_block_time_for_compute() {
	new_test_ext(100, 1000).execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		CurrentSealType::set(SealSource::Compute);

		let start_difficulty = SealMinimums::compute_difficulty();
		PreviousBlockTimestamp::<Test>::set(Some(500));

		SealMinimums::on_timestamp_set(1000);
		SealMinimums::on_initialize(1);
		SealMinimums::on_finalize(1);

		assert_eq!(SealMinimums::compute_difficulty(), start_difficulty);
		assert_eq!(PastComputeBlockTimes::<Test>::get().into_inner(), vec![500]);
	});
}

#[test]
fn it_will_adjust_difficulty() {
	new_test_ext(100, 10_000_000).execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_ok!(PastComputeBlockTimes::<Test>::try_mutate(|a| {
			a.try_append(&mut vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 1])
		}));
		System::set_block_number(2);

		let start_difficulty = SealMinimums::compute_difficulty();

		SealMinimums::on_timestamp_set(2);
		SealMinimums::on_initialize(1);
		SealMinimums::on_finalize(1);

		System::assert_last_event(
			Event::ComputeDifficultyAdjusted {
				start_difficulty,
				actual_block_time: 901,
				expected_block_time: 1000,
				new_difficulty: 11_098_779,
			}
			.into(),
		);
		assert_ne!(SealMinimums::compute_difficulty(), start_difficulty);
		assert_eq!(PastComputeBlockTimes::<Test>::get().len(), 1);
	});
}

#[test]
fn it_calculates_next_difficulty() {
	// clamped
	assert_next_difficulty(100, 0, 400);
	assert_next_difficulty(100, 1, 400);
	assert_next_difficulty(100, 25, 400);
	assert_next_difficulty(100, 26, 384);
	assert_next_difficulty(100, 50, 200);
	assert_next_difficulty(100, 100, 100);
	assert_next_difficulty(100, 200, 50);
	// clamped
	assert_next_difficulty(100, 5_000, 25);
	assert_next_difficulty(100, 10_000, 25);
}

#[test]
fn it_handles_overflowing_difficulty() {
	new_test_ext(0, 1);
	let actual = SealMinimums::calculate_next_difficulty(u128::MAX - 500, 1000, 0, 1, u128::MAX);
	assert_eq!(u128::MAX, actual, "Failed to overflow difficulty");
}

// assume that the current difficulty is 100 and the target window time is 100
fn assert_next_difficulty(start_difficulty: u64, time_observed: u64, next_difficulty: u64) {
	let next_difficulty: u128 = next_difficulty.into();
	let actual = SealMinimums::calculate_next_difficulty(
		start_difficulty.into(),
		100,
		time_observed,
		1,
		10000,
	);
	assert_eq!(next_difficulty, actual, "Failed for time_observed {}", time_observed);
}