use super::*;

/// Strategies for the ARK mainnet.
pub struct Mainnet;

impl Subtree for Mainnet {
    type Suite = Secp256k1;

    fn name(&self) -> &'static str {
        "ARK mainnet"
    }
    fn master(&self, seed: &Seed) -> SecpExtPrivateKey {
        Secp256k1::master(seed)
    }
    fn key_id(&self, pk: &SecpPublicKey) -> SecpKeyId {
        pk.ark_key_id()
    }
}

impl Network for Mainnet {
    fn p2pkh_addr(&self) -> &'static [u8; 1] {
        b"\x17" // 23
    }
    /// There is no BIP-0016 on ARK, so there is no such prefix either
    fn p2sh_addr(&self) -> &'static [u8; 1] {
        unimplemented!()
    }
    fn wif(&self) -> &'static [u8; 1] {
        b"\xaa" // 170
    }
    fn bip32_xprv(&self) -> &'static [u8; 4] {
        b"\x46\x08\x95\x20"
    }
    fn bip32_xpub(&self) -> &'static [u8; 4] {
        b"\x46\x09\x06\x00"
    }
    fn message_prefix(&self) -> &'static str {
        // TODO usually there is a binary length prefix, but so many btc forks screwed that
        // up (including IoP) that now many include it as part of this string. Wigy could
        // not find out whether ARK has a length prefix here and if yes, what is that.
        "ARK message:\n"
    }
    fn slip44(&self) -> i32 {
        0x6f // 111
    }
    fn subtree(&self) -> &dyn Subtree<Suite = Secp256k1> {
        self
    }
}

/// Strategies for the ARK devnet.
pub struct Devnet;

impl Subtree for Devnet {
    type Suite = Secp256k1;

    fn name(&self) -> &'static str {
        "ARK devnet"
    }
    fn master(&self, seed: &Seed) -> SecpExtPrivateKey {
        Secp256k1::master(seed)
    }
    fn key_id(&self, pk: &SecpPublicKey) -> SecpKeyId {
        pk.ark_key_id()
    }
}

impl Network for Devnet {
    fn p2pkh_addr(&self) -> &'static [u8; 1] {
        b"\x1e" // 30
    }
    fn p2sh_addr(&self) -> &'static [u8; 1] {
        unimplemented!()
    }
    fn wif(&self) -> &'static [u8; 1] {
        b"\xaa" // 170
    }
    fn bip32_xprv(&self) -> &'static [u8; 4] {
        b"\x46\x08\x95\x20"
    }
    fn bip32_xpub(&self) -> &'static [u8; 4] {
        b"\x46\x09\x06\x00"
    }
    fn message_prefix(&self) -> &'static str {
        "DARK message:\n"
    }
    fn slip44(&self) -> i32 {
        1
    }
    fn subtree(&self) -> &dyn Subtree<Suite = Secp256k1> {
        self
    }
}

/// Strategies for the ARK testnet.
pub struct Testnet;

impl Subtree for Testnet {
    type Suite = Secp256k1;

    fn name(&self) -> &'static str {
        "ARK testnet"
    }
    fn master(&self, seed: &Seed) -> SecpExtPrivateKey {
        Secp256k1::master(seed)
    }
    fn key_id(&self, pk: &SecpPublicKey) -> SecpKeyId {
        pk.ark_key_id()
    }
}

impl Network for Testnet {
    fn p2pkh_addr(&self) -> &'static [u8; 1] {
        b"\x17" // 23
    }
    fn p2sh_addr(&self) -> &'static [u8; 1] {
        unimplemented!()
    }
    fn wif(&self) -> &'static [u8; 1] {
        b"\xba" // 186
    }
    fn bip32_xprv(&self) -> &'static [u8; 4] {
        b"\x70\x61\x59\x56"
    }
    fn bip32_xpub(&self) -> &'static [u8; 4] {
        b"\x70\x61\x70\x39"
    }
    fn message_prefix(&self) -> &'static str {
        "DARK message:\n"
    }
    fn slip44(&self) -> i32 {
        1
    }
    fn subtree(&self) -> &dyn Subtree<Suite = Secp256k1> {
        self
    }
}
