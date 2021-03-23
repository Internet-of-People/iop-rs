use super::*;

/// Maybe an overly-zealous attempt to encode [BIP-0043](https://github.com/bitcoin/bips/blob/master/bip-0043.mediawiki) in source code.
pub struct Bip43Path;

impl Bip43Path {
    /// Return an object representing the purpose with the given index
    pub fn purpose(purpose: i32) -> Bip43Purpose {
        Bip43Purpose { purpose }
    }
}

/// A representation of a given purpose as a [BIP-0032](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) subtree
/// based on [BIP-0043](https://github.com/bitcoin/bips/blob/master/bip-0043.mediawiki)
pub struct Bip43Purpose {
    purpose: i32,
}

impl Bip43Purpose {
    /// An accessor to the purpose given when this object was created
    pub fn purpose(&self) -> i32 {
        self.purpose
    }

    /// Returns the bip32 path that belongs to the root of the subtree with this purpose
    pub fn bip32_path(&self) -> bip32::Path {
        bip32::Path::default().append(ChildIndex::Hardened(self.purpose))
    }
}
