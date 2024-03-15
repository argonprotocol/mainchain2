// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/types/registry';

import type { FinalityGrandpaEquivocationPrecommit, FinalityGrandpaEquivocationPrevote, FinalityGrandpaPrecommit, FinalityGrandpaPrevote, FrameSupportDispatchDispatchClass, FrameSupportDispatchDispatchInfo, FrameSupportDispatchPays, FrameSupportDispatchPerDispatchClassU32, FrameSupportDispatchPerDispatchClassWeight, FrameSupportDispatchPerDispatchClassWeightsPerClass, FrameSupportPalletId, FrameSupportTokensMiscBalanceStatus, FrameSystemAccountInfo, FrameSystemCall, FrameSystemCodeUpgradeAuthorization, FrameSystemError, FrameSystemEvent, FrameSystemEventRecord, FrameSystemExtensionsCheckGenesis, FrameSystemExtensionsCheckNonZeroSender, FrameSystemExtensionsCheckNonce, FrameSystemExtensionsCheckSpecVersion, FrameSystemExtensionsCheckTxVersion, FrameSystemExtensionsCheckWeight, FrameSystemLastRuntimeUpgradeInfo, FrameSystemLimitsBlockLength, FrameSystemLimitsBlockWeights, FrameSystemLimitsWeightsPerClass, FrameSystemPhase, PalletBalancesAccountData, PalletBalancesBalanceLock, PalletBalancesCall, PalletBalancesError, PalletBalancesEvent, PalletBalancesIdAmountRuntimeFreezeReason, PalletBalancesIdAmountRuntimeHoldReason, PalletBalancesReasons, PalletBalancesReserveData, PalletBlockRewardsBlockPayout, PalletBlockRewardsCall, PalletBlockRewardsError, PalletBlockRewardsEvent, PalletBlockRewardsFreezeReason, PalletBlockRewardsHoldReason, PalletBlockSealCall, PalletBlockSealError, PalletBlockSealSpecCall, PalletBlockSealSpecError, PalletBlockSealSpecEvent, PalletBondCall, PalletBondError, PalletBondEvent, PalletBondHoldReason, PalletChainTransferCall, PalletChainTransferError, PalletChainTransferEvent, PalletChainTransferQueuedTransferOut, PalletDataDomainCall, PalletDataDomainDataDomainRegistration, PalletDataDomainError, PalletDataDomainEvent, PalletGrandpaCall, PalletGrandpaError, PalletGrandpaEvent, PalletGrandpaStoredPendingChange, PalletGrandpaStoredState, PalletMiningSlotCall, PalletMiningSlotError, PalletMiningSlotEvent, PalletMiningSlotHoldReason, PalletMiningSlotMinerHistory, PalletMintCall, PalletMintError, PalletMintEvent, PalletMintHoldReason, PalletNotariesCall, PalletNotariesError, PalletNotariesEvent, PalletNotebookCall, PalletNotebookError, PalletNotebookEvent, PalletOffencesEvent, PalletSessionCall, PalletSessionError, PalletSessionEvent, PalletSudoCall, PalletSudoError, PalletSudoEvent, PalletTicksCall, PalletTicksError, PalletTimestampCall, PalletTransactionPaymentChargeTransactionPayment, PalletTransactionPaymentEvent, PalletTransactionPaymentReleases, PalletTxPauseCall, PalletTxPauseError, PalletTxPauseEvent, SpArithmeticArithmeticError, SpConsensusGrandpaAppPublic, SpConsensusGrandpaAppSignature, SpConsensusGrandpaEquivocation, SpConsensusGrandpaEquivocationProof, SpCoreCryptoKeyTypeId, SpCoreEcdsaSignature, SpCoreEd25519Public, SpCoreEd25519Signature, SpCoreSr25519Signature, SpRuntimeDigest, SpRuntimeDigestDigestItem, SpRuntimeDispatchError, SpRuntimeModuleError, SpRuntimeMultiSignature, SpRuntimeTokenError, SpRuntimeTransactionalError, SpSessionMembershipProof, SpStakingOffenceOffenceDetails, SpVersionRuntimeVersion, SpWeightsRuntimeDbWeight, SpWeightsWeightV2Weight, UlxNodeRuntimeOpaqueSessionKeys, UlxNodeRuntimeRuntime, UlxNodeRuntimeRuntimeFreezeReason, UlxNodeRuntimeRuntimeHoldReason, UlxNotaryAuditAccountHistoryLookupError, UlxNotaryAuditErrorVerifyError, UlxPrimitivesAccountAccountType, UlxPrimitivesBalanceChangeAccountOrigin, UlxPrimitivesBalanceChangeMerkleProof, UlxPrimitivesBlockSealAppPublic, UlxPrimitivesBlockSealAppSignature, UlxPrimitivesBlockSealMiningRegistration, UlxPrimitivesBlockSealRewardDestination, UlxPrimitivesBlockVoteBlockVoteT, UlxPrimitivesBond, UlxPrimitivesBondBondFund, UlxPrimitivesDataDomainSemver, UlxPrimitivesDataDomainVersionHost, UlxPrimitivesDataDomainZoneRecord, UlxPrimitivesDigestsBlockVoteDigest, UlxPrimitivesDigestsNotebookDigest, UlxPrimitivesDigestsNotebookDigestRecord, UlxPrimitivesDigestsParentVotingKeyDigest, UlxPrimitivesHost, UlxPrimitivesInherentsBlockSealInherent, UlxPrimitivesNotaryNotaryMeta, UlxPrimitivesNotaryNotaryNotebookKeyDetails, UlxPrimitivesNotaryNotaryNotebookVoteDigestDetails, UlxPrimitivesNotaryNotaryRecord, UlxPrimitivesNotebookChainTransfer, UlxPrimitivesNotebookNotebookHeader, UlxPrimitivesNotebookSignedNotebookHeader, UlxPrimitivesProvidersBlockSealerInfo } from '@polkadot/types/lookup';

