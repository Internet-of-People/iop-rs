# Changelog

## 0.0.10 (2020-11-20)

### Added

- FFI bindings for creating Coeus transactions
- Automated build of sdk-wasm on github

## 0.0.9 (2020-11-13)

### Fixed

- Enforcing canonical Json introduced in 0.0.7 would have broken layer-1 consensus in unexpected ways.
  Fixed this regression before getting it on devnet or mainnet.

## 0.0.8 (2020-11-12)

- Rereleased, because the sdk-wasm package was released with some changes missing on npmjs.com

## 0.0.7 (2020-11-12)

### Fixed

- Implemented mostly size-based fees on Coeus transactions
- Enforce canonical Json format and limit sizes on CoeusAsset and MorpheusAsset deserialization from bytes

## 0.0.6 (2020-11-06)

We're heavily refactoring our crate structure while implementing Coeus, our decentralized naming system as our second Layer-2 component. Our end goal is to make fine-grained crates for reducing dependency footprint for integrators by separating client/server sides and making Morpheus and Coeus optional plugins.

### Added

- Coeus: generic decentralized naming system built on top of a distributed ledger. Allows binding arbitrary data to names, allowing naming schemas, wallets, DIDs, devices, etc. This helps public figures and services to be more accessible and transparent. For more details, see the [IOP Developer Portal](https://developer.iop.global/dns).

### Changed

- BREAKING: changed crate hierarchy
  - coeus-core: a temporary crate with proto, sdk and node parts are not separated yet
  - coeus-core-wasm: WebAssembly bindings for Coeus, sdk and node parts are not separated yet 
  - hydra-proto: extracted hydra-dependent blockchain features (e.g. Morpheus and Coeus transactions) into their own crate
  - hydra-sdk: Hydra BIP32 subtree plugin for the Vault
  - morpheus-sdk-legacy: the old morpheus-sdk is now considered to be legacy code and was thus renamed
  - morpheus-sdk: Morpheus BIP32 subtree plugin for the Vault
  - sdk: IOP aggregate crate for clients (stub)
  - vault: extracted keyvault state serialization into its own crate

### Fixed

- serde structure of Morpheus transactions, parsing them will not collide with core transactions
- core transactions without an asset will not serialize an `"asset": "none"`JSON field  
- cleaned up some basic API types like BlockHeight and BlockCount
- fixed contact email in crate descriptions

## 0.0.5 (2020-09-21)

### Changed

- Extracted and released `json-digest` as separate crate

## 0.0.4 (2020-09-07)

### Added

- Create and verify JWT tokens with multicipher
- Hydra transaction builder, exposed on FFI and Wasm
  - transfer
  - vote
  - unvote
- `HydraSigner` available on Wasm
- xprv on `HydraPrivate`

## 0.0.2 (2020-07-28)

Initial release