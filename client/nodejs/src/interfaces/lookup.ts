// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
  /**
   * Lookup3: frame_system::AccountInfo<Nonce, pallet_balances::types::AccountData<Balance>>
   **/
  FrameSystemAccountInfo: {
    nonce: 'u32',
    consumers: 'u32',
    providers: 'u32',
    sufficients: 'u32',
    data: 'PalletBalancesAccountData'
  },
  /**
   * Lookup5: pallet_balances::types::AccountData<Balance>
   **/
  PalletBalancesAccountData: {
    free: 'u128',
    reserved: 'u128',
    frozen: 'u128',
    flags: 'u128'
  },
  /**
   * Lookup9: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
   **/
  FrameSupportDispatchPerDispatchClassWeight: {
    normal: 'SpWeightsWeightV2Weight',
    operational: 'SpWeightsWeightV2Weight',
    mandatory: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup10: sp_weights::weight_v2::Weight
   **/
  SpWeightsWeightV2Weight: {
    refTime: 'Compact<u64>',
    proofSize: 'Compact<u64>'
  },
  /**
   * Lookup15: sp_runtime::generic::digest::Digest
   **/
  SpRuntimeDigest: {
    logs: 'Vec<SpRuntimeDigestDigestItem>'
  },
  /**
   * Lookup17: sp_runtime::generic::digest::DigestItem
   **/
  SpRuntimeDigestDigestItem: {
    _enum: {
      Other: 'Bytes',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      Consensus: '([u8;4],Bytes)',
      Seal: '([u8;4],Bytes)',
      PreRuntime: '([u8;4],Bytes)',
      __Unused7: 'Null',
      RuntimeEnvironmentUpdated: 'Null'
    }
  },
  /**
   * Lookup20: frame_system::EventRecord<ulx_node_runtime::RuntimeEvent, primitive_types::H256>
   **/
  FrameSystemEventRecord: {
    phase: 'FrameSystemPhase',
    event: 'Event',
    topics: 'Vec<H256>'
  },
  /**
   * Lookup22: frame_system::pallet::Event<T>
   **/
  FrameSystemEvent: {
    _enum: {
      ExtrinsicSuccess: {
        dispatchInfo: 'FrameSupportDispatchDispatchInfo',
      },
      ExtrinsicFailed: {
        dispatchError: 'SpRuntimeDispatchError',
        dispatchInfo: 'FrameSupportDispatchDispatchInfo',
      },
      CodeUpdated: 'Null',
      NewAccount: {
        account: 'AccountId32',
      },
      KilledAccount: {
        account: 'AccountId32',
      },
      Remarked: {
        _alias: {
          hash_: 'hash',
        },
        sender: 'AccountId32',
        hash_: 'H256',
      },
      UpgradeAuthorized: {
        codeHash: 'H256',
        checkVersion: 'bool'
      }
    }
  },
  /**
   * Lookup23: frame_support::dispatch::DispatchInfo
   **/
  FrameSupportDispatchDispatchInfo: {
    weight: 'SpWeightsWeightV2Weight',
    class: 'FrameSupportDispatchDispatchClass',
    paysFee: 'FrameSupportDispatchPays'
  },
  /**
   * Lookup24: frame_support::dispatch::DispatchClass
   **/
  FrameSupportDispatchDispatchClass: {
    _enum: ['Normal', 'Operational', 'Mandatory']
  },
  /**
   * Lookup25: frame_support::dispatch::Pays
   **/
  FrameSupportDispatchPays: {
    _enum: ['Yes', 'No']
  },
  /**
   * Lookup26: sp_runtime::DispatchError
   **/
  SpRuntimeDispatchError: {
    _enum: {
      Other: 'Null',
      CannotLookup: 'Null',
      BadOrigin: 'Null',
      Module: 'SpRuntimeModuleError',
      ConsumerRemaining: 'Null',
      NoProviders: 'Null',
      TooManyConsumers: 'Null',
      Token: 'SpRuntimeTokenError',
      Arithmetic: 'SpArithmeticArithmeticError',
      Transactional: 'SpRuntimeTransactionalError',
      Exhausted: 'Null',
      Corruption: 'Null',
      Unavailable: 'Null',
      RootNotAllowed: 'Null'
    }
  },
  /**
   * Lookup27: sp_runtime::ModuleError
   **/
  SpRuntimeModuleError: {
    index: 'u8',
    error: '[u8;4]'
  },
  /**
   * Lookup28: sp_runtime::TokenError
   **/
  SpRuntimeTokenError: {
    _enum: ['FundsUnavailable', 'OnlyProvider', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported', 'CannotCreateHold', 'NotExpendable', 'Blocked']
  },
  /**
   * Lookup29: sp_arithmetic::ArithmeticError
   **/
  SpArithmeticArithmeticError: {
    _enum: ['Underflow', 'Overflow', 'DivisionByZero']
  },
  /**
   * Lookup30: sp_runtime::TransactionalError
   **/
  SpRuntimeTransactionalError: {
    _enum: ['LimitReached', 'NoLayer']
  },
  /**
   * Lookup31: pallet_proxy::pallet::Event<T>
   **/
  PalletProxyEvent: {
    _enum: {
      ProxyExecuted: {
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      PureCreated: {
        pure: 'AccountId32',
        who: 'AccountId32',
        proxyType: 'UlxNodeRuntimeProxyType',
        disambiguationIndex: 'u16',
      },
      Announced: {
        real: 'AccountId32',
        proxy: 'AccountId32',
        callHash: 'H256',
      },
      ProxyAdded: {
        delegator: 'AccountId32',
        delegatee: 'AccountId32',
        proxyType: 'UlxNodeRuntimeProxyType',
        delay: 'u32',
      },
      ProxyRemoved: {
        delegator: 'AccountId32',
        delegatee: 'AccountId32',
        proxyType: 'UlxNodeRuntimeProxyType',
        delay: 'u32'
      }
    }
  },
  /**
   * Lookup34: ulx_node_runtime::ProxyType
   **/
  UlxNodeRuntimeProxyType: {
    _enum: ['Any', 'NonTransfer', 'PriceIndex']
  },
  /**
   * Lookup36: pallet_mining_slot::pallet::Event<T>
   **/
  PalletMiningSlotEvent: {
    _enum: {
      NewMiners: {
        startIndex: 'u32',
        newMiners: 'Vec<UlxPrimitivesBlockSealMiningRegistration>',
      },
      SlotBidderAdded: {
        accountId: 'AccountId32',
        bidAmount: 'u128',
        index: 'u32',
      },
      SlotBidderReplaced: {
        accountId: 'AccountId32',
        bondId: 'Option<u64>',
        keptOwnershipBond: 'bool',
      },
      UnbondedMiner: {
        accountId: 'AccountId32',
        bondId: 'Option<u64>',
        keptOwnershipBond: 'bool'
      }
    }
  },
  /**
   * Lookup38: ulx_primitives::block_seal::MiningRegistration<sp_core::crypto::AccountId32, Balance>
   **/
  UlxPrimitivesBlockSealMiningRegistration: {
    accountId: 'AccountId32',
    rewardDestination: 'UlxPrimitivesBlockSealRewardDestination',
    bondId: 'Option<u64>',
    bondAmount: 'Compact<u128>',
    ownershipTokens: 'Compact<u128>'
  },
  /**
   * Lookup39: ulx_primitives::block_seal::RewardDestination<sp_core::crypto::AccountId32>
   **/
  UlxPrimitivesBlockSealRewardDestination: {
    _enum: {
      Owner: 'Null',
      Account: 'AccountId32'
    }
  },
  /**
   * Lookup43: pallet_bitcoin_utxos::pallet::Event<T>
   **/
  PalletBitcoinUtxosEvent: {
    _enum: {
      UtxoVerified: {
        utxoId: 'u64',
      },
      UtxoRejected: {
        utxoId: 'u64',
        rejectedReason: 'UlxPrimitivesBitcoinBitcoinRejectedReason',
      },
      UtxoSpent: {
        utxoId: 'u64',
        blockHeight: 'u64',
      },
      UtxoUnwatched: {
        utxoId: 'u64'
      }
    }
  },
  /**
   * Lookup44: ulx_primitives::bitcoin::BitcoinRejectedReason
   **/
  UlxPrimitivesBitcoinBitcoinRejectedReason: {
    _enum: ['SatoshisMismatch', 'Spent', 'LookupExpired', 'DuplicateUtxo']
  },
  /**
   * Lookup45: pallet_vaults::pallet::Event<T>
   **/
  PalletVaultsEvent: {
    _enum: {
      VaultCreated: {
        vaultId: 'u32',
        bitcoinArgons: 'u128',
        miningArgons: 'u128',
        securitizationPercent: 'u128',
        operatorAccountId: 'AccountId32',
      },
      VaultModified: {
        vaultId: 'u32',
        bitcoinArgons: 'u128',
        miningArgons: 'u128',
        securitizationPercent: 'u128',
      },
      VaultClosed: {
        vaultId: 'u32',
        bitcoinAmountStillBonded: 'u128',
        miningAmountStillBonded: 'u128',
        securitizationStillBonded: 'u128'
      }
    }
  },
  /**
   * Lookup47: pallet_bond::pallet::Event<T>
   **/
  PalletBondEvent: {
    _enum: {
      BondCreated: {
        vaultId: 'u32',
        bondId: 'u64',
        bondType: 'UlxPrimitivesBondBondType',
        bondedAccountId: 'AccountId32',
        utxoId: 'Option<u64>',
        amount: 'u128',
        expiration: 'UlxPrimitivesBondBondExpiration',
      },
      BondCompleted: {
        vaultId: 'u32',
        bondId: 'u64',
      },
      BondCanceled: {
        vaultId: 'u32',
        bondId: 'u64',
        bondedAccountId: 'AccountId32',
        bondType: 'UlxPrimitivesBondBondType',
        returnedFee: 'u128',
      },
      BitcoinBondBurned: {
        vaultId: 'u32',
        bondId: 'u64',
        utxoId: 'u64',
        amountBurned: 'u128',
        amountHeld: 'u128',
        wasUtxoSpent: 'bool',
      },
      BitcoinUtxoCosignRequested: {
        bondId: 'u64',
        vaultId: 'u32',
        utxoId: 'u64',
      },
      BitcoinUtxoCosigned: {
        bondId: 'u64',
        vaultId: 'u32',
        utxoId: 'u64',
        pubkey: 'UlxPrimitivesBitcoinCompressedBitcoinPubkey',
        signature: 'Bytes',
      },
      BitcoinCosignPastDue: {
        bondId: 'u64',
        vaultId: 'u32',
        utxoId: 'u64',
        compensationAmount: 'u128',
        compensationStillOwed: 'u128',
        compensatedAccountId: 'AccountId32'
      }
    }
  },
  /**
   * Lookup48: ulx_primitives::bond::BondType
   **/
  UlxPrimitivesBondBondType: {
    _enum: ['Mining', 'Bitcoin']
  },
  /**
   * Lookup49: ulx_primitives::bond::BondExpiration<BlockNumber>
   **/
  UlxPrimitivesBondBondExpiration: {
    _enum: {
      UlixeeBlock: 'u32',
      BitcoinBlock: 'u64'
    }
  },
  /**
   * Lookup50: ulx_primitives::bitcoin::CompressedBitcoinPubkey
   **/
  UlxPrimitivesBitcoinCompressedBitcoinPubkey: '[u8;33]',
  /**
   * Lookup54: pallet_notaries::pallet::Event<T>
   **/
  PalletNotariesEvent: {
    _enum: {
      NotaryProposed: {
        operatorAccount: 'AccountId32',
        meta: 'UlxPrimitivesNotaryNotaryMeta',
        expires: 'u32',
      },
      NotaryActivated: {
        notary: 'UlxPrimitivesNotaryNotaryRecord',
      },
      NotaryMetaUpdateQueued: {
        notaryId: 'u32',
        meta: 'UlxPrimitivesNotaryNotaryMeta',
        effectiveTick: 'u32',
      },
      NotaryMetaUpdated: {
        notaryId: 'u32',
        meta: 'UlxPrimitivesNotaryNotaryMeta'
      }
    }
  },
  /**
   * Lookup55: ulx_primitives::notary::NotaryMeta<MaxHosts>
   **/
  UlxPrimitivesNotaryNotaryMeta: {
    public: '[u8;32]',
    hosts: 'Vec<Bytes>'
  },
  /**
   * Lookup60: ulx_primitives::notary::NotaryRecord<sp_core::crypto::AccountId32, BlockNumber, MaxHosts>
   **/
  UlxPrimitivesNotaryNotaryRecord: {
    notaryId: 'Compact<u32>',
    operatorAccountId: 'AccountId32',
    activatedBlock: 'Compact<u32>',
    metaUpdatedBlock: 'Compact<u32>',
    metaUpdatedTick: 'Compact<u32>',
    meta: 'UlxPrimitivesNotaryNotaryMeta'
  },
  /**
   * Lookup62: pallet_notebook::pallet::Event<T>
   **/
  PalletNotebookEvent: {
    _enum: {
      NotebookSubmitted: {
        notaryId: 'u32',
        notebookNumber: 'u32',
      },
      NotebookAuditFailure: {
        notaryId: 'u32',
        notebookNumber: 'u32',
        firstFailureReason: 'UlxNotaryAuditErrorVerifyError'
      }
    }
  },
  /**
   * Lookup63: ulx_notary_audit::error::VerifyError
   **/
  UlxNotaryAuditErrorVerifyError: {
    _enum: {
      MissingAccountOrigin: {
        accountId: 'AccountId32',
        accountType: 'UlxPrimitivesAccountAccountType',
      },
      HistoryLookupError: {
        source: 'UlxNotaryAuditAccountHistoryLookupError',
      },
      InvalidAccountChangelist: 'Null',
      InvalidChainTransfersList: 'Null',
      InvalidBalanceChangeRoot: 'Null',
      InvalidHeaderTaxRecorded: 'Null',
      InvalidPreviousNonce: 'Null',
      InvalidPreviousBalance: 'Null',
      InvalidPreviousAccountOrigin: 'Null',
      InvalidPreviousBalanceChangeNotebook: 'Null',
      InvalidBalanceChange: 'Null',
      InvalidBalanceChangeSignature: {
        changeIndex: 'u16',
      },
      InvalidNoteRecipients: 'Null',
      BalanceChangeError: {
        changeIndex: 'u16',
        noteIndex: 'u16',
        message: 'Text',
      },
      InvalidNetBalanceChangeset: 'Null',
      InsufficientBalance: {
        balance: 'u128',
        amount: 'u128',
        noteIndex: 'u16',
        changeIndex: 'u16',
      },
      ExceededMaxBalance: {
        balance: 'u128',
        amount: 'u128',
        noteIndex: 'u16',
        changeIndex: 'u16',
      },
      BalanceChangeMismatch: {
        changeIndex: 'u16',
        providedBalance: 'u128',
        calculatedBalance: 'i128',
      },
      BalanceChangeNotNetZero: {
        sent: 'u128',
        claimed: 'u128',
      },
      InvalidDomainLeaseAllocation: 'Null',
      TaxBalanceChangeNotNetZero: {
        sent: 'u128',
        claimed: 'u128',
      },
      MissingBalanceProof: 'Null',
      InvalidPreviousBalanceProof: 'Null',
      InvalidNotebookHash: 'Null',
      InvalidNotebookHeaderHash: 'Null',
      DuplicateChainTransfer: 'Null',
      DuplicatedAccountOriginUid: 'Null',
      InvalidNotarySignature: 'Null',
      NotebookTooOld: 'Null',
      CatchupNotebooksMissing: 'Null',
      DecodeError: 'Null',
      AccountEscrowHoldDoesntExist: 'Null',
      AccountAlreadyHasEscrowHold: 'Null',
      EscrowHoldNotReadyForClaim: {
        currentTick: 'u32',
        claimTick: 'u32',
      },
      AccountLocked: 'Null',
      MissingEscrowHoldNote: 'Null',
      InvalidEscrowHoldNote: 'Null',
      InvalidEscrowClaimers: 'Null',
      EscrowNoteBelowMinimum: 'Null',
      InvalidTaxNoteAccount: 'Null',
      InvalidTaxOperation: 'Null',
      InsufficientTaxIncluded: {
        taxSent: 'u128',
        taxOwed: 'u128',
        accountId: 'AccountId32',
      },
      InsufficientBlockVoteTax: 'Null',
      IneligibleTaxVoter: 'Null',
      BlockVoteInvalidSignature: 'Null',
      InvalidBlockVoteAllocation: 'Null',
      InvalidBlockVoteRoot: 'Null',
      InvalidBlockVotesCount: 'Null',
      InvalidBlockVotingPower: 'Null',
      InvalidBlockVoteList: 'Null',
      InvalidComputeProof: 'Null',
      InvalidBlockVoteSource: 'Null',
      InsufficientBlockVoteMinimum: 'Null',
      BlockVoteDataDomainMismatch: 'Null',
      BlockVoteEscrowReused: 'Null'
    }
  },
  /**
   * Lookup64: ulx_primitives::account::AccountType
   **/
  UlxPrimitivesAccountAccountType: {
    _enum: ['Tax', 'Deposit']
  },
  /**
   * Lookup65: ulx_notary_audit::AccountHistoryLookupError
   **/
  UlxNotaryAuditAccountHistoryLookupError: {
    _enum: ['RootNotFound', 'LastChangeNotFound', 'InvalidTransferToLocalchain', 'BlockSpecificationNotFound']
  },
  /**
   * Lookup68: pallet_chain_transfer::pallet::Event<T>
   **/
  PalletChainTransferEvent: {
    _enum: {
      TransferToLocalchain: {
        accountId: 'AccountId32',
        amount: 'u128',
        transferId: 'u32',
        notaryId: 'u32',
        expirationBlock: 'u32',
      },
      TransferToLocalchainExpired: {
        accountId: 'AccountId32',
        transferId: 'u32',
        notaryId: 'u32',
      },
      TransferIn: {
        accountId: 'AccountId32',
        amount: 'u128',
        notaryId: 'u32'
      }
    }
  },
  /**
   * Lookup69: pallet_block_seal_spec::pallet::Event<T>
   **/
  PalletBlockSealSpecEvent: {
    _enum: {
      VoteMinimumAdjusted: {
        expectedBlockVotes: 'u128',
        actualBlockVotes: 'u128',
        startVoteMinimum: 'u128',
        newVoteMinimum: 'u128',
      },
      ComputeDifficultyAdjusted: {
        expectedBlockTime: 'u64',
        actualBlockTime: 'u64',
        startDifficulty: 'u128',
        newDifficulty: 'u128'
      }
    }
  },
  /**
   * Lookup70: pallet_data_domain::pallet::Event<T>
   **/
  PalletDataDomainEvent: {
    _enum: {
      ZoneRecordUpdated: {
        domainHash: 'H256',
        zoneRecord: 'UlxPrimitivesDataDomainZoneRecord',
      },
      DataDomainRegistered: {
        domainHash: 'H256',
        registration: 'PalletDataDomainDataDomainRegistration',
      },
      DataDomainRenewed: {
        domainHash: 'H256',
      },
      DataDomainExpired: {
        domainHash: 'H256',
      },
      DataDomainRegistrationCanceled: {
        domainHash: 'H256',
        registration: 'PalletDataDomainDataDomainRegistration'
      }
    }
  },
  /**
   * Lookup71: ulx_primitives::data_domain::ZoneRecord<sp_core::crypto::AccountId32>
   **/
  UlxPrimitivesDataDomainZoneRecord: {
    paymentAccount: 'AccountId32',
    notaryId: 'u32',
    versions: 'BTreeMap<UlxPrimitivesDataDomainSemver, UlxPrimitivesDataDomainVersionHost>'
  },
  /**
   * Lookup73: ulx_primitives::data_domain::Semver
   **/
  UlxPrimitivesDataDomainSemver: {
    major: 'u32',
    minor: 'u32',
    patch: 'u32'
  },
  /**
   * Lookup74: ulx_primitives::data_domain::VersionHost
   **/
  UlxPrimitivesDataDomainVersionHost: {
    datastoreId: 'Bytes',
    host: 'Bytes'
  },
  /**
   * Lookup78: pallet_data_domain::DataDomainRegistration<sp_core::crypto::AccountId32>
   **/
  PalletDataDomainDataDomainRegistration: {
    accountId: 'AccountId32',
    registeredAtTick: 'u32'
  },
  /**
   * Lookup79: pallet_price_index::pallet::Event<T>
   **/
  PalletPriceIndexEvent: {
    _enum: {
      NewIndex: 'Null',
      OperatorChanged: {
        operatorId: 'AccountId32'
      }
    }
  },
  /**
   * Lookup80: pallet_session::pallet::Event
   **/
  PalletSessionEvent: {
    _enum: {
      NewSession: {
        sessionIndex: 'u32'
      }
    }
  },
  /**
   * Lookup81: pallet_block_rewards::pallet::Event<T>
   **/
  PalletBlockRewardsEvent: {
    _enum: {
      RewardCreated: {
        maturationBlock: 'u32',
        rewards: 'Vec<UlxPrimitivesBlockSealBlockPayout>',
      },
      RewardUnlocked: {
        rewards: 'Vec<UlxPrimitivesBlockSealBlockPayout>'
      }
    }
  },
  /**
   * Lookup83: ulx_primitives::block_seal::BlockPayout<sp_core::crypto::AccountId32, Balance>
   **/
  UlxPrimitivesBlockSealBlockPayout: {
    accountId: 'AccountId32',
    ulixees: 'u128',
    argons: 'u128'
  },
  /**
   * Lookup84: pallet_grandpa::pallet::Event
   **/
  PalletGrandpaEvent: {
    _enum: {
      NewAuthorities: {
        authoritySet: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
      },
      Paused: 'Null',
      Resumed: 'Null'
    }
  },
  /**
   * Lookup87: sp_consensus_grandpa::app::Public
   **/
  SpConsensusGrandpaAppPublic: '[u8;32]',
  /**
   * Lookup88: pallet_offences::pallet::Event
   **/
  PalletOffencesEvent: {
    _enum: {
      Offence: {
        kind: '[u8;16]',
        timeslot: 'Bytes'
      }
    }
  },
  /**
   * Lookup90: pallet_mint::pallet::Event<T>
   **/
  PalletMintEvent: {
    _enum: {
      ArgonsMinted: {
        mintType: 'PalletMintMintType',
        accountId: 'AccountId32',
        utxoId: 'Option<u64>',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup91: pallet_mint::pallet::MintType
   **/
  PalletMintMintType: {
    _enum: ['Bitcoin', 'Ulixee']
  },
  /**
   * Lookup92: pallet_balances::pallet::Event<T, I>
   **/
  PalletBalancesEvent: {
    _enum: {
      Endowed: {
        account: 'AccountId32',
        freeBalance: 'u128',
      },
      DustLost: {
        account: 'AccountId32',
        amount: 'u128',
      },
      Transfer: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
      },
      BalanceSet: {
        who: 'AccountId32',
        free: 'u128',
      },
      Reserved: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Unreserved: {
        who: 'AccountId32',
        amount: 'u128',
      },
      ReserveRepatriated: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
        destinationStatus: 'FrameSupportTokensMiscBalanceStatus',
      },
      Deposit: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Withdraw: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Slashed: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Minted: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Burned: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Suspended: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Restored: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Upgraded: {
        who: 'AccountId32',
      },
      Issued: {
        amount: 'u128',
      },
      Rescinded: {
        amount: 'u128',
      },
      Locked: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Unlocked: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Frozen: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Thawed: {
        who: 'AccountId32',
        amount: 'u128',
      },
      TotalIssuanceForced: {
        _alias: {
          new_: 'new',
        },
        old: 'u128',
        new_: 'u128'
      }
    }
  },
  /**
   * Lookup93: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup95: pallet_tx_pause::pallet::Event<T>
   **/
  PalletTxPauseEvent: {
    _enum: {
      CallPaused: {
        fullName: '(Bytes,Bytes)',
      },
      CallUnpaused: {
        fullName: '(Bytes,Bytes)'
      }
    }
  },
  /**
   * Lookup98: pallet_transaction_payment::pallet::Event<T>
   **/
  PalletTransactionPaymentEvent: {
    _enum: {
      TransactionFeePaid: {
        who: 'AccountId32',
        actualFee: 'u128',
        tip: 'u128'
      }
    }
  },
  /**
   * Lookup99: pallet_sudo::pallet::Event<T>
   **/
  PalletSudoEvent: {
    _enum: {
      Sudid: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>',
      },
      KeyChanged: {
        _alias: {
          new_: 'new',
        },
        old: 'Option<AccountId32>',
        new_: 'AccountId32',
      },
      KeyRemoved: 'Null',
      SudoAsDone: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup101: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup105: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup106: frame_system::CodeUpgradeAuthorization<T>
   **/
  FrameSystemCodeUpgradeAuthorization: {
    codeHash: 'H256',
    checkVersion: 'bool'
  },
  /**
   * Lookup107: frame_system::pallet::Call<T>
   **/
  FrameSystemCall: {
    _enum: {
      remark: {
        remark: 'Bytes',
      },
      set_heap_pages: {
        pages: 'u64',
      },
      set_code: {
        code: 'Bytes',
      },
      set_code_without_checks: {
        code: 'Bytes',
      },
      set_storage: {
        items: 'Vec<(Bytes,Bytes)>',
      },
      kill_storage: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'Vec<Bytes>',
      },
      kill_prefix: {
        prefix: 'Bytes',
        subkeys: 'u32',
      },
      remark_with_event: {
        remark: 'Bytes',
      },
      __Unused8: 'Null',
      authorize_upgrade: {
        codeHash: 'H256',
      },
      authorize_upgrade_without_checks: {
        codeHash: 'H256',
      },
      apply_authorized_upgrade: {
        code: 'Bytes'
      }
    }
  },
  /**
   * Lookup111: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'SpWeightsWeightV2Weight',
    maxBlock: 'SpWeightsWeightV2Weight',
    perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup112: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportDispatchPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup113: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'SpWeightsWeightV2Weight',
    maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
    maxTotal: 'Option<SpWeightsWeightV2Weight>',
    reserved: 'Option<SpWeightsWeightV2Weight>'
  },
  /**
   * Lookup115: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportDispatchPerDispatchClassU32'
  },
  /**
   * Lookup116: frame_support::dispatch::PerDispatchClass<T>
   **/
  FrameSupportDispatchPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup117: sp_weights::RuntimeDbWeight
   **/
  SpWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup118: sp_version::RuntimeVersion
   **/
  SpVersionRuntimeVersion: {
    specName: 'Text',
    implName: 'Text',
    authoringVersion: 'u32',
    specVersion: 'u32',
    implVersion: 'u32',
    apis: 'Vec<([u8;8],u32)>',
    transactionVersion: 'u32',
    stateVersion: 'u8'
  },
  /**
   * Lookup123: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered', 'MultiBlockMigrationsOngoing', 'NothingAuthorized', 'Unauthorized']
  },
  /**
   * Lookup124: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup127: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, ulx_node_runtime::ProxyType, BlockNumber>
   **/
  PalletProxyProxyDefinition: {
    delegate: 'AccountId32',
    proxyType: 'UlxNodeRuntimeProxyType',
    delay: 'u32'
  },
  /**
   * Lookup131: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
   **/
  PalletProxyAnnouncement: {
    real: 'AccountId32',
    callHash: 'H256',
    height: 'u32'
  },
  /**
   * Lookup133: pallet_proxy::pallet::Call<T>
   **/
  PalletProxyCall: {
    _enum: {
      proxy: {
        real: 'MultiAddress',
        forceProxyType: 'Option<UlxNodeRuntimeProxyType>',
        call: 'Call',
      },
      add_proxy: {
        delegate: 'MultiAddress',
        proxyType: 'UlxNodeRuntimeProxyType',
        delay: 'u32',
      },
      remove_proxy: {
        delegate: 'MultiAddress',
        proxyType: 'UlxNodeRuntimeProxyType',
        delay: 'u32',
      },
      remove_proxies: 'Null',
      create_pure: {
        proxyType: 'UlxNodeRuntimeProxyType',
        delay: 'u32',
        index: 'u16',
      },
      kill_pure: {
        spawner: 'MultiAddress',
        proxyType: 'UlxNodeRuntimeProxyType',
        index: 'u16',
        height: 'Compact<u32>',
        extIndex: 'Compact<u32>',
      },
      announce: {
        real: 'MultiAddress',
        callHash: 'H256',
      },
      remove_announcement: {
        real: 'MultiAddress',
        callHash: 'H256',
      },
      reject_announcement: {
        delegate: 'MultiAddress',
        callHash: 'H256',
      },
      proxy_announced: {
        delegate: 'MultiAddress',
        real: 'MultiAddress',
        forceProxyType: 'Option<UlxNodeRuntimeProxyType>',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup139: pallet_ticks::pallet::Call<T>
   **/
  PalletTicksCall: 'Null',
  /**
   * Lookup140: pallet_mining_slot::pallet::Call<T>
   **/
  PalletMiningSlotCall: {
    _enum: {
      bid: {
        bondInfo: 'Option<PalletMiningSlotMiningSlotBid>',
        rewardDestination: 'UlxPrimitivesBlockSealRewardDestination'
      }
    }
  },
  /**
   * Lookup142: pallet_mining_slot::MiningSlotBid<VaultId, Balance>
   **/
  PalletMiningSlotMiningSlotBid: {
    vaultId: 'u32',
    amount: 'u128'
  },
  /**
   * Lookup143: pallet_bitcoin_utxos::pallet::Call<T>
   **/
  PalletBitcoinUtxosCall: {
    _enum: {
      sync: {
        utxoSync: 'UlxPrimitivesInherentsBitcoinUtxoSync',
      },
      set_confirmed_block: {
        bitcoinHeight: 'u64',
        bitcoinBlockHash: 'UlxPrimitivesBitcoinH256Le',
      },
      set_operator: {
        accountId: 'AccountId32'
      }
    }
  },
  /**
   * Lookup144: ulx_primitives::inherents::BitcoinUtxoSync
   **/
  UlxPrimitivesInherentsBitcoinUtxoSync: {
    spent: 'BTreeMap<u64, u64>',
    verified: 'BTreeMap<u64, UlxPrimitivesBitcoinUtxoRef>',
    invalid: 'BTreeMap<u64, UlxPrimitivesBitcoinBitcoinRejectedReason>',
    syncToBlock: 'UlxPrimitivesBitcoinBitcoinBlock'
  },
  /**
   * Lookup149: ulx_primitives::bitcoin::UtxoRef
   **/
  UlxPrimitivesBitcoinUtxoRef: {
    txid: 'UlxPrimitivesBitcoinH256Le',
    outputIndex: 'Compact<u32>'
  },
  /**
   * Lookup150: ulx_primitives::bitcoin::H256Le
   **/
  UlxPrimitivesBitcoinH256Le: '[u8;32]',
  /**
   * Lookup156: ulx_primitives::bitcoin::BitcoinBlock
   **/
  UlxPrimitivesBitcoinBitcoinBlock: {
    blockHeight: 'Compact<u64>',
    blockHash: 'UlxPrimitivesBitcoinH256Le'
  },
  /**
   * Lookup157: pallet_vaults::pallet::Call<T>
   **/
  PalletVaultsCall: {
    _enum: {
      create: {
        bitcoinAnnualPercentRate: 'Compact<u128>',
        miningAnnualPercentRate: 'Compact<u128>',
        bitcoinAmountAllocated: 'Compact<u128>',
        miningAmountAllocated: 'Compact<u128>',
        securitizationPercent: 'Compact<u128>',
        bitcoinPubkeyHashes: 'Vec<UlxPrimitivesBitcoinBitcoinPubkeyHash>',
      },
      modify: {
        vaultId: 'u32',
        totalMiningAmountOffered: 'u128',
        totalBitcoinAmountOffered: 'u128',
        securitizationPercent: 'u128',
      },
      close: {
        vaultId: 'u32',
      },
      add_bitcoin_pubkey_hashes: {
        vaultId: 'u32',
        bitcoinPubkeyHashes: 'Vec<UlxPrimitivesBitcoinBitcoinPubkeyHash>'
      }
    }
  },
  /**
   * Lookup160: ulx_primitives::bitcoin::BitcoinPubkeyHash
   **/
  UlxPrimitivesBitcoinBitcoinPubkeyHash: '[u8;20]',
  /**
   * Lookup162: pallet_bond::pallet::Call<T>
   **/
  PalletBondCall: {
    _enum: {
      bond_bitcoin: {
        vaultId: 'u32',
        satoshis: 'Compact<u64>',
        bitcoinPubkeyHash: 'UlxPrimitivesBitcoinBitcoinPubkeyHash',
      },
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      unlock_bitcoin_bond: {
        bondId: 'u64',
        toScriptPubkey: 'Bytes',
        bitcoinNetworkFee: 'u64',
      },
      cosign_bitcoin_unlock: {
        bondId: 'u64',
        pubkey: 'UlxPrimitivesBitcoinCompressedBitcoinPubkey',
        signature: 'Bytes'
      }
    }
  },
  /**
   * Lookup165: pallet_notaries::pallet::Call<T>
   **/
  PalletNotariesCall: {
    _enum: {
      propose: {
        meta: 'UlxPrimitivesNotaryNotaryMeta',
      },
      activate: {
        operatorAccount: 'AccountId32',
      },
      update: {
        notaryId: 'Compact<u32>',
        meta: 'UlxPrimitivesNotaryNotaryMeta',
        effectiveTick: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup166: pallet_notebook::pallet::Call<T>
   **/
  PalletNotebookCall: {
    _enum: {
      submit: {
        notebooks: 'Vec<UlxPrimitivesNotebookSignedNotebookHeader>'
      }
    }
  },
  /**
   * Lookup168: ulx_primitives::notebook::SignedNotebookHeader
   **/
  UlxPrimitivesNotebookSignedNotebookHeader: {
    header: 'UlxPrimitivesNotebookNotebookHeader',
    signature: '[u8;64]'
  },
  /**
   * Lookup169: ulx_primitives::notebook::NotebookHeader
   **/
  UlxPrimitivesNotebookNotebookHeader: {
    version: 'Compact<u16>',
    notebookNumber: 'Compact<u32>',
    tick: 'Compact<u32>',
    tax: 'Compact<u128>',
    notaryId: 'Compact<u32>',
    chainTransfers: 'Vec<UlxPrimitivesNotebookChainTransfer>',
    changedAccountsRoot: 'H256',
    changedAccountOrigins: 'Vec<UlxPrimitivesBalanceChangeAccountOrigin>',
    blockVotesRoot: 'H256',
    blockVotesCount: 'Compact<u32>',
    blocksWithVotes: 'Vec<H256>',
    blockVotingPower: 'Compact<u128>',
    secretHash: 'H256',
    parentSecret: 'Option<H256>',
    dataDomains: 'Vec<(H256,AccountId32)>'
  },
  /**
   * Lookup172: ulx_primitives::notebook::ChainTransfer
   **/
  UlxPrimitivesNotebookChainTransfer: {
    _enum: {
      ToMainchain: {
        accountId: 'AccountId32',
        amount: 'Compact<u128>',
      },
      ToLocalchain: {
        transferId: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup175: ulx_primitives::balance_change::AccountOrigin
   **/
  UlxPrimitivesBalanceChangeAccountOrigin: {
    notebookNumber: 'Compact<u32>',
    accountUid: 'Compact<u32>'
  },
  /**
   * Lookup183: pallet_chain_transfer::pallet::Call<T>
   **/
  PalletChainTransferCall: {
    _enum: {
      send_to_localchain: {
        amount: 'Compact<u128>',
        notaryId: 'u32'
      }
    }
  },
  /**
   * Lookup184: pallet_block_seal_spec::pallet::Call<T>
   **/
  PalletBlockSealSpecCall: {
    _enum: {
      configure: {
        voteMinimum: 'Option<u128>',
        computeDifficulty: 'Option<u128>'
      }
    }
  },
  /**
   * Lookup186: pallet_data_domain::pallet::Call<T>
   **/
  PalletDataDomainCall: {
    _enum: {
      set_zone_record: {
        domainHash: 'H256',
        zoneRecord: 'UlxPrimitivesDataDomainZoneRecord'
      }
    }
  },
  /**
   * Lookup187: pallet_price_index::pallet::Call<T>
   **/
  PalletPriceIndexCall: {
    _enum: {
      submit: {
        index: 'PalletPriceIndexPriceIndex',
      },
      set_operator: {
        accountId: 'AccountId32'
      }
    }
  },
  /**
   * Lookup188: pallet_price_index::PriceIndex<Moment>
   **/
  PalletPriceIndexPriceIndex: {
    btcUsdPrice: 'Compact<u128>',
    argonUsdPrice: 'Compact<u128>',
    argonCpi: 'i128',
    timestamp: 'Compact<u64>'
  },
  /**
   * Lookup190: pallet_session::pallet::Call<T>
   **/
  PalletSessionCall: {
    _enum: {
      set_keys: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'UlxNodeRuntimeOpaqueSessionKeys',
        proof: 'Bytes',
      },
      purge_keys: 'Null'
    }
  },
  /**
   * Lookup191: ulx_node_runtime::opaque::SessionKeys
   **/
  UlxNodeRuntimeOpaqueSessionKeys: {
    grandpa: 'SpConsensusGrandpaAppPublic',
    blockSealAuthority: 'UlxPrimitivesBlockSealAppPublic'
  },
  /**
   * Lookup192: ulx_primitives::block_seal::app::Public
   **/
  UlxPrimitivesBlockSealAppPublic: '[u8;32]',
  /**
   * Lookup193: pallet_block_seal::pallet::Call<T>
   **/
  PalletBlockSealCall: {
    _enum: {
      apply: {
        seal: 'UlxPrimitivesInherentsBlockSealInherent'
      }
    }
  },
  /**
   * Lookup194: ulx_primitives::inherents::BlockSealInherent
   **/
  UlxPrimitivesInherentsBlockSealInherent: {
    _enum: {
      Vote: {
        sealStrength: 'U256',
        notaryId: 'Compact<u32>',
        sourceNotebookNumber: 'Compact<u32>',
        sourceNotebookProof: 'UlxPrimitivesBalanceChangeMerkleProof',
        blockVote: 'UlxPrimitivesBlockVoteBlockVoteT',
        minerSignature: 'UlxPrimitivesBlockSealAppSignature',
      },
      Compute: 'Null'
    }
  },
  /**
   * Lookup197: ulx_primitives::balance_change::MerkleProof
   **/
  UlxPrimitivesBalanceChangeMerkleProof: {
    proof: 'Vec<H256>',
    numberOfLeaves: 'Compact<u32>',
    leafIndex: 'Compact<u32>'
  },
  /**
   * Lookup199: ulx_primitives::block_vote::BlockVoteT<primitive_types::H256>
   **/
  UlxPrimitivesBlockVoteBlockVoteT: {
    accountId: 'AccountId32',
    blockHash: 'H256',
    index: 'Compact<u32>',
    power: 'Compact<u128>',
    dataDomainHash: 'H256',
    dataDomainAccount: 'AccountId32',
    signature: 'SpRuntimeMultiSignature',
    blockRewardsAccountId: 'AccountId32'
  },
  /**
   * Lookup200: sp_runtime::MultiSignature
   **/
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: '[u8;64]',
      Sr25519: '[u8;64]',
      Ecdsa: '[u8;65]'
    }
  },
  /**
   * Lookup202: ulx_primitives::block_seal::app::Signature
   **/
  UlxPrimitivesBlockSealAppSignature: '[u8;64]',
  /**
   * Lookup203: pallet_block_rewards::pallet::Call<T>
   **/
  PalletBlockRewardsCall: 'Null',
  /**
   * Lookup204: pallet_grandpa::pallet::Call<T>
   **/
  PalletGrandpaCall: {
    _enum: {
      report_equivocation: {
        equivocationProof: 'SpConsensusGrandpaEquivocationProof',
        keyOwnerProof: 'SpSessionMembershipProof',
      },
      report_equivocation_unsigned: {
        equivocationProof: 'SpConsensusGrandpaEquivocationProof',
        keyOwnerProof: 'SpSessionMembershipProof',
      },
      note_stalled: {
        delay: 'u32',
        bestFinalizedBlockNumber: 'u32'
      }
    }
  },
  /**
   * Lookup205: sp_consensus_grandpa::EquivocationProof<primitive_types::H256, N>
   **/
  SpConsensusGrandpaEquivocationProof: {
    setId: 'u64',
    equivocation: 'SpConsensusGrandpaEquivocation'
  },
  /**
   * Lookup206: sp_consensus_grandpa::Equivocation<primitive_types::H256, N>
   **/
  SpConsensusGrandpaEquivocation: {
    _enum: {
      Prevote: 'FinalityGrandpaEquivocationPrevote',
      Precommit: 'FinalityGrandpaEquivocationPrecommit'
    }
  },
  /**
   * Lookup207: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Prevote<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrevote: {
    roundNumber: 'u64',
    identity: 'SpConsensusGrandpaAppPublic',
    first: '(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)',
    second: '(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)'
  },
  /**
   * Lookup208: finality_grandpa::Prevote<primitive_types::H256, N>
   **/
  FinalityGrandpaPrevote: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup209: sp_consensus_grandpa::app::Signature
   **/
  SpConsensusGrandpaAppSignature: '[u8;64]',
  /**
   * Lookup211: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Precommit<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrecommit: {
    roundNumber: 'u64',
    identity: 'SpConsensusGrandpaAppPublic',
    first: '(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)',
    second: '(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)'
  },
  /**
   * Lookup212: finality_grandpa::Precommit<primitive_types::H256, N>
   **/
  FinalityGrandpaPrecommit: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup214: sp_session::MembershipProof
   **/
  SpSessionMembershipProof: {
    session: 'u32',
    trieNodes: 'Vec<Bytes>',
    validatorCount: 'u32'
  },
  /**
   * Lookup215: pallet_mint::pallet::Call<T>
   **/
  PalletMintCall: 'Null',
  /**
   * Lookup216: pallet_balances::pallet::Call<T, I>
   **/
  PalletBalancesCall: {
    _enum: {
      transfer_allow_death: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      __Unused1: 'Null',
      force_transfer: {
        source: 'MultiAddress',
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      transfer_keep_alive: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      transfer_all: {
        dest: 'MultiAddress',
        keepAlive: 'bool',
      },
      force_unreserve: {
        who: 'MultiAddress',
        amount: 'u128',
      },
      upgrade_accounts: {
        who: 'Vec<AccountId32>',
      },
      __Unused7: 'Null',
      force_set_balance: {
        who: 'MultiAddress',
        newFree: 'Compact<u128>',
      },
      force_adjust_total_issuance: {
        direction: 'PalletBalancesAdjustmentDirection',
        delta: 'Compact<u128>',
      },
      burn: {
        value: 'Compact<u128>',
        keepAlive: 'bool'
      }
    }
  },
  /**
   * Lookup218: pallet_balances::types::AdjustmentDirection
   **/
  PalletBalancesAdjustmentDirection: {
    _enum: ['Increase', 'Decrease']
  },
  /**
   * Lookup220: pallet_tx_pause::pallet::Call<T>
   **/
  PalletTxPauseCall: {
    _enum: {
      pause: {
        fullName: '(Bytes,Bytes)',
      },
      unpause: {
        ident: '(Bytes,Bytes)'
      }
    }
  },
  /**
   * Lookup221: pallet_sudo::pallet::Call<T>
   **/
  PalletSudoCall: {
    _enum: {
      sudo: {
        call: 'Call',
      },
      sudo_unchecked_weight: {
        call: 'Call',
        weight: 'SpWeightsWeightV2Weight',
      },
      set_key: {
        _alias: {
          new_: 'new',
        },
        new_: 'MultiAddress',
      },
      sudo_as: {
        who: 'MultiAddress',
        call: 'Call',
      },
      remove_key: 'Null'
    }
  },
  /**
   * Lookup222: pallet_proxy::pallet::Error<T>
   **/
  PalletProxyError: {
    _enum: ['TooMany', 'NotFound', 'NotProxy', 'Unproxyable', 'Duplicate', 'NoPermission', 'Unannounced', 'NoSelfProxy']
  },
  /**
   * Lookup224: pallet_ticks::pallet::Error<T>
   **/
  PalletTicksError: 'Null',
  /**
   * Lookup230: pallet_mining_slot::pallet::Error<T>
   **/
  PalletMiningSlotError: {
    _enum: {
      SlotNotTakingBids: 'Null',
      TooManyBlockRegistrants: 'Null',
      InsufficientOwnershipTokens: 'Null',
      BidTooLow: 'Null',
      CannotRegisterOverlappingSessions: 'Null',
      BondNotFound: 'Null',
      NoMoreBondIds: 'Null',
      VaultClosed: 'Null',
      MinimumBondAmountNotMet: 'Null',
      ExpirationAtBlockOverflow: 'Null',
      InsufficientFunds: 'Null',
      InsufficientVaultFunds: 'Null',
      ExpirationTooSoon: 'Null',
      NoPermissions: 'Null',
      HoldUnexpectedlyModified: 'Null',
      UnrecoverableHold: 'Null',
      VaultNotFound: 'Null',
      BondAlreadyClosed: 'Null',
      FeeExceedsBondAmount: 'Null',
      AccountWouldBeBelowMinimum: 'Null',
      GenericBondError: 'UlxPrimitivesBondBondError'
    }
  },
  /**
   * Lookup231: ulx_primitives::bond::BondError
   **/
  UlxPrimitivesBondBondError: {
    _enum: ['BondNotFound', 'NoMoreBondIds', 'MinimumBondAmountNotMet', 'VaultClosed', 'ExpirationAtBlockOverflow', 'AccountWouldBeBelowMinimum', 'InsufficientFunds', 'InsufficientVaultFunds', 'InsufficientBitcoinsForMining', 'ExpirationTooSoon', 'NoPermissions', 'HoldUnexpectedlyModified', 'UnrecoverableHold', 'VaultNotFound', 'NoVaultBitcoinPubkeysAvailable', 'FeeExceedsBondAmount', 'InvalidBitcoinScript']
  },
  /**
   * Lookup232: ulx_primitives::bitcoin::UtxoValue
   **/
  UlxPrimitivesBitcoinUtxoValue: {
    utxoId: 'u64',
    scriptPubkey: 'UlxPrimitivesBitcoinBitcoinCosignScriptPubkey',
    satoshis: 'Compact<u64>',
    submittedAtHeight: 'Compact<u64>',
    watchForSpentUntilHeight: 'Compact<u64>'
  },
  /**
   * Lookup233: ulx_primitives::bitcoin::BitcoinCosignScriptPubkey
   **/
  UlxPrimitivesBitcoinBitcoinCosignScriptPubkey: {
    _enum: {
      P2WSH: {
        wscriptHash: 'H256'
      }
    }
  },
  /**
   * Lookup240: pallet_bitcoin_utxos::pallet::Error<T>
   **/
  PalletBitcoinUtxosError: {
    _enum: ['NoPermissions', 'NoBitcoinConfirmedBlock', 'InsufficientBitcoinAmount', 'NoBitcoinPricesAvailable', 'ScriptPubkeyConflict', 'UtxoNotLocked', 'RedemptionsUnavailable', 'InvalidBitcoinSyncHeight', 'BitcoinHeightNotConfirmed', 'MaxUtxosExceeded', 'InvalidBitcoinScript']
  },
  /**
   * Lookup241: ulx_primitives::bond::Vault<sp_core::crypto::AccountId32, Balance>
   **/
  UlxPrimitivesBondVault: {
    operatorAccountId: 'AccountId32',
    bitcoinArgons: 'UlxPrimitivesBondVaultArgons',
    securitizationPercent: 'u128',
    securitizedArgons: 'u128',
    miningArgons: 'UlxPrimitivesBondVaultArgons',
    isClosed: 'bool'
  },
  /**
   * Lookup242: ulx_primitives::bond::VaultArgons<Balance>
   **/
  UlxPrimitivesBondVaultArgons: {
    annualPercentRate: 'Compact<u128>',
    allocated: 'Compact<u128>',
    bonded: 'Compact<u128>'
  },
  /**
   * Lookup243: pallet_vaults::pallet::Error<T>
   **/
  PalletVaultsError: {
    _enum: ['BondNotFound', 'NoMoreVaultIds', 'NoMoreBondIds', 'MinimumBondAmountNotMet', 'ExpirationAtBlockOverflow', 'InsufficientFunds', 'InsufficientVaultFunds', 'InsufficientBitcoinsForMining', 'AccountBelowMinimumBalance', 'VaultClosed', 'InvalidVaultAmount', 'VaultReductionBelowAllocatedFunds', 'InvalidSecuritization', 'MaxVaultBitcoinPubkeys', 'MaxSecuritizationPercentExceeded', 'InvalidBondType', 'BitcoinUtxoNotFound', 'InsufficientSatoshisBonded', 'NoBitcoinPricesAvailable', 'InvalidBitcoinScript', 'ExpirationTooSoon', 'NoPermissions', 'HoldUnexpectedlyModified', 'UnrecoverableHold', 'VaultNotFound', 'FeeExceedsBondAmount', 'NoVaultBitcoinPubkeysAvailable']
  },
  /**
   * Lookup244: ulx_primitives::bond::Bond<sp_core::crypto::AccountId32, Balance, BlockNumber>
   **/
  UlxPrimitivesBond: {
    bondType: 'UlxPrimitivesBondBondType',
    vaultId: 'u32',
    utxoId: 'Option<u64>',
    bondedAccountId: 'AccountId32',
    totalFee: 'Compact<u128>',
    prepaidFee: 'Compact<u128>',
    amount: 'Compact<u128>',
    expiration: 'UlxPrimitivesBondBondExpiration'
  },
  /**
   * Lookup247: pallet_bond::pallet::UtxoState
   **/
  PalletBondUtxoState: {
    bondId: 'u64',
    satoshis: 'u64',
    vaultPubkeyHash: 'UlxPrimitivesBitcoinBitcoinPubkeyHash',
    ownerPubkeyHash: 'UlxPrimitivesBitcoinBitcoinPubkeyHash',
    vaultClaimHeight: 'u64',
    openClaimHeight: 'u64',
    registerBlock: 'u64',
    utxoScriptPubkey: 'UlxPrimitivesBitcoinBitcoinCosignScriptPubkey',
    isVerified: 'bool'
  },
  /**
   * Lookup250: pallet_bond::pallet::UtxoCosignRequest<Balance>
   **/
  PalletBondUtxoCosignRequest: {
    bitcoinNetworkFee: 'u64',
    cosignDueBlock: 'u64',
    toScriptPubkey: 'Bytes',
    redemptionPrice: 'u128'
  },
  /**
   * Lookup254: pallet_bond::pallet::Error<T>
   **/
  PalletBondError: {
    _enum: {
      BondNotFound: 'Null',
      NoMoreBondIds: 'Null',
      MinimumBondAmountNotMet: 'Null',
      ExpirationAtBlockOverflow: 'Null',
      InsufficientFunds: 'Null',
      InsufficientVaultFunds: 'Null',
      InsufficientBitcoinsForMining: 'Null',
      AccountWouldGoBelowMinimumBalance: 'Null',
      VaultClosed: 'Null',
      InvalidVaultAmount: 'Null',
      BondRedemptionNotLocked: 'Null',
      BitcoinUnlockInitiationDeadlinePassed: 'Null',
      BitcoinFeeTooHigh: 'Null',
      InvalidBondType: 'Null',
      BitcoinUtxoNotFound: 'Null',
      InsufficientSatoshisBonded: 'Null',
      NoBitcoinPricesAvailable: 'Null',
      InvalidBitcoinScript: 'Null',
      ExpirationTooSoon: 'Null',
      NoPermissions: 'Null',
      HoldUnexpectedlyModified: 'Null',
      UnrecoverableHold: 'Null',
      VaultNotFound: 'Null',
      FeeExceedsBondAmount: 'Null',
      GenericBondError: 'UlxPrimitivesBondBondError'
    }
  },
  /**
   * Lookup266: pallet_notaries::pallet::Error<T>
   **/
  PalletNotariesError: {
    _enum: ['ProposalNotFound', 'MaxNotariesExceeded', 'MaxProposalsPerBlockExceeded', 'NotAnActiveNotary', 'InvalidNotaryOperator', 'NoMoreNotaryIds', 'EffectiveTickTooSoon']
  },
  /**
   * Lookup270: ulx_primitives::notary::NotaryNotebookKeyDetails
   **/
  UlxPrimitivesNotaryNotaryNotebookKeyDetails: {
    notebookNumber: 'Compact<u32>',
    tick: 'Compact<u32>',
    blockVotesRoot: 'H256',
    secretHash: 'H256',
    parentSecret: 'Option<H256>'
  },
  /**
   * Lookup272: ulx_primitives::digests::NotebookDigest<ulx_notary_audit::error::VerifyError>
   **/
  UlxPrimitivesDigestsNotebookDigest: {
    notebooks: 'Vec<UlxPrimitivesDigestsNotebookDigestRecord>'
  },
  /**
   * Lookup274: ulx_primitives::digests::NotebookDigestRecord<ulx_notary_audit::error::VerifyError>
   **/
  UlxPrimitivesDigestsNotebookDigestRecord: {
    notaryId: 'Compact<u32>',
    notebookNumber: 'Compact<u32>',
    tick: 'Compact<u32>',
    auditFirstFailure: 'Option<UlxNotaryAuditErrorVerifyError>'
  },
  /**
   * Lookup277: pallet_notebook::pallet::Error<T>
   **/
  PalletNotebookError: {
    _enum: ['DuplicateNotebookNumber', 'MissingNotebookNumber', 'NotebookTickAlreadyUsed', 'InvalidNotebookSignature', 'InvalidSecretProvided', 'CouldNotDecodeNotebook', 'DuplicateNotebookDigest', 'MissingNotebookDigest', 'InvalidNotebookDigest', 'MultipleNotebookInherentsProvided', 'InternalError']
  },
  /**
   * Lookup278: pallet_chain_transfer::QueuedTransferOut<sp_core::crypto::AccountId32, Balance, BlockNumber>
   **/
  PalletChainTransferQueuedTransferOut: {
    accountId: 'AccountId32',
    amount: 'u128',
    expirationBlock: 'u32',
    notaryId: 'u32'
  },
  /**
   * Lookup284: frame_support::PalletId
   **/
  FrameSupportPalletId: '[u8;8]',
  /**
   * Lookup285: pallet_chain_transfer::pallet::Error<T>
   **/
  PalletChainTransferError: {
    _enum: ['MaxBlockTransfersExceeded', 'InsufficientFunds', 'InsufficientNotarizedFunds', 'InvalidOrDuplicatedLocalchainTransfer', 'NotebookIncludesExpiredLocalchainTransfer', 'InvalidNotaryUsedForTransfer']
  },
  /**
   * Lookup290: ulx_primitives::notary::NotaryNotebookVoteDigestDetails
   **/
  UlxPrimitivesNotaryNotaryNotebookVoteDigestDetails: {
    notaryId: 'Compact<u32>',
    notebookNumber: 'Compact<u32>',
    tick: 'Compact<u32>',
    blockVotesCount: 'Compact<u32>',
    blockVotingPower: 'Compact<u128>'
  },
  /**
   * Lookup292: ulx_primitives::digests::BlockVoteDigest
   **/
  UlxPrimitivesDigestsBlockVoteDigest: {
    votingPower: 'Compact<u128>',
    votesCount: 'Compact<u32>'
  },
  /**
   * Lookup296: pallet_block_seal_spec::pallet::Error<T>
   **/
  PalletBlockSealSpecError: {
    _enum: ['MaxNotebooksAtTickExceeded']
  },
  /**
   * Lookup299: pallet_data_domain::pallet::Error<T>
   **/
  PalletDataDomainError: {
    _enum: ['DomainNotRegistered', 'NotDomainOwner']
  },
  /**
   * Lookup300: pallet_price_index::pallet::Error<T>
   **/
  PalletPriceIndexError: {
    _enum: ['NotAuthorizedOperator', 'MissingValue', 'PricesTooOld']
  },
  /**
   * Lookup305: sp_core::crypto::KeyTypeId
   **/
  SpCoreCryptoKeyTypeId: '[u8;4]',
  /**
   * Lookup306: pallet_session::pallet::Error<T>
   **/
  PalletSessionError: {
    _enum: ['InvalidProof', 'NoAssociatedValidatorId', 'DuplicatedKey', 'NoKeys', 'NoAccount']
  },
  /**
   * Lookup307: ulx_primitives::providers::BlockSealerInfo<sp_core::crypto::AccountId32>
   **/
  UlxPrimitivesProvidersBlockSealerInfo: {
    minerRewardsAccount: 'AccountId32',
    blockVoteRewardsAccount: 'AccountId32'
  },
  /**
   * Lookup308: ulx_primitives::digests::ParentVotingKeyDigest
   **/
  UlxPrimitivesDigestsParentVotingKeyDigest: {
    parentVotingKey: 'Option<H256>'
  },
  /**
   * Lookup309: pallet_block_seal::pallet::Error<T>
   **/
  PalletBlockSealError: {
    _enum: ['InvalidVoteSealStrength', 'InvalidSubmitter', 'UnableToDecodeVoteAccount', 'UnregisteredBlockAuthor', 'InvalidBlockVoteProof', 'NoGrandparentVoteMinimum', 'DuplicateBlockSealProvided', 'InsufficientVotingPower', 'ParentVotingKeyNotFound', 'InvalidVoteGrandparentHash', 'IneligibleNotebookUsed', 'NoEligibleVotingRoot', 'UnregisteredDataDomain', 'InvalidDataDomainAccount', 'InvalidAuthoritySignature', 'CouldNotDecodeVote', 'MaxNotebooksAtTickExceeded', 'NoClosestMinerFoundForVote', 'BlockVoteInvalidSignature']
  },
  /**
   * Lookup311: pallet_block_rewards::pallet::Error<T>
   **/
  PalletBlockRewardsError: 'Null',
  /**
   * Lookup312: pallet_grandpa::StoredState<N>
   **/
  PalletGrandpaStoredState: {
    _enum: {
      Live: 'Null',
      PendingPause: {
        scheduledAt: 'u32',
        delay: 'u32',
      },
      Paused: 'Null',
      PendingResume: {
        scheduledAt: 'u32',
        delay: 'u32'
      }
    }
  },
  /**
   * Lookup313: pallet_grandpa::StoredPendingChange<N, Limit>
   **/
  PalletGrandpaStoredPendingChange: {
    scheduledAt: 'u32',
    delay: 'u32',
    nextAuthorities: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
    forced: 'Option<u32>'
  },
  /**
   * Lookup316: pallet_grandpa::pallet::Error<T>
   **/
  PalletGrandpaError: {
    _enum: ['PauseFailed', 'ResumeFailed', 'ChangePending', 'TooSoon', 'InvalidKeyOwnershipProof', 'InvalidEquivocationProof', 'DuplicateOffenceReport']
  },
  /**
   * Lookup317: sp_staking::offence::OffenceDetails<sp_core::crypto::AccountId32, Offender>
   **/
  SpStakingOffenceOffenceDetails: {
    offender: '(AccountId32,PalletMiningSlotMinerHistory)',
    reporters: 'Vec<AccountId32>'
  },
  /**
   * Lookup319: pallet_mining_slot::MinerHistory
   **/
  PalletMiningSlotMinerHistory: {
    authorityIndex: 'u32'
  },
  /**
   * Lookup324: pallet_mint::pallet::Error<T>
   **/
  PalletMintError: {
    _enum: ['TooManyPendingMints']
  },
  /**
   * Lookup326: pallet_balances::types::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup327: pallet_balances::types::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup330: pallet_balances::types::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup333: pallet_balances::types::IdAmount<ulx_node_runtime::RuntimeHoldReason, Balance>
   **/
  PalletBalancesIdAmountRuntimeHoldReason: {
    id: 'UlxNodeRuntimeRuntimeHoldReason',
    amount: 'u128'
  },
  /**
   * Lookup334: ulx_node_runtime::RuntimeHoldReason
   **/
  UlxNodeRuntimeRuntimeHoldReason: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      MiningSlot: 'PalletMiningSlotHoldReason',
      __Unused5: 'Null',
      Vaults: 'PalletVaultsHoldReason',
      Bonds: 'PalletBondHoldReason',
      __Unused8: 'Null',
      __Unused9: 'Null',
      __Unused10: 'Null',
      __Unused11: 'Null',
      __Unused12: 'Null',
      __Unused13: 'Null',
      __Unused14: 'Null',
      __Unused15: 'Null',
      __Unused16: 'Null',
      __Unused17: 'Null',
      BlockRewards: 'PalletBlockRewardsHoldReason'
    }
  },
  /**
   * Lookup335: pallet_mining_slot::pallet::HoldReason
   **/
  PalletMiningSlotHoldReason: {
    _enum: ['RegisterAsMiner']
  },
  /**
   * Lookup336: pallet_vaults::pallet::HoldReason
   **/
  PalletVaultsHoldReason: {
    _enum: ['EnterVault', 'BondFee']
  },
  /**
   * Lookup337: pallet_bond::pallet::HoldReason
   **/
  PalletBondHoldReason: {
    _enum: ['UnlockingBitcoin']
  },
  /**
   * Lookup338: pallet_block_rewards::pallet::HoldReason
   **/
  PalletBlockRewardsHoldReason: {
    _enum: ['MaturationPeriod']
  },
  /**
   * Lookup341: pallet_balances::types::IdAmount<ulx_node_runtime::RuntimeFreezeReason, Balance>
   **/
  PalletBalancesIdAmountRuntimeFreezeReason: {
    id: 'UlxNodeRuntimeRuntimeFreezeReason',
    amount: 'u128'
  },
  /**
   * Lookup342: ulx_node_runtime::RuntimeFreezeReason
   **/
  UlxNodeRuntimeRuntimeFreezeReason: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      __Unused4: 'Null',
      __Unused5: 'Null',
      __Unused6: 'Null',
      __Unused7: 'Null',
      __Unused8: 'Null',
      __Unused9: 'Null',
      __Unused10: 'Null',
      __Unused11: 'Null',
      __Unused12: 'Null',
      __Unused13: 'Null',
      __Unused14: 'Null',
      __Unused15: 'Null',
      __Unused16: 'Null',
      __Unused17: 'Null',
      BlockRewards: 'PalletBlockRewardsFreezeReason'
    }
  },
  /**
   * Lookup343: pallet_block_rewards::pallet::FreezeReason
   **/
  PalletBlockRewardsFreezeReason: {
    _enum: ['MaturationPeriod']
  },
  /**
   * Lookup345: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'Expendability', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves', 'TooManyHolds', 'TooManyFreezes', 'IssuanceDeactivated', 'DeltaZero']
  },
  /**
   * Lookup347: pallet_tx_pause::pallet::Error<T>
   **/
  PalletTxPauseError: {
    _enum: ['IsPaused', 'IsUnpaused', 'Unpausable', 'NotFound']
  },
  /**
   * Lookup348: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup349: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup352: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
   **/
  FrameSystemExtensionsCheckNonZeroSender: 'Null',
  /**
   * Lookup353: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup354: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup355: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup358: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup359: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup360: pallet_transaction_payment::ChargeTransactionPayment<T>
   **/
  PalletTransactionPaymentChargeTransactionPayment: 'Compact<u128>',
  /**
   * Lookup361: ulx_node_runtime::Runtime
   **/
  UlxNodeRuntimeRuntime: 'Null'
};
