mod plugin;
mod private;
mod private_kind;
mod public;
mod public_kind;
mod types;

pub use plugin::*;
pub use private::*;
pub use private_kind::*;
pub use public::*;
pub use public_kind::*;
pub use types::*;

use super::*;

#[cfg(test)]
mod test {
    use super::*;

    use crate::vault_morpheus::plugin::Plugin as MorpheusPlugin;
    use iop_keyvault::{multicipher::MKeyId, secp256k1::hyd, PublicKey, Seed};
    use iop_morpheus_core::hydra_sdk::vault_hydra::{self as hydra, HydraSigner};
    use iop_morpheus_core::{
        crypto::sign::PrivateKeySigner,
        data::{auth::Authentication, did::Did, diddoc::Right},
        hydra::{
            transaction::{TransactionData, TxBatch},
            txtype::{
                morpheus, morpheus::OperationAttempt, Aip29Transaction, CommonTransactionFields,
            },
        },
    };
    use iop_vault::Vault;

    #[test]
    fn api() -> Result<()> {
        let unlock_password = "correct horse battery staple";
        let mut vault = Vault::create(None, Seed::DEMO_PHRASE, "", unlock_password)?;
        Plugin::rewind(&mut vault, unlock_password)?;

        let morpheus = Plugin::get(&vault)?;
        let morpheus_priv = morpheus.private(unlock_password)?;
        let mut personas = morpheus_priv.personas()?;
        let persona_0 = personas.key(0)?;
        let pub_0 = persona_0.neuter();
        let pk0 = pub_0.public_key();

        assert_eq!(&pk0.to_string(), "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6");

        let id0 = pk0.key_id();

        assert_eq!(&id0.to_string(), "iezqztJ6XX6GDxdSgdiySiT3J");

        let persona_0_by_pk = morpheus_priv.key_by_pk(&pk0)?;

        assert_eq!(persona_0_by_pk.path().idx(), 0);

        let err = Plugin::rewind(&mut vault, unlock_password).unwrap_err();
        assert!((&err.to_string()).contains("was already added"));

        Ok(())
    }

    const DEMO_VAULT_DAT: &str = r#"
    {
        "encryptedSeed": "uKOE-HCgv-CUHFuL6jCUHMdXrfgGX-nsUM2FwE-5JY0GhSxOFTQSGB4F_N6VwuDYPQ8-q0Q_eQVCpgOsjRzqJAnr8nhyV32yNtpCsGYimpnEjr_enZDOd4jajLjt7b48J7V5yDKKVyp8",
        "plugins": [
            {
                "pluginName": "Morpheus",
                "parameters": {},
                "publicState": {
                    "personas": [
                        "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6",
                        "pezDj6ea4tVfNRUTMyssVDepAAzPW67Fe3yHtuHL6ZNtcfJ",
                        "pezsfLDb1fngso3J7TXU6jP3nSr2iubcJZ4KXanxrhs9gr"
                    ]
                }
            }
        ]
    }"#;

    #[test]
    fn serialize() -> Result<()> {
        let unlock_password = "correct horse battery staple";
        let vault: Vault = serde_json::from_str(DEMO_VAULT_DAT)?;

        let m = vault_morpheus::Plugin::get(&vault)?;

        let m_private = m.private(unlock_password)?;
        let m_pk: MPublicKey = "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6".parse()?;
        let persona0 = m_private.key_by_pk(&m_pk)?;
        let did = Did::from(persona0.neuter().public_key().key_id());

        assert_eq!(&did.to_string(), "did:morpheus:ezqztJ6XX6GDxdSgdiySiT3J");

        Ok(())
    }

    #[test]
    fn morpheus_tx_builder() -> Result<()> {
        let unlock_password = "test";
        let mut vault = Vault::create(None, Seed::DEMO_PHRASE, "", unlock_password)?;

        let hyd_params = hydra::Parameters::new(&hyd::Testnet, 0);
        hydra::Plugin::rewind(&mut vault, unlock_password, &hyd_params)?;
        let hydra_plugin = hydra::Plugin::get(&vault, &hyd_params)?;
        let hyd_bip44_pubkey0 = hydra_plugin.public()?.key(0)?;
        let hyd_wallet_pubkey0 = hyd_bip44_pubkey0.to_public_key();
        let hydra_priv = hydra_plugin.private(&unlock_password)?;
        let hydra_signer = hydra_priv.key_by_pk(&hyd_wallet_pubkey0)?.to_private_key();

        println!("Hydra Wallet 0 Public Key: {}", hyd_wallet_pubkey0.to_string());
        println!("Hydra Wallet 0 Address: {}", hyd_bip44_pubkey0.to_p2pkh_addr());
        println!(
            "Hydra Wallet 0 Ark KeyId: {}",
            MKeyId::from(hyd_wallet_pubkey0.ark_key_id()).to_string()
        );

        MorpheusPlugin::rewind(&mut vault, &unlock_password)?;
        let morpheus_plugin = MorpheusPlugin::get(&vault)?;
        let mph_private = morpheus_plugin.private(&unlock_password)?;
        let mph_bip32_privkey0 = mph_private.personas()?.key(0)?;
        let mph_bip32_pubkey0 = mph_bip32_privkey0.neuter();
        let mph_persona_pubkey0 = mph_bip32_pubkey0.public_key();
        let mph_persona_did0 = Did::new(mph_persona_pubkey0.key_id());

        println!("Morpheus Persona 0 Public Key: {}", mph_persona_pubkey0.to_string());
        println!("Morpheus Persona 0 Did: {}", mph_persona_did0.to_string());

        let common_fields = CommonTransactionFields {
            sender_public_key: hyd_wallet_pubkey0,
            nonce: 14,
            ..Default::default()
        };

        let reg_proof_attempt = morpheus::OperationAttempt::RegisterBeforeProof {
            content_id: "<<placeholder of your 3rd favourite wisdom>>".to_owned(),
        };
        let reg_proof_tx =
            morpheus::Transaction::new(common_fields.clone(), vec![reg_proof_attempt]);
        let mut reg_proof_tx_data: TransactionData = reg_proof_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut reg_proof_tx_data)?;
        show_tx_json("Register-before-proof transaction:", vec![reg_proof_tx_data])?;

        // let recipients = vec![hyd_core::PaymentsItem { amount: 1, recipient_id: "tBlah" }];
        // let multitransfer_tx = hyd_core::MultiTransferTransaction::new(core_tx_fields, recipients);

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

    fn show_tx_json(message: &str, txs: Vec<TransactionData>) -> Result<()> {
        let tx_batch = TxBatch { transactions: txs };
        let txs_json_str = serde_json::to_string(&tx_batch)?;
        println!("{}", message);
        println!("curl --header 'Content-Type: application/json' --request POST --data '{}' http://test.hydra.iop.global:4703/api/v2/transactions", txs_json_str);
        Ok(())
    }
}
