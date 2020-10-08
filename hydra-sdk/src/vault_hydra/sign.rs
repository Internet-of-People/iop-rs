use super::*;

pub trait HydraSigner {
    fn sign_hydra_transaction(&self, tx: &mut TransactionData) -> Result<()>;
}

impl HydraSigner for SecpPrivateKey {
    fn sign_hydra_transaction(&self, tx: &mut TransactionData) -> Result<()> {
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
