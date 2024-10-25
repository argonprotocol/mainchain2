use crate::{BlockSealDigest, BlockVotingPower, ComputeDifficulty};
use codec::{Decode, Encode, MaxEncodedLen};
use core::cmp::Ordering;
use scale_info::TypeInfo;
use sp_core::U256;

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct ForkPower {
	#[codec(compact)]
	pub notebooks: u64,
	pub voting_power: U256,
	pub seal_strength: U256,
	pub total_compute_difficulty: U256,
	#[codec(compact)]
	pub vote_created_blocks: u128,
}
impl ForkPower {
	pub fn add(
		&mut self,
		block_voting_power: BlockVotingPower,
		notebooks: u32,
		seal_digest: BlockSealDigest,
		compute_difficulty: ComputeDifficulty,
	) {
		match seal_digest {
			BlockSealDigest::Vote { seal_strength } => {
				self.add_vote(block_voting_power, notebooks, seal_strength);
			},
			BlockSealDigest::Compute { .. } => {
				self.add_compute(block_voting_power, notebooks, compute_difficulty);
			},
		}
	}

	pub fn add_vote(
		&mut self,
		block_voting_power: BlockVotingPower,
		notebooks: u32,
		seal_strength: U256,
	) {
		self.voting_power = self.voting_power.saturating_add(U256::from(block_voting_power));
		self.notebooks = self.notebooks.saturating_add(notebooks as u64);
		self.seal_strength = seal_strength;
		self.vote_created_blocks = self.vote_created_blocks.saturating_add(1);
	}

	pub fn add_compute(
		&mut self,
		block_voting_power: BlockVotingPower,
		notebooks: u32,
		compute_difficulty: ComputeDifficulty,
	) {
		self.voting_power = self.voting_power.saturating_add(U256::from(block_voting_power));
		self.notebooks = self.notebooks.saturating_add(notebooks as u64);
		self.total_compute_difficulty =
			self.total_compute_difficulty.saturating_add(compute_difficulty.into());
	}
}

impl Default for ForkPower {
	fn default() -> Self {
		Self {
			voting_power: U256::zero(),
			notebooks: 0,
			seal_strength: U256::MAX,
			total_compute_difficulty: U256::zero(),
			vote_created_blocks: 0,
		}
	}
}

impl PartialOrd for ForkPower {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let mut cmp = self.notebooks.cmp(&other.notebooks);
		if cmp == Ordering::Equal {
			cmp = self.voting_power.cmp(&other.voting_power);
		}
		if cmp == Ordering::Equal {
			// count forks with tax votes over compute
			cmp = self.vote_created_blocks.cmp(&other.vote_created_blocks);
		}
		if cmp == Ordering::Equal {
			// smaller vote proof is better
			cmp = other.seal_strength.cmp(&self.seal_strength)
		}
		if cmp == Ordering::Equal {
			cmp = self.total_compute_difficulty.cmp(&other.total_compute_difficulty)
		}
		Some(cmp)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn it_should_compare_fork_power() {
		assert_eq!(ForkPower::default(), ForkPower::default());

		assert!(
			ForkPower { voting_power: 1.into(), ..Default::default() } >
				ForkPower { voting_power: 0.into(), ..Default::default() }
		);

		assert!(
			ForkPower { notebooks: 1, ..Default::default() } >
				ForkPower { notebooks: 0, ..Default::default() }
		);

		assert!(
			ForkPower { seal_strength: 200.into(), ..Default::default() } >
				ForkPower { seal_strength: 201.into(), ..Default::default() }
		);

		assert!(
			ForkPower { total_compute_difficulty: 1000.into(), ..Default::default() } >
				ForkPower { total_compute_difficulty: 999.into(), ..Default::default() }
		);
	}
}