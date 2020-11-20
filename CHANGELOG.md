# Changelog

## 0.0.10 (2020-11-20)

- Just rereleased it to be in synch with our other crates

## 0.0.9 (2020-11-13)

- Just rereleased it to be in synch with our other crates

## 0.0.8 (2020-11-12)

- Just rereleased it to be in synch with our other crates

## 0.0.7 (2020-11-12)

- Just rereleased it to be in synch with our other crates

## 0.0.6 (2020-11-06)

### Add

- Implement Hash, PartialOrd and Ord on MPublicKey

### Fix

- Pin orion to 0.15.3 to fix wasm32-* target
- JsBip32 naming was off in Wasm
- Contact email address was fixed in Cargo.toml

## 0.0.5 (2020-09-21)

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

### Change

- Breaking change: multicipher serde format changed for all self-describing serializers, saving some bytes

## 0.0.2 (2020-07-17)

Initial release
