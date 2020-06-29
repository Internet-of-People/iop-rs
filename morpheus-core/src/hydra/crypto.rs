use failure::{ensure, Fallible};

use crate::hydra::transaction::TransactionData;
use iop_keyvault::{secp256k1::*, PrivateKey as _};

pub trait HydraSigner {
    fn sign_hydra_transaction(&self, tx: &mut TransactionData) -> Fallible<()>;
}

impl HydraSigner for SecpPrivateKey {
    fn sign_hydra_transaction(&self, tx: &mut TransactionData) -> Fallible<()> {
        ensure!(
            tx.sender_public_key == self.public_key().to_string(),
            "Attempt to sign transaction with key different from tx.sender_public_key"
        );
        let bytes = tx.to_bytes(true, true, false)?;
        let signature = self.sign(&bytes);
        tx.signature = Some(hex::encode(signature.to_der()));
        tx.id = Some(tx.get_id()?);
        Ok(())
    }
}

// TODO consider this "dead code" and remove it if not needed anywhere else
// pub fn hex_to_hydra_priv_key(private_key_hex: &str) -> Fallible<SecpPrivateKey> {
//     let key_bytes = hex::decode(private_key_hex)?;
//     Ok(SecpPrivateKey::from_bytes(key_bytes)?)
// }
//
// pub fn hex_to_hydra_pub_key(public_key_hex: &str) -> Fallible<SecpPublicKey> {
//     let key_bytes = hex::decode(public_key_hex)?;
//     Ok(SecpPublicKey::from_bytes(key_bytes)?)
// }
//
// pub fn verify_ecdsa_signature(pk: &SecpPublicKey, data: &[u8], sig_der: &str) -> Fallible<bool> {
//     let sig_bytes = hex::decode(sig_der)?;
//     let sig = SecpSignature::from_der(&sig_bytes)?;
//     Ok(pk.verify(data, &sig))
// }
//
// pub fn hydra_pub_key_to_hex(public_key: &SecpPublicKey) -> String {
//     hex::encode(public_key.to_bytes())
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn private_key_from_passphrase() {
//         let private_key = SecpPrivateKey::from_ark_passphrase("this is a top secret passphrase");
//         assert_eq!(
//             hex::encode(private_key.unwrap().to_bytes()),
//             "d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712"
//         );
//     }
//
//     #[test]
//     fn private_key_from_hex() {
//         let private_key = hex_to_hydra_priv_key(
//             "d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712",
//         );
//         assert!(private_key.is_ok());
//         assert_eq!(
//             hex::encode(private_key.unwrap().to_bytes()),
//             "d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712"
//         );
//     }
//
//     #[test]
//     fn public_key_from_passphrase() {
//         let public_key = SecpPrivateKey::from_ark_passphrase("this is a top secret passphrase")
//             .unwrap()
//             .public_key();
//         assert_eq!(
//             hex::encode(public_key.to_bytes()),
//             "034151a3ec46b5670a682b0a63394f863587d1bc97483b1b6c70eb58e7f0aed192"
//         );
//     }
//
//     #[test]
//     fn public_key_from_hex() {
//         let public_key = hex_to_hydra_pub_key(
//             "034151a3ec46b5670a682b0a63394f863587d1bc97483b1b6c70eb58e7f0aed192",
//         );
//         assert_eq!(
//             hex::encode(public_key.unwrap().to_bytes()),
//             "034151a3ec46b5670a682b0a63394f863587d1bc97483b1b6c70eb58e7f0aed192"
//         );
//     }
// }
