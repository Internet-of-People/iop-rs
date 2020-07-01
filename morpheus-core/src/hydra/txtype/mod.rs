pub mod hyd_core;
pub mod morpheus;

use super::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TransactionType {
    Core(hyd_core::TransactionType),
    Morpheus(morpheus::TransactionType),
}

impl TransactionType {
    pub fn type_group(self) -> u32 {
        match self {
            Self::Core(_) => hyd_core::TransactionType::TYPE_GROUP,
            Self::Morpheus(_) => morpheus::TransactionType::TYPE_GROUP,
        }
    }

    pub fn into_u16(self) -> u16 {
        match self {
            Self::Core(core_type) => core_type as u16,
            Self::Morpheus(morpheus_type) => morpheus_type as u16,
        }
    }
}

// TODO consider using a better programming construction than this Default here
impl Default for TransactionType {
    fn default() -> Self {
        Self::Core(hyd_core::TransactionType::Transfer)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Asset {
    Core(hyd_core::Asset),
    Morpheus(morpheus::Asset),
}

pub trait Aip29Transaction {
    fn fee(&self) -> u64;
    fn to_data(&self) -> TransactionData;
}

#[derive(Clone, Debug)]
pub struct CommonTransactionFields {
    pub network: &'static dyn Network<Suite = Secp256k1>,
    pub sender_public_key: String, // TODO should be SecpPublicKey, but conflicts with being Default
    pub nonce: u64, // TODO consider allowing Option<u64> here and autofilling nonce when None
    pub amount: u64,
    pub manual_fee: Option<u64>,
    pub vendor_field: Option<String>,
}

impl Default for CommonTransactionFields {
    fn default() -> Self {
        Self {
            network: &hyd::Testnet,
            sender_public_key: Default::default(),
            nonce: Default::default(),
            amount: Default::default(),
            manual_fee: Default::default(),
            vendor_field: Default::default(),
        }
    }
}

impl CommonTransactionFields {
    pub fn calculate_fee(&self, tx: &dyn Aip29Transaction) -> u64 {
        self.manual_fee.unwrap_or_else(|| tx.fee())
    }

    fn to_data(&self) -> TransactionData {
        let mut tx_data = TransactionData::default();
        tx_data.network = Some(self.network.p2pkh_addr()[0]);
        tx_data.version = Some(2);

        tx_data.sender_public_key = self.sender_public_key.to_owned();
        tx_data.nonce = Some(self.nonce);
        tx_data.amount = self.amount;
        tx_data.vendor_field = self.vendor_field.to_owned();

        tx_data
    }
}

#[cfg(test)]
mod test {
    use failure::Fallible;

    use crate::crypto::{
        hd::{hydra, morpheus as hd_morpheus, Vault},
        sign::PrivateKeySigner,
    };
    use crate::data::{auth::Authentication, did::Did, diddoc::Right};
    use crate::hydra::txtype::morpheus::OperationAttempt;
    use crate::hydra::{
        crypto::HydraSigner,
        transaction::{TransactionData, TxBatch},
        txtype::{hyd_core, morpheus, Aip29Transaction, CommonTransactionFields},
    };
    use iop_keyvault::{multicipher::MKeyId, secp256k1::hyd, PublicKey, Seed};

    #[test]
    fn builder() -> Fallible<()> {
        let unlock_password = "test";
        let mut vault = Vault::create(None, Seed::DEMO_PHRASE, "", unlock_password)?;

        let hyd_params = hydra::Parameters::new(&hyd::Testnet, 0);
        hydra::Plugin::rewind(&mut vault, unlock_password, &hyd_params)?;
        let hydra_plugin = hydra::Plugin::get(&vault, &hyd_params)?;
        let hyd_bip44_pubkey0 = hydra_plugin.public()?.key(0)?;
        //let hyd_bip44_privkey0 = hydra_plugin.private(&unlock_password)?.key(0)?;
        //let hyd_bip44_pubkey0 = hyd_bip44_privkey0.neuter();
        let hyd_wallet_pubkey0 = hyd_bip44_pubkey0.to_public_key();

        println!("Hydra Wallet 0 Public Key: {}", hyd_wallet_pubkey0.to_string());
        println!("Hydra Wallet 0 Address: {}", hyd_bip44_pubkey0.to_p2pkh_addr());
        println!(
            "Hydra Wallet 0 Ark KeyId: {}",
            MKeyId::from(hyd_wallet_pubkey0.ark_key_id()).to_string()
        );

        hd_morpheus::Plugin::rewind(&mut vault, &unlock_password)?;
        let morpheus_plugin = hd_morpheus::Plugin::get(&vault)?;
        let mph_private = morpheus_plugin.private(&unlock_password)?;
        let mph_bip32_privkey0 = mph_private.personas()?.key(0)?;
        let mph_bip32_pubkey0 = mph_bip32_privkey0.neuter();
        let mph_persona_pubkey0 = mph_bip32_pubkey0.public_key();
        let mph_persona_did0 = Did::new(mph_persona_pubkey0.key_id());

        println!("Morpheus Persona 0 Public Key: {}", mph_persona_pubkey0.to_string());
        println!("Morpheus Persona 0 Did: {}", mph_persona_did0.to_string());

        let common_fields = CommonTransactionFields {
            sender_public_key: hyd_wallet_pubkey0.to_string(),
            nonce: 14,
            ..Default::default()
        };

        let transfer_common = CommonTransactionFields {
            amount: 3_141_593,
            manual_fee: Some(1_000_000),
            ..common_fields.clone()
        };
        let transfer_tx = hyd_core::TransferTransaction::new(
            transfer_common,
            "tjseecxRmob5qBS2T3qc8frXDKz3YUGB8J".to_owned(),
        );
        let mut transfer_tx_data: TransactionData = transfer_tx.to_data();
        let hydra_priv = hydra_plugin.private(&unlock_password)?;
        let hydra_signer = hydra_priv.key_by_pk(&hyd_wallet_pubkey0)?.to_private_key();
        hydra_signer.sign_hydra_transaction(&mut transfer_tx_data)?;
        show_tx_json("Transfer transaction:", vec![transfer_tx_data])?;

        let reg_proof_attempt = morpheus::OperationAttempt::RegisterBeforeProof {
            content_id: "<<placeholder of your 3rd favourite wisdom>>".to_owned(),
        };
        let reg_proof_tx =
            morpheus::Transaction::new(common_fields.clone(), vec![reg_proof_attempt]);
        let mut reg_proof_tx_data: TransactionData = reg_proof_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut reg_proof_tx_data)?;
        show_tx_json("Register-before-proof transaction:", vec![reg_proof_tx_data])?;

        // TODO consider implementing other Core transaction types as well
        // let recipients = vec![hyd_core::PaymentsItem { amount: 1, recipient_id: "tBlah" }];
        // let multitransfer_tx = hyd_core::MultiTransferTransaction::new(core_tx_fields, recipients);

        // TODO consider implementing second_sig
        // TODO consider implementing multisig

        let auth: Authentication = "iez25N5WZ1Q6TQpgpyYgiu9gTX".parse()?;
        let last_tx_id =
            Some("88df06a4faa3401c35c82177dcbd6a27e56acde4155ff11adfe4fdbd7509ec65".to_owned());
        let addkey_attempt = morpheus::SignableOperationAttempt {
            did: mph_persona_did0.to_string(),
            last_tx_id: last_tx_id.clone(),
            operation: morpheus::SignableOperationDetails::AddKey {
                auth: auth.clone(),
                expires_at_height: None,
            },
        };
        let addright_attempt = morpheus::SignableOperationAttempt {
            did: mph_persona_did0.to_string(),
            last_tx_id: last_tx_id.clone(),
            operation: morpheus::SignableOperationDetails::AddRight {
                auth: auth.clone(),
                right: Right::Impersonation.to_string(),
            },
        };

        let mph_persona_signer_privkey = mph_private.key_by_pk(&mph_persona_pubkey0)?;
        let morpheus_signer = PrivateKeySigner::new(mph_persona_signer_privkey.private_key());

        let morpheus_tx1_signables = vec![
            addkey_attempt,
            addright_attempt,
            // _tombstone_attempt,
        ];

        let mph_op_attempts1_builder = morpheus::SignableOperation::new(morpheus_tx1_signables);
        let mph_signed_op_attempts1 = mph_op_attempts1_builder.sign(&morpheus_signer)?;

        let did_ops_tx1 = morpheus::Transaction::new(
            common_fields.clone(),
            vec![OperationAttempt::Signed(mph_signed_op_attempts1)],
        );

        let mut did_ops_tx1_data: TransactionData = did_ops_tx1.to_data();
        hydra_signer.sign_hydra_transaction(&mut did_ops_tx1_data)?;
        show_tx_json("Morpheus transaction 1:", vec![did_ops_tx1_data.clone()])?;

        // let last_tx_id = Some(did_ops_tx1_data.get_id()?);
        let last_tx_id =
            Some("88df06a4faa3401c35c82177dcbd6a27e56acde4155ff11adfe4fdbd7509ec65".to_string());
        let revokeright_attempt = morpheus::SignableOperationAttempt {
            did: mph_persona_did0.to_string(),
            last_tx_id: last_tx_id.clone(),
            operation: morpheus::SignableOperationDetails::RevokeRight {
                auth: auth.clone(),
                right: Right::Impersonation.to_string(),
            },
        };
        let revokekey_attempt = morpheus::SignableOperationAttempt {
            did: mph_persona_did0.to_string(),
            last_tx_id: last_tx_id.clone(),
            operation: morpheus::SignableOperationDetails::RevokeKey { auth },
        };

        let morpheus_tx2_signables = vec![
            revokeright_attempt,
            revokekey_attempt,
            // _tombstone_attempt,
        ];

        let mph_op_attempts2_builder = morpheus::SignableOperation::new(morpheus_tx2_signables);
        let mph_signed_op_attempts2 = mph_op_attempts2_builder.sign(&morpheus_signer)?;

        let common_fields2 =
            CommonTransactionFields { nonce: common_fields.nonce, ..common_fields };
        // let common_fields2 = common_fields;

        let did_ops_tx2 = morpheus::Transaction::new(
            common_fields2,
            vec![OperationAttempt::Signed(mph_signed_op_attempts2)],
        );

        let _tombstone_attempt = morpheus::SignableOperationAttempt {
            did: mph_persona_did0.to_string(),
            last_tx_id: last_tx_id.clone(),
            operation: morpheus::SignableOperationDetails::TombstoneDid {},
        };

        let mut did_ops_tx2_data: TransactionData = did_ops_tx2.to_data();
        hydra_signer.sign_hydra_transaction(&mut did_ops_tx2_data)?;
        show_tx_json("Morpheus transaction 2:", vec![did_ops_tx2_data.clone()])?;

        show_tx_json("Morpheus transaction batch:", vec![did_ops_tx1_data, did_ops_tx2_data])?;

        Ok(())
    }

    fn show_tx_json(message: &str, txs: Vec<TransactionData>) -> Fallible<()> {
        let tx_batch = TxBatch { transactions: txs };
        let txs_json_str = serde_json::to_string(&tx_batch)?;
        println!("{}", message);
        println!("curl --header 'Content-Type: application/json' --request POST --data '{}' http://test.hydra.iop.global:4703/api/v2/transactions", txs_json_str);
        Ok(())
    }
}