declare module '@polkadot/types/types/registry' {
  interface InterfaceTypes {
    FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
    FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
    FinalityGrandpaPrecommit: FinalityGrandpaPrecommit;
    FinalityGrandpaPrevote: FinalityGrandpaPrevote;
    FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
    FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
    FrameSupportDispatchPays: FrameSupportDispatchPays;
    FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
    FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
    FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    FrameSupportPalletId: FrameSupportPalletId;
    FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
    FrameSystemAccountInfo: FrameSystemAccountInfo;
    FrameSystemCall: FrameSystemCall;
    FrameSystemCodeUpgradeAuthorization: FrameSystemCodeUpgradeAuthorization;
    FrameSystemError: FrameSystemError;
    FrameSystemEvent: FrameSystemEvent;
    FrameSystemEventRecord: FrameSystemEventRecord;
    FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
    FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
    FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
    FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
    FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
    FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
    FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
    FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
    FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
    FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
    FrameSystemPhase: FrameSystemPhase;
    PalletBalancesAccountData: PalletBalancesAccountData;
    PalletBalancesBalanceLock: PalletBalancesBalanceLock;
    PalletBalancesCall: PalletBalancesCall;
    PalletBalancesError: PalletBalancesError;
    PalletBalancesEvent: PalletBalancesEvent;
    PalletBalancesIdAmountRuntimeFreezeReason: PalletBalancesIdAmountRuntimeFreezeReason;
    PalletBalancesIdAmountRuntimeHoldReason: PalletBalancesIdAmountRuntimeHoldReason;
    PalletBalancesReasons: PalletBalancesReasons;
    PalletBalancesReserveData: PalletBalancesReserveData;
    PalletBlockRewardsBlockPayout: PalletBlockRewardsBlockPayout;
    PalletBlockRewardsCall: PalletBlockRewardsCall;
    PalletBlockRewardsError: PalletBlockRewardsError;
    PalletBlockRewardsEvent: PalletBlockRewardsEvent;
    PalletBlockRewardsFreezeReason: PalletBlockRewardsFreezeReason;
    PalletBlockRewardsHoldReason: PalletBlockRewardsHoldReason;
    PalletBlockSealCall: PalletBlockSealCall;
    PalletBlockSealError: PalletBlockSealError;
    PalletBlockSealSpecCall: PalletBlockSealSpecCall;
    PalletBlockSealSpecError: PalletBlockSealSpecError;
    PalletBlockSealSpecEvent: PalletBlockSealSpecEvent;
    PalletBondCall: PalletBondCall;
    PalletBondError: PalletBondError;
    PalletBondEvent: PalletBondEvent;
    PalletBondHoldReason: PalletBondHoldReason;
    PalletChainTransferCall: PalletChainTransferCall;
    PalletChainTransferError: PalletChainTransferError;
    PalletChainTransferEvent: PalletChainTransferEvent;
    PalletChainTransferQueuedTransferOut: PalletChainTransferQueuedTransferOut;
    PalletDataDomainCall: PalletDataDomainCall;
    PalletDataDomainDataDomainRegistration: PalletDataDomainDataDomainRegistration;
    PalletDataDomainError: PalletDataDomainError;
    PalletDataDomainEvent: PalletDataDomainEvent;
    PalletGrandpaCall: PalletGrandpaCall;
    PalletGrandpaError: PalletGrandpaError;
    PalletGrandpaEvent: PalletGrandpaEvent;
    PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
    PalletGrandpaStoredState: PalletGrandpaStoredState;
    PalletMiningSlotCall: PalletMiningSlotCall;
    PalletMiningSlotError: PalletMiningSlotError;
    PalletMiningSlotEvent: PalletMiningSlotEvent;
    PalletMiningSlotHoldReason: PalletMiningSlotHoldReason;
    PalletMiningSlotMinerHistory: PalletMiningSlotMinerHistory;
    PalletMintCall: PalletMintCall;
    PalletMintError: PalletMintError;
    PalletMintEvent: PalletMintEvent;
    PalletMintHoldReason: PalletMintHoldReason;
    PalletNotariesCall: PalletNotariesCall;
    PalletNotariesError: PalletNotariesError;
    PalletNotariesEvent: PalletNotariesEvent;
    PalletNotebookCall: PalletNotebookCall;
    PalletNotebookError: PalletNotebookError;
    PalletNotebookEvent: PalletNotebookEvent;
    PalletOffencesEvent: PalletOffencesEvent;
    PalletSessionCall: PalletSessionCall;
    PalletSessionError: PalletSessionError;
    PalletSessionEvent: PalletSessionEvent;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTicksCall: PalletTicksCall;
    PalletTicksError: PalletTicksError;
    PalletTimestampCall: PalletTimestampCall;
    PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
    PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
    PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
    PalletTxPauseCall: PalletTxPauseCall;
    PalletTxPauseError: PalletTxPauseError;
    PalletTxPauseEvent: PalletTxPauseEvent;
    SpArithmeticArithmeticError: SpArithmeticArithmeticError;
    SpConsensusGrandpaAppPublic: SpConsensusGrandpaAppPublic;
    SpConsensusGrandpaAppSignature: SpConsensusGrandpaAppSignature;
    SpConsensusGrandpaEquivocation: SpConsensusGrandpaEquivocation;
    SpConsensusGrandpaEquivocationProof: SpConsensusGrandpaEquivocationProof;
    SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
    SpCoreEcdsaSignature: SpCoreEcdsaSignature;
    SpCoreEd25519Public: SpCoreEd25519Public;
    SpCoreEd25519Signature: SpCoreEd25519Signature;
    SpCoreSr25519Signature: SpCoreSr25519Signature;
    SpRuntimeDigest: SpRuntimeDigest;
    SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
    SpRuntimeDispatchError: SpRuntimeDispatchError;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpRuntimeTransactionalError: SpRuntimeTransactionalError;
    SpSessionMembershipProof: SpSessionMembershipProof;
    SpStakingOffenceOffenceDetails: SpStakingOffenceOffenceDetails;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
    SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
    UlxNodeRuntimeOpaqueSessionKeys: UlxNodeRuntimeOpaqueSessionKeys;
    UlxNodeRuntimeRuntime: UlxNodeRuntimeRuntime;
    UlxNodeRuntimeRuntimeFreezeReason: UlxNodeRuntimeRuntimeFreezeReason;
    UlxNodeRuntimeRuntimeHoldReason: UlxNodeRuntimeRuntimeHoldReason;
    UlxNotaryAuditAccountHistoryLookupError: UlxNotaryAuditAccountHistoryLookupError;
    UlxNotaryAuditErrorVerifyError: UlxNotaryAuditErrorVerifyError;
    UlxPrimitivesAccountAccountType: UlxPrimitivesAccountAccountType;
    UlxPrimitivesBalanceChangeAccountOrigin: UlxPrimitivesBalanceChangeAccountOrigin;
    UlxPrimitivesBalanceChangeMerkleProof: UlxPrimitivesBalanceChangeMerkleProof;
    UlxPrimitivesBlockSealAppPublic: UlxPrimitivesBlockSealAppPublic;
    UlxPrimitivesBlockSealAppSignature: UlxPrimitivesBlockSealAppSignature;
    UlxPrimitivesBlockSealMiningRegistration: UlxPrimitivesBlockSealMiningRegistration;
    UlxPrimitivesBlockSealRewardDestination: UlxPrimitivesBlockSealRewardDestination;
    UlxPrimitivesBlockVoteBlockVoteT: UlxPrimitivesBlockVoteBlockVoteT;
    UlxPrimitivesBond: UlxPrimitivesBond;
    UlxPrimitivesBondBondFund: UlxPrimitivesBondBondFund;
    UlxPrimitivesDataDomainSemver: UlxPrimitivesDataDomainSemver;
    UlxPrimitivesDataDomainVersionHost: UlxPrimitivesDataDomainVersionHost;
    UlxPrimitivesDataDomainZoneRecord: UlxPrimitivesDataDomainZoneRecord;
    UlxPrimitivesDigestsBlockVoteDigest: UlxPrimitivesDigestsBlockVoteDigest;
    UlxPrimitivesDigestsNotebookDigest: UlxPrimitivesDigestsNotebookDigest;
    UlxPrimitivesDigestsNotebookDigestRecord: UlxPrimitivesDigestsNotebookDigestRecord;
    UlxPrimitivesDigestsParentVotingKeyDigest: UlxPrimitivesDigestsParentVotingKeyDigest;
    UlxPrimitivesHost: UlxPrimitivesHost;
    UlxPrimitivesInherentsBlockSealInherent: UlxPrimitivesInherentsBlockSealInherent;
    UlxPrimitivesNotaryNotaryMeta: UlxPrimitivesNotaryNotaryMeta;
    UlxPrimitivesNotaryNotaryNotebookKeyDetails: UlxPrimitivesNotaryNotaryNotebookKeyDetails;
    UlxPrimitivesNotaryNotaryNotebookVoteDigestDetails: UlxPrimitivesNotaryNotaryNotebookVoteDigestDetails;
    UlxPrimitivesNotaryNotaryRecord: UlxPrimitivesNotaryNotaryRecord;
    UlxPrimitivesNotebookChainTransfer: UlxPrimitivesNotebookChainTransfer;
    UlxPrimitivesNotebookNotebookHeader: UlxPrimitivesNotebookNotebookHeader;
    UlxPrimitivesNotebookSignedNotebookHeader: UlxPrimitivesNotebookSignedNotebookHeader;
    UlxPrimitivesProvidersBlockSealerInfo: UlxPrimitivesProvidersBlockSealerInfo;
  } // InterfaceTypes
} // declare module
