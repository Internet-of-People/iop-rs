# Changelog

## 0.0.5

### Change
- Breaking change: Migrate from failure to anyhow

### Fix
- parsing Secp objects in multicipher
- Remove unused log dependency

## 0.0.4 (2020-09-07)

### Add
- `SecpKeyId` can be created from p2pkh addresses with Wasm
- `SecpPrivateKey` can be created from WIF with Wasm
- Expose network in `Bip32*Node` and `Bip44*` types on Wasm
- Added Changelog ;-)

## 0.0.3 (2020-09-03)

### Change:
- Breaking change: multicipher serde format changed for all self-describing serializers, saving some bytes

## 0.0.2 (2020-07-17)

Initial release
