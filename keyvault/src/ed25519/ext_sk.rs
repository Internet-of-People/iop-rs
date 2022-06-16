use super::*;

/// Implementation of Ed25519::ExtendedPrivateKey
#[derive(Clone)]
pub struct EdExtPrivateKey {
    chain_code: ChainCode,
    sk: EdPrivateKey,
}

impl EdExtPrivateKey {
    /// Borrows the chain code of the extended private key
    pub fn chain_code(&self) -> &ChainCode {
        &self.chain_code
    }

    pub(crate) fn cook_new<F: Fn(&mut HmacSha512)>(salt: &[u8], recipe: F) -> Self {
        // This unwrap would only panic if the digest algorithm had some inconsistent
        // generic parameters, but the SHA512 we use is consistent with itself
        let mut hasher = <HmacSha512 as KeyInit>::new_from_slice(salt).unwrap();

        recipe(&mut hasher);

        let hash_bytes = hasher.finalize().into_bytes();

        let sk_bytes = &hash_bytes[..PRIVATE_KEY_SIZE];
        let cc_bytes = &hash_bytes[PRIVATE_KEY_SIZE..];

        let chain_code = ChainCode::from_bytes(cc_bytes).unwrap();
        let sk = EdPrivateKey::from_bytes(sk_bytes).unwrap();

        Self { chain_code, sk }
    }
}

impl ExtendedPrivateKey<Ed25519> for EdExtPrivateKey {
    fn derive_normal_child(&self, _idx: i32) -> Result<EdExtPrivateKey> {
        bail!("Normal derivation of Ed25519 is invalid based on SLIP-0010.")
    }
    /// There is a potential [vulnerability](https://forum.web3.foundation/t/key-recovery-attack-on-bip32-ed25519/44) in
    /// that might affect all SLIP-0010 compatible Ed25519 wallets. We should never assume that there is only 1
    /// public key that can verify a given signature. Actually, there are 8 public keys.
    fn derive_hardened_child(&self, idx: i32) -> Result<EdExtPrivateKey> {
        ensure!(idx >= 0, "Derivation index cannot be negative");
        let idx = idx as u32;

        let xprv = EdExtPrivateKey::cook_new(&self.chain_code.to_bytes(), |hasher| {
            hasher.update(&[0x00u8]);
            hasher.update(&self.sk.to_bytes());
            hasher.update(&(0x8000_0000u32 + idx).to_be_bytes());
        });

        Ok(xprv)
    }
    fn neuter(&self) -> EdPublicKey {
        self.sk.public_key()
    }
    fn private_key(&self) -> EdPrivateKey {
        self.sk.clone()
    }
}
