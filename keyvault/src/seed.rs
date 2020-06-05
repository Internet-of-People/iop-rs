use ::bip39::{Mnemonic, Seed as Bip39Seed};
use failure::{bail, Fallible};
use serde::{Deserialize, Serialize};

// TODO consider de/serialize attributes here, currently only needed for demo
/// The seed used for BIP32 derivations. A seed cannot be turned back into a phrase, because there is salted hashing involed
/// in creating it from the BIP39 mnemonic phrase.
#[derive(Debug, Deserialize, Serialize)]
pub struct Seed {
    // Note that you cannot restore the mnemonic from the seed, because it goes through a one-way function
    bytes: Vec<u8>,
}

// TODO this interface should enable selecting language and listing words by prefix
impl Seed {
    /// Number of bits in entropy generated from the bip39 mnemonic
    pub const BITS: usize = 512;

    pub const DEMO_PHRASE: &'static str = "include pear escape sail spy orange cute despair witness trouble sleep torch wire burst unable brass expose fiction drift clock duck oxygen aerobic already";

    /// Legacy password used in the 0.0.1 version of the crate. Since 0.0.2 the crate always requires a password, which should be "" by default when
    /// the user does not provide one.
    pub const PASSWORD: &'static str = "morpheus";

    /// Creates seed from a 24-word BIP39 mnemonic
    ///
    /// # Example
    ///
    /// ```
    /// # use iop_keyvault::{Bip39, Seed};
    /// let phrase = "plastic attend shadow hill conduct whip staff shoe achieve repair museum improve below inform youth alpha above limb paddle derive spoil offer hospital advance";
    /// let seed_expected = "86f07ba8b38f3de2080912569a07b21ca4ae2275bc305a14ff928c7dc5407f32a1a3a26d4e2c4d9d5e434209c1db3578d94402cf313f3546344d0e4661c9f8d9";
    /// let seed = Bip39::new().phrase(phrase).unwrap().password("morpheus");
    /// assert_eq!(hex::encode(seed.as_bytes()), seed_expected);
    /// ```
    pub(crate) fn from_bip39(mnemonic: &Mnemonic, password: &str) -> Self {
        let bytes = Bip39Seed::new(mnemonic, password).as_bytes().to_owned();
        Self { bytes }
    }

    /// Creates seed from a raw 512-bit binary seed
    ///
    /// # Example
    ///
    /// ```
    /// # use iop_keyvault::Seed;
    /// let bytes = "86f07ba8b38f3de2080912569a07b21ca4ae2275bc305a14ff928c7dc5407f32a1a3a26d4e2c4d9d5e434209c1db3578d94402cf313f3546344d0e4661c9f8d9";
    /// let seed_res = Seed::from_bytes(hex::decode(bytes).unwrap().as_slice());
    /// assert!(seed_res.is_ok());
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Fallible<Self> {
        if bytes.len() * 8 != Self::BITS {
            bail!("Only {}-bit seeds are supported", Self::BITS)
        }
        let bytes = bytes.to_vec();
        Ok(Self { bytes })
    }

    // TODO this should be changed to something like Entropy::unlock(password) -> Seed
    /// Returns the bytes of the seed
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}
