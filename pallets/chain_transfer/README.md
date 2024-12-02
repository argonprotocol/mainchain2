This Pallet allows transfers to both Localchain and EVM-based chains via hyperbridge.

## Localchain

You can exchange assets back and forth with the Localchain using the `send_to_localchain` function in this pallet, and a
corresponding localchain transaction to send back to mainchain.

## Hyperbridge

Hyperbridge is a cross-chain bridge that allows for the transfer of assets between different blockchains. It is a
decentralized, trustless, and secure solution that enables users to move assets between different chains without the
need for a centralized intermediary. More here: https://docs.hyperbridge.network/

A token transfer ui is available here: https://app.hyperbridge.network/

### Relayer

There's a hyperbridge script that will all you to start a local relayer to retrieve the initial genesis state:
`hyperbridge/get_hyperbridge_state.sh` - this is what you register in argon (sudo -> ismp -> createConsensusClient)
`hyperbridge/get_argon_state.sh` - this is what you send to hyperbridge to register the chain

### Configuration

Configuration details relevant to transactions are below:

#### Testnet

Contract addresses for the hyperbridge contracts: https://docs.hyperbridge.network/developers/evm/contracts/testnet

Token Gateway is 0xFcDa26cA021d5535C3059547390E6cCd8De7acA6

##### EVM

- Ethereum => Evm 11155111,
- Optimism => Evm 11155420,
- Arbitrum => Evm 421614,
- Base => Evm 84532,
- Polygon => Evm 80001,
- BinanceSmartChain => Evm 97,
- Gnosis => Evm 10200,

##### Substrate

- Hyperbridge Paseo => Polkadot(4009),

#### Mainnet

Contract addresses for the hyperbridge contracts:https://docs.hyperbridge.network/developers/evm/contracts/mainnet

Token Gateway is 0xFd413e3AFe560182C4471F4d143A96d3e259B6dE

##### EVM

- Ethereum => Evm 1,
- Arbitrum => Evm 42161,
- Optimism => Evm 10,
- Base => Evm 8453,
- BinanceSmartChain => Evm 56,
- Gnosis => Evm 100,

##### Substrate

- Hyperbridge Polkadot => Polkadot(3367),
- Hyperbridge Kusama => Kusama(3340),

### Transferring Manually from Ethereum

You have to get Eth (need to have 1 link min in a mainnet account to get testnet eth).

1. Drip tokens from the FeeToken
   contract (https://sepolia.etherscan.io/address/0x1794aB22388303ce9Cb798bE966eeEBeFe59C3a3#writeContract#F1)
    - Token Address is the FeeToken: 0xA801da100bF16D07F668F4A49E1f71fc54D05177
1. Approve TokenGateway to Spend Dripped
   tokens (https://sepolia.etherscan.io/address/0xA801da100bF16D07F668F4A49E1f71fc54D05177#writeContract#F1)
    - Spender: 0xFcDa26cA021d5535C3059547390E6cCd8De7acA6
    - Amount: 1000e18
1. On Token Gateway, call
   `teleport` (https://sepolia.etherscan.io/address/0xfcda26ca021d5535c3059547390e6ccd8de7aca6#writeContract#F8) with
   the following parameters:
    - payableAmount: 0
    - teleportParams:
        - amount: Amount with 18 decimals (1 argon is 1e18)
        - relayerFee: 0
        - assetId: 0x752223BEDAB02BD0B8CB986805C5DBFF2031AA233CFCC0E49110B551FE6D3884 (keccak256 of the asset symbol
          ARGON/ARGOWN)
        - redeem: false
        - to: 0x(32 byte public key of user) -> Can be found on Polkadotjs -> Developer -> Utilities -> Address To
          Convert -> hex public key
            - eg, 0x507478d8aa5d510c89e0c05b3cf2d37aafad9ca6447e8be1050608bef4242a11
        - dest: 0x5355425354524154452d6172676e (hex encoding of the chain name SUBSTRATE-argn)
        - timeout: 0 for no timeout, else in seconds
        - nativeCost: 0
        - data: 0x

### Example Transactions

_Token Transfer_
https://sepolia.etherscan.io/tx/0x928934b04556706a6e60ea0c6596249787602a0dd63ab810ef49ff415c8a9411#eventlog

_Asset Registration_
https://sepolia.etherscan.io/tx/0x9451786d8c9c62ae1b82c747e1366d924f44d0d2b5ac3978c11a2d1604d77c4e#eventlog