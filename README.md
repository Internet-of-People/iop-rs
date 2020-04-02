# Keyvault

Keyvault is a general purpose hierarchical deterministic (HD) generator for asymmetric keys.
It is based on the same concepts as a Bitcoin HD-wallet and is built on the same specifications like
[HD wallets of Bip32](https://en.bitcoin.it/wiki/BIP_0032),
[Mnemonic word lists of Bip39](https://en.bitcoin.it/wiki/BIP_0039) and
[Purpose fields of Bip43](https://en.bitcoin.it/wiki/BIP_0043).

Though keyvault is capable of generating wallet addresses as defined in
[Multi-Account cryptocurrency wallets of Bip44](https://en.bitcoin.it/wiki/BIP_0044),
it is not only an address generator for multiple cryptocurrencies.
Keyvault can also derive all the keys you might need in other software stacks
and aims to be your all-in-one Swiss Army knife identity manager.

Keyvault can
- use the same seed to derive keys with multiple cipher suites, currently `ed25519` and `secp256k1`
- use any purpose field and account hierarchy, not only Bip43 and Bip44
- handle several purposes (i.e. attached subhierarchies) at the same time
- be used from other platforms via its WebAssembly bindings

Keyvault was originally created as part of the
[Mercury communication protocol](https://github.com/Internet-of-People/mercury-rust)
but being a general-purpose tool it was reused in other components as well,
hence it was separated into its own repository in the end.
