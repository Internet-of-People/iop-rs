use super::*;

/// Strategies for the Hydra mainnet.
pub struct Mainnet;

impl Subtree for Mainnet {
    type Suite = Secp256k1;

    fn name(&self) -> &'static str {
        "HYD mainnet"
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
        b"\x64" // 100
    }
    fn p2sh_addr(&self) -> &'static [u8; 1] {
        unimplemented!()
    }
    fn wif(&self) -> &'static [u8; 1] {
        b"\x6f" // 111
    }
    fn bip32_xprv(&self) -> &'static [u8; 4] {
        b"\x4d\xe7\x4d\xef" // TODO
    }
    fn bip32_xpub(&self) -> &'static [u8; 4] {
        b"\xc1\x05\x5b\x57" // TODO
    }
    fn message_prefix(&self) -> &'static str {
        // TODO usually there is a binary length prefix, but so many btc forks screwed that
        // up (including IoP) that now many include it as part of this string. Wigy could
        // not find out whether ARK has a length prefix here and if yes, what is that.
        "HYD message:\n"
    }
    fn slip44(&self) -> i32 {
        0x485944 // 4741444
    }
    fn subtree(&self) -> &dyn Subtree<Suite = Secp256k1> {
        self
    }
}

/// Strategies for the Hydra devnet.
pub struct Devnet;

impl Subtree for Devnet {
    type Suite = Secp256k1;

    fn name(&self) -> &'static str {
        "HYD devnet"
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
        b"\x5a" // 90
    }
    fn p2sh_addr(&self) -> &'static [u8; 1] {
        unimplemented!()
    }
    fn wif(&self) -> &'static [u8; 1] {
        b"\x55" // 85
    }
    fn bip32_xprv(&self) -> &'static [u8; 4] {
        b"\x4d\xe7\x41\x47"
    }
    fn bip32_xpub(&self) -> &'static [u8; 4] {
        b"\xc1\x05\x4e\xaf"
    }
    fn message_prefix(&self) -> &'static str {
        "dHYD message:\n"
    }
    fn slip44(&self) -> i32 {
        1
    }
    fn subtree(&self) -> &dyn Subtree<Suite = Secp256k1> {
        self
    }
}

/// Strategies for the Hydra testnet.
pub struct Testnet;

impl Subtree for Testnet {
    type Suite = Secp256k1;

    fn name(&self) -> &'static str {
        "HYD testnet"
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
        b"\x80" // 128
    }
    fn p2sh_addr(&self) -> &'static [u8; 1] {
        unimplemented!()
    }
    fn wif(&self) -> &'static [u8; 1] {
        b"\xb3" // 179
    }
    fn bip32_xprv(&self) -> &'static [u8; 4] {
        b"\x4d\xe7\x57\x6d"
    }
    fn bip32_xpub(&self) -> &'static [u8; 4] {
        b"\xc1\x05\x66\x6a"
    }
    fn message_prefix(&self) -> &'static str {
        "HYD message:\n"
    }
    fn slip44(&self) -> i32 {
        1
    }
    fn subtree(&self) -> &dyn Subtree<Suite = Secp256k1> {
        self
    }
}
