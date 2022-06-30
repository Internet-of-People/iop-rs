# Changelog

## 0.0.16 (2022-06-30)

### Added

- New WASM bindings:
  - `allNetworkNames()` lists all network names that are accepted as a parameter in some methods.
  - `Bip39.shortEntropy()`, `Bip39.shortPhrase()` improves compatibility with old wallets like Coinomi.
  - `wrapWithNonce()` helps adding extra entropy to a JSON document, where selective masking will be used.
  - `MorpheusPrivate.path` and `MorpheusPrivateKind.path` returns BIP32 derivation paths to allow compatibility with other wallets.
  - `MorpheusOperationSigner.signWithId()` makes it easier to sign with a key if only its identifier is known to the caller.
  - `JwtBuilder.timeToLive` property helps to override the default 5s expiration of tokens.
  - `new MorpheusSignableOperation(json)` and `new MorpheusSignedOperation(json)` mirrors `MorpheusSignableOperation.toJSON()` and
    `MorpheusSignedOperation.toJSON()`, so it is easier to build complex SSI transactions.
- Documented all WASM methods in the SDK, except those related to Coeus for now.

### Changed

- Vault freshly created from a random seed is dirty (unsaved).
- All crates are now using Rust edition 2021.

## 0.0.15 (2021-11-09)

### Fixed

- `JsSubtreePolicies` methods should not take ownership of self

### Added

- Exposed all 4 DID kinds (persona, device, group, resource) through WASM, FFI and vault file format.
- Exposed SignedJson serialization format through WASM and FFI.
- Made the native Rust SDK more usable by re-exporting symbols through it.
- Morpheus node stores transaction ID of PoE SSI operations, so it can be queried in IBeforeProofHistory.
- scripts/publish.sh helps publishing all crates in the correct order.

## 0.0.14 (2021-07-21)

### Fixed

- DID documents generated were missing the `index` property in the objects in the `keys` list.

### Changed

- Separated `sign` and `signWithKey` methods on `MorpheusOperationSigner` both in WASM and FFI

## 0.0.13 (2021-07-09)

### Added

- You can now set the vendor field (aka. smart bridge field) and set a manual fee on Hydra core transactions using the 2 new optional
  arguments TypeScript SDK HydraTxBuilder factory methods got.
- Added some missing bindings to WASM and FFI, `SecpKeyId.toAddress` being the most important one.
- Multicipher objects (`MPrivateKey`, `MPublicKey` and `MKeyId`) can be safely downcasted in Rust to secp256p1 and ed25519 cipher
  objects. Not supported in WASM and FFI yet.

### Changed

- Separated node-wasm and sdk-wasm, so hydra-core plugins can have code that does not bloat the SDK
- Merged morpheus-rust and keyvault-rust repositories as iop-rs

## 0.0.12-hotfix1 (2021-05-06)

### Added

- You can now set the vendor field (aka. smart bridge field) and set a manual fee on Hydra core transactions using the 2 new optional
  arguments TypeScript SDK HydraTxBuilder factory methods got.

## 0.0.12 (2021-03-17)

### Added

- Crate `iop-sdk` now exports important types and is generally usable as an early draft in clients. Note that it currently does not
  follow any conventions of the Dart and Typescript SDKs but exposes some internal implementation details.
- Hydra and Morpheus vault plugins are now thread-safe and thus easily usable in async environments as well.

### Changed

- BREAKING: renamed Hydra and Morpheus `vault::Plugin` function `rewind()` to `init()` for clarity. Naming changes also affect FFI
  and WASM interfaces.
- BREAKING: suffixed functions that mutate Hydra and Morpheus vaults (by generating keys into them) with `_mut`. This does not affect
  FFI and WASM though.
- updated dependencies

### Fixed

- cleaned up dependencies
- removed legacy SDK codebase

## 0.0.11 (2020-11-24)

### Added

- WASM binding for CoeusAsset::fee
- scripts/set-version.sh helps in releasing new versions

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

We're heavily refactoring our crate structure while implementing Coeus, our decentralized naming system as our second Layer-2
component. Our end goal is to make fine-grained crates for reducing dependency footprint for integrators by separating client/server
sides and making Morpheus and Coeus optional plugins.

### Added

- Coeus: generic decentralized naming system built on top of a distributed ledger. Allows binding arbitrary data to names, allowing
  naming schemas, wallets, DIDs, devices, etc. This helps public figures and services to be more accessible and transparent. For more
  details, see the [IOP Developer Portal](https://developer.iop.technology/dns).

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
