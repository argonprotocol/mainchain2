// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/storage';

import type { ApiTypes, AugmentedQuery, QueryableStorageEntry } from '@polkadot/api-base/types';
import type { BTreeMap, Bytes, Null, Option, U256, U8aFixed, Vec, bool, u128, u16, u32, u64 } from '@polkadot/types-codec';
import type { AnyNumber, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';
import type { FrameSupportDispatchPerDispatchClassWeight, FrameSystemAccountInfo, FrameSystemEventRecord, FrameSystemLastRuntimeUpgradeInfo, FrameSystemPhase, PalletBalancesAccountData, PalletBalancesBalanceLock, PalletBalancesIdAmountRuntimeFreezeReason, PalletBalancesIdAmountRuntimeHoldReason, PalletBalancesReserveData, PalletBlockRewardsBlockPayout, PalletChainTransferQueuedTransferOut, PalletDataDomainDataDomainRegistration, PalletGrandpaStoredPendingChange, PalletGrandpaStoredState, PalletTransactionPaymentReleases, SpConsensusGrandpaAppPublic, SpCoreCryptoKeyTypeId, SpCoreEd25519Public, SpRuntimeDigest, SpStakingOffenceOffenceDetails, UlxNodeRuntimeOpaqueSessionKeys, UlxNotaryAuditErrorVerifyError, UlxPrimitivesBalanceChangeAccountOrigin, UlxPrimitivesBlockSealAppPublic, UlxPrimitivesBlockSealMiningRegistration, UlxPrimitivesBond, UlxPrimitivesBondBondFund, UlxPrimitivesDataDomain, UlxPrimitivesDataDomainZoneRecord, UlxPrimitivesDigestsBlockVoteDigest, UlxPrimitivesDigestsNotebookDigest, UlxPrimitivesDigestsParentVotingKeyDigest, UlxPrimitivesInherentsBlockSealInherent, UlxPrimitivesNotaryNotaryMeta, UlxPrimitivesNotaryNotaryNotebookKeyDetails, UlxPrimitivesNotaryNotaryNotebookVoteDigestDetails, UlxPrimitivesNotaryNotaryRecord, UlxPrimitivesProvidersBlockSealerInfo } from '@polkadot/types/lookup';
import type { Observable } from '@polkadot/types/types';

export type __AugmentedQuery<ApiType extends ApiTypes> = AugmentedQuery<ApiType, () => unknown>;
export type __QueryableStorageEntry<ApiType extends ApiTypes> = QueryableStorageEntry<ApiType>;

declare module '@polkadot/api-base/types/storage' {
  interface AugmentedQueries<ApiType extends ApiTypes> {
    argonBalances: {
      /**
       * The Balances pallet example of storing the balance of an account.
       * 
       * # Example
       * 
       * ```nocompile
       * impl pallet_balances::Config for Runtime {
       * type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>
       * }
       * ```
       * 
       * You can also store the balance of an account in the `System` pallet.
       * 
       * # Example
       * 
       * ```nocompile
       * impl pallet_balances::Config for Runtime {
       * type AccountStore = System
       * }
       * ```
       * 
       * But this comes with tradeoffs, storing account balances in the system pallet stores
       * `frame_system` data alongside the account data contrary to storing account balances in the
       * `Balances` pallet, which uses a `StorageMap` to store balances data only.
       * NOTE: This is only used in the case that this pallet is used to store balances.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<PalletBalancesAccountData>, [AccountId32]>;
      /**
       * Freeze locks on account balances.
       **/
      freezes: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesIdAmountRuntimeFreezeReason>>, [AccountId32]>;
      /**
       * Holds on account balances.
       **/
      holds: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesIdAmountRuntimeHoldReason>>, [AccountId32]>;
      /**
       * The total units of outstanding deactivated balance in the system.
       **/
      inactiveIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      /**
       * Any liquidity locks on some account balances.
       * NOTE: Should only be accessed when setting, changing and freeing a lock.
       **/
      locks: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesBalanceLock>>, [AccountId32]>;
      /**
       * Named reserves on some account balances.
       **/
      reserves: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesReserveData>>, [AccountId32]>;
      /**
       * The total units issued in the system.
       **/
      totalIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []>;
    };
    authorship: {
      /**
       * Author of current block.
       **/
      author: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
    };
    blockRewards: {
      payoutsByBlock: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<PalletBlockRewardsBlockPayout>>, [u32]>;
    };
    blockSeal: {
      lastBlockSealerInfo: AugmentedQuery<ApiType, () => Observable<Option<UlxPrimitivesProvidersBlockSealerInfo>>, []>;
      /**
       * The calculated parent voting key for a block. Refers to the Notebook BlockVote Revealed
       * Secret + VotesMerkleRoot of the parent block notebooks.
       **/
      parentVotingKey: AugmentedQuery<ApiType, () => Observable<Option<H256>>, []>;
      /**
       * Author of current block (temporary storage).
       **/
      tempAuthor: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
      /**
       * Ensures only a single inherent is applied
       **/
      tempSealInherent: AugmentedQuery<ApiType, () => Observable<Option<UlxPrimitivesInherentsBlockSealInherent>>, []>;
      /**
       * Temporarily track the parent voting key digest
       **/
      tempVotingKeyDigest: AugmentedQuery<ApiType, () => Observable<Option<UlxPrimitivesDigestsParentVotingKeyDigest>>, []>;
    };
    blockSealSpec: {
      /**
       * The current vote minimum of the chain. Block votes use this minimum to determine the
       * minimum amount of tax or compute needed to create a vote. It is adjusted up or down to
       * target a max number of votes
       **/
      currentComputeDifficulty: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      /**
       * The current vote minimum of the chain. Block votes use this minimum to determine the
       * minimum amount of tax or compute needed to create a vote. It is adjusted up or down to
       * target a max number of votes
       **/
      currentVoteMinimum: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      pastBlockVotes: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, u32, u128]>>>, []>;
      pastComputeBlockTimes: AugmentedQuery<ApiType, () => Observable<Vec<u64>>, []>;
      previousBlockTimestamp: AugmentedQuery<ApiType, () => Observable<Option<u64>>, []>;
      tempBlockTimestamp: AugmentedQuery<ApiType, () => Observable<Option<u64>>, []>;
      /**
       * Temporary store the vote digest
       **/
      tempBlockVoteDigest: AugmentedQuery<ApiType, () => Observable<Option<UlxPrimitivesDigestsBlockVoteDigest>>, []>;
      /**
       * Temporary store of any current tick notebooks included in this block (vs tick)
       **/
      tempCurrentTickNotebooksInBlock: AugmentedQuery<ApiType, () => Observable<Vec<UlxPrimitivesNotaryNotaryNotebookVoteDigestDetails>>, []>;
      /**
       * Keeps the last 3 vote minimums. The first one applies to the current block.
       **/
      voteMinimumHistory: AugmentedQuery<ApiType, () => Observable<Vec<u128>>, []>;
    };
    bond: {
      /**
       * Completion of each bond, upon which date funds are returned to the bond fund or self-bonder
       **/
      bondCompletions: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<u64>>, [u32]>;
      /**
       * Expiration block number for each bond fund
       **/
      bondFundExpirations: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<u32>>, [u32]>;
      /**
       * BondFunds by id
       **/
      bondFunds: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<UlxPrimitivesBondBondFund>>, [u32]>;
      /**
       * Bonds by id
       **/
      bonds: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<UlxPrimitivesBond>>, [u64]>;
      nextBondFundId: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      nextBondId: AugmentedQuery<ApiType, () => Observable<Option<u64>>, []>;
    };
    chainTransfer: {
      expiringTransfersOut: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<ITuple<[AccountId32, u32]>>>, [u32]>;
      pendingTransfersOut: AugmentedQuery<ApiType, (arg1: AccountId32 | string | Uint8Array, arg2: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletChainTransferQueuedTransferOut>>, [AccountId32, u32]>;
      transfersUsedInBlockNotebooks: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<ITuple<[AccountId32, u32]>>>, [u32]>;
    };
    dataDomain: {
      domainPaymentAddressHistory: AugmentedQuery<ApiType, (arg: UlxPrimitivesDataDomain | { domainName?: any; topLevelDomain?: any } | string | Uint8Array) => Observable<Vec<ITuple<[AccountId32, u32]>>>, [UlxPrimitivesDataDomain]>;
      expiringDomainsByBlock: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<UlxPrimitivesDataDomain>>, [u32]>;
      registeredDataDomains: AugmentedQuery<ApiType, (arg: UlxPrimitivesDataDomain | { domainName?: any; topLevelDomain?: any } | string | Uint8Array) => Observable<Option<PalletDataDomainDataDomainRegistration>>, [UlxPrimitivesDataDomain]>;
      zoneRecordsByDomain: AugmentedQuery<ApiType, (arg: UlxPrimitivesDataDomain | { domainName?: any; topLevelDomain?: any } | string | Uint8Array) => Observable<Option<UlxPrimitivesDataDomainZoneRecord>>, [UlxPrimitivesDataDomain]>;
    };
    grandpa: {
      /**
       * The current list of authorities.
       **/
      authorities: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>>, []>;
      /**
       * The number of changes (both in terms of keys and underlying economic responsibilities)
       * in the "set" of Grandpa validators from genesis.
       **/
      currentSetId: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * next block number where we can force a change.
       **/
      nextForced: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Pending change: (signaled at, scheduled change).
       **/
      pendingChange: AugmentedQuery<ApiType, () => Observable<Option<PalletGrandpaStoredPendingChange>>, []>;
      /**
       * A mapping from grandpa set ID to the index of the *most recent* session for which its
       * members were responsible.
       * 
       * This is only used for validating equivocation proofs. An equivocation proof must
       * contains a key-ownership proof for a given session, therefore we need a way to tie
       * together sessions and GRANDPA set ids, i.e. we need to validate that a validator
       * was the owner of a given key on a given session, and what the active set ID was
       * during that session.
       * 
       * TWOX-NOTE: `SetId` is not under user control.
       **/
      setIdSession: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<u32>>, [u64]>;
      /**
       * `true` if we are currently stalled.
       **/
      stalled: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[u32, u32]>>>, []>;
      /**
       * State of the current authority set.
       **/
      state: AugmentedQuery<ApiType, () => Observable<PalletGrandpaStoredState>, []>;
    };
    historical: {
      /**
       * Mapping from historical session indices to session-data root hash and validator count.
       **/
      historicalSessions: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<ITuple<[H256, u32]>>>, [u32]>;
      /**
       * The range of historical sessions we store. [first, last)
       **/
      storedRange: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[u32, u32]>>>, []>;
    };
    miningSlot: {
      /**
       * Lookup by account id to the corresponding index in ActiveMinersByIndex and Authorities
       **/
      accountIndexLookup: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<u32>>, [AccountId32]>;
      /**
       * Miners that are active in the current block (post initialize)
       **/
      activeMinersByIndex: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<UlxPrimitivesBlockSealMiningRegistration>>, [u32]>;
      activeMinersCount: AugmentedQuery<ApiType, () => Observable<u16>, []>;
      /**
       * Authorities are the session keys that are actively participating in the network.
       * The tuple is the authority, and the blake2 256 hash of the authority used for xor lookups
       **/
      authoritiesByIndex: AugmentedQuery<ApiType, () => Observable<BTreeMap<u32, ITuple<[UlxPrimitivesBlockSealAppPublic, U256]>>>, []>;
      /**
       * Is the next slot still open for bids
       **/
      isNextSlotBiddingOpen: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * The configuration for a miner to supply if there are no registered miners
       **/
      minerZero: AugmentedQuery<ApiType, () => Observable<Option<UlxPrimitivesBlockSealMiningRegistration>>, []>;
      /**
       * The cohort set to go into effect in the next slot. The Vec has all
       * registrants with their bid amount
       **/
      nextSlotCohort: AugmentedQuery<ApiType, () => Observable<Vec<UlxPrimitivesBlockSealMiningRegistration>>, []>;
      /**
       * Tokens that must be bonded to take a Miner role
       **/
      ownershipBondAmount: AugmentedQuery<ApiType, () => Observable<u128>, []>;
    };
    mint: {
      /**
       * Last moved block of ulixee tokens
       **/
      ulixeeAccountLastTransferBlock: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<u32>>, [AccountId32]>;
    };
    notaries: {
      activeNotaries: AugmentedQuery<ApiType, () => Observable<Vec<UlxPrimitivesNotaryNotaryRecord>>, []>;
      expiringProposals: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<AccountId32>>, [u32]>;
      nextNotaryId: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      notaryKeyHistory: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<ITuple<[u32, SpCoreEd25519Public]>>>, [u32]>;
      proposedNotaries: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<ITuple<[UlxPrimitivesNotaryNotaryMeta, u32]>>>, [AccountId32]>;
      queuedNotaryMetaChanges: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<BTreeMap<u32, UlxPrimitivesNotaryNotaryMeta>>, [u32]>;
    };
    notebook: {
      /**
       * Storage map of account origin (notary_id, notebook, account_uid) to the last
       * notebook containing this account in the changed accounts merkle root
       * (NotebookChangedAccountsRootByNotary)
       **/
      accountOriginLastChangedNotebookByNotary: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: UlxPrimitivesBalanceChangeAccountOrigin | { notebookNumber?: any; accountUid?: any } | string | Uint8Array) => Observable<Option<u32>>, [u32, UlxPrimitivesBalanceChangeAccountOrigin]>;
      /**
       * The notebooks included in this block
       **/
      blockNotebooks: AugmentedQuery<ApiType, () => Observable<UlxPrimitivesDigestsNotebookDigest>, []>;
      /**
       * List of last few notebook details by notary. The bool is whether the notebook is eligible
       * for votes (received at correct tick and audit passed)
       **/
      lastNotebookDetailsByNotary: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<ITuple<[UlxPrimitivesNotaryNotaryNotebookKeyDetails, bool]>>>, [u32]>;
      /**
       * Notaries locked for failing audits
       * TODO: we need a mechanism to unlock a notary with "Fixes"
       **/
      notariesLockedForFailedAudit: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<ITuple<[u32, u32, UlxNotaryAuditErrorVerifyError]>>>, [u32]>;
      /**
       * Double storage map of notary id + notebook # to the change root
       **/
      notebookChangedAccountsRootByNotary: AugmentedQuery<ApiType, (arg1: u32 | AnyNumber | Uint8Array, arg2: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>, [u32, u32]>;
      /**
       * Temporary store a copy of the notebook digest in storage
       **/
      tempNotebookDigest: AugmentedQuery<ApiType, () => Observable<Option<UlxPrimitivesDigestsNotebookDigest>>, []>;
    };
    offences: {
      /**
       * A vector of reports of the same kind that happened at the same time slot.
       **/
      concurrentReportsIndex: AugmentedQuery<ApiType, (arg1: U8aFixed | string | Uint8Array, arg2: Bytes | string | Uint8Array) => Observable<Vec<H256>>, [U8aFixed, Bytes]>;
      /**
       * The primary structure that holds all offence records keyed by report identifiers.
       **/
      reports: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<SpStakingOffenceOffenceDetails>>, [H256]>;
    };
    session: {
      /**
       * Current index of the session.
       **/
      currentIndex: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Indices of disabled validators.
       * 
       * The vec is always kept sorted so that we can find whether a given validator is
       * disabled using binary search. It gets cleared when `on_session_ending` returns
       * a new set of identities.
       **/
      disabledValidators: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []>;
      /**
       * The owner of a key. The key is the `KeyTypeId` + the encoded key.
       **/
      keyOwner: AugmentedQuery<ApiType, (arg: ITuple<[SpCoreCryptoKeyTypeId, Bytes]> | [SpCoreCryptoKeyTypeId | string | Uint8Array, Bytes | string | Uint8Array]) => Observable<Option<AccountId32>>, [ITuple<[SpCoreCryptoKeyTypeId, Bytes]>]>;
      /**
       * The next session keys for a validator.
       **/
      nextKeys: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Option<UlxNodeRuntimeOpaqueSessionKeys>>, [AccountId32]>;
      /**
       * True if the underlying economic identities or weighting behind the validators
       * has changed in the queued validator set.
       **/
      queuedChanged: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * The queued keys for the next session. When the next session begins, these keys
       * will be used to determine the validator's session keys.
       **/
      queuedKeys: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[AccountId32, UlxNodeRuntimeOpaqueSessionKeys]>>>, []>;
      /**
       * The current set of validators.
       **/
      validators: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []>;
    };
    sudo: {
      /**
       * The `AccountId` of the sudo key.
       **/
      key: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []>;
    };
    system: {
      /**
       * The full account information for a particular account ID.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<FrameSystemAccountInfo>, [AccountId32]>;
      /**
       * Total length (in bytes) for all extrinsics put together, for the current block.
       **/
      allExtrinsicsLen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Map of block numbers to block hashes.
       **/
      blockHash: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<H256>, [u32]>;
      /**
       * The current weight for the block.
       **/
      blockWeight: AugmentedQuery<ApiType, () => Observable<FrameSupportDispatchPerDispatchClassWeight>, []>;
      /**
       * Digest of the current block, also part of the block header.
       **/
      digest: AugmentedQuery<ApiType, () => Observable<SpRuntimeDigest>, []>;
      /**
       * The number of events in the `Events<T>` list.
       **/
      eventCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Events deposited for the current block.
       * 
       * NOTE: The item is unbound and should therefore never be read on chain.
       * It could otherwise inflate the PoV size of a block.
       * 
       * Events have a large in-memory size. Box the events to not go out-of-memory
       * just in case someone still reads them from within the runtime.
       **/
      events: AugmentedQuery<ApiType, () => Observable<Vec<FrameSystemEventRecord>>, []>;
      /**
       * Mapping between a topic (represented by T::Hash) and a vector of indexes
       * of events in the `<Events<T>>` list.
       * 
       * All topic vectors have deterministic storage locations depending on the topic. This
       * allows light-clients to leverage the changes trie storage tracking mechanism and
       * in case of changes fetch the list of events of interest.
       * 
       * The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just
       * the `EventIndex` then in case if the topic has the same contents on the next block
       * no notification will be triggered thus the event might be lost.
       **/
      eventTopics: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Vec<ITuple<[u32, u32]>>>, [H256]>;
      /**
       * The execution phase of the block.
       **/
      executionPhase: AugmentedQuery<ApiType, () => Observable<Option<FrameSystemPhase>>, []>;
      /**
       * Total extrinsics count for the current block.
       **/
      extrinsicCount: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Extrinsics data for the current block (maps an extrinsic's index to its data).
       **/
      extrinsicData: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32]>;
      /**
       * Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened.
       **/
      lastRuntimeUpgrade: AugmentedQuery<ApiType, () => Observable<Option<FrameSystemLastRuntimeUpgradeInfo>>, []>;
      /**
       * The current block number being processed. Set by `execute_block`.
       **/
      number: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Hash of the previous block.
       **/
      parentHash: AugmentedQuery<ApiType, () => Observable<H256>, []>;
      /**
       * True if we have upgraded so that AccountInfo contains three types of `RefCount`. False
       * (default) if not.
       **/
      upgradedToTripleRefCount: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * True if we have upgraded so that `type RefCount` is `u32`. False (default) if not.
       **/
      upgradedToU32RefCount: AugmentedQuery<ApiType, () => Observable<bool>, []>;
    };
    ticks: {
      currentTick: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      genesisTickUtcTimestamp: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * Blocks from the last 100 ticks. Trimmed in on_initialize.
       * NOTE: cannot include the current block hash until next block
       **/
      recentBlocksAtTicks: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<H256>>, [u32]>;
      tickDuration: AugmentedQuery<ApiType, () => Observable<u64>, []>;
    };
    timestamp: {
      /**
       * Whether the timestamp has been updated in this block.
       * 
       * This value is updated to `true` upon successful submission of a timestamp by a node.
       * It is then checked at the end of each block execution in the `on_finalize` hook.
       **/
      didUpdate: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * The current time for the current block.
       **/
      now: AugmentedQuery<ApiType, () => Observable<u64>, []>;
    };
    transactionPayment: {
      nextFeeMultiplier: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      storageVersion: AugmentedQuery<ApiType, () => Observable<PalletTransactionPaymentReleases>, []>;
    };
    txPause: {
      /**
       * The set of calls that are explicitly paused.
       **/
      pausedCalls: AugmentedQuery<ApiType, (arg: ITuple<[Bytes, Bytes]> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array]) => Observable<Option<Null>>, [ITuple<[Bytes, Bytes]>]>;
    };
    ulixeeBalances: {
      /**
       * The Balances pallet example of storing the balance of an account.
       * 
       * # Example
       * 
       * ```nocompile
       * impl pallet_balances::Config for Runtime {
       * type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>
       * }
       * ```
       * 
       * You can also store the balance of an account in the `System` pallet.
       * 
       * # Example
       * 
       * ```nocompile
       * impl pallet_balances::Config for Runtime {
       * type AccountStore = System
       * }
       * ```
       * 
       * But this comes with tradeoffs, storing account balances in the system pallet stores
       * `frame_system` data alongside the account data contrary to storing account balances in the
       * `Balances` pallet, which uses a `StorageMap` to store balances data only.
       * NOTE: This is only used in the case that this pallet is used to store balances.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<PalletBalancesAccountData>, [AccountId32]>;
      /**
       * Freeze locks on account balances.
       **/
      freezes: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesIdAmountRuntimeFreezeReason>>, [AccountId32]>;
      /**
       * Holds on account balances.
       **/
      holds: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesIdAmountRuntimeHoldReason>>, [AccountId32]>;
      /**
       * The total units of outstanding deactivated balance in the system.
       **/
      inactiveIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []>;
      /**
       * Any liquidity locks on some account balances.
       * NOTE: Should only be accessed when setting, changing and freeing a lock.
       **/
      locks: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesBalanceLock>>, [AccountId32]>;
      /**
       * Named reserves on some account balances.
       **/
      reserves: AugmentedQuery<ApiType, (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesReserveData>>, [AccountId32]>;
      /**
       * The total units issued in the system.
       **/
      totalIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []>;
    };
  } // AugmentedQueries
} // declare module