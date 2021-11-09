pub mod ciphersuite {
    pub mod ed25519 {
        pub use iop_keyvault::ed25519::*;
    }
    pub mod secp256k1 {
        pub use iop_keyvault::secp256k1::*;
    }
}

pub mod multicipher {
    pub use iop_keyvault::multicipher::*;
}

pub mod journal {
    pub use iop_journal_proto::*;
}

pub use json_digest;

pub mod hydra {
    pub use iop_hydra_proto::*;
}

pub mod morpheus {
    pub use iop_morpheus_proto::*;
}

pub mod coeus {
    pub use iop_coeus_proto::*;
}

pub mod vault {
    pub use iop_hydra_sdk::vault as hydra;
    pub use iop_keyvault::{
        Bip39, Bip39ErrorKind, Bip39Language, Bip39Phrase, Network, Networks, PrivateKey,
        PublicKey, Seed, Subtree,
    };
    pub use iop_morpheus_sdk::vault as morpheus;
    pub use iop_vault::*;
}

#[cfg(test)]
mod test {
    use super::*;

    // to_data
    use hydra::txtype::Aip29Transaction;
    // sign_hydra_transaction
    use vault::hydra::HydraSigner;
    // key_id
    use vault::PublicKey;

    #[test]
    fn hydra_tx_builder() -> anyhow::Result<()> {
        let unlock_password = "test";
        let mut vault = vault::Vault::create(None, vault::Seed::DEMO_PHRASE, "", unlock_password)?;

        let hyd_params = vault::hydra::Parameters::new(&ciphersuite::secp256k1::hyd::Testnet, 0);
        vault::hydra::Plugin::init(&mut vault, unlock_password, &hyd_params)?;
        let hydra_plugin = vault::hydra::Plugin::get(&vault, &hyd_params)?;
        let hyd_bip44_pubkey0 = hydra_plugin.public()?.key(0)?;
        let hyd_wallet_pubkey0 = hyd_bip44_pubkey0.to_public_key();
        let hydra_priv = hydra_plugin.private(&unlock_password)?;
        let hydra_signer = hydra_priv.key_by_pk(&hyd_wallet_pubkey0)?.to_private_key();

        println!("Hydra Wallet 0 Public Key: {}", hyd_wallet_pubkey0.to_string());
        println!("Hydra Wallet 0 Address: {}", hyd_bip44_pubkey0.to_p2pkh_addr());
        println!(
            "Hydra Wallet 0 Ark KeyId: {}",
            multicipher::MKeyId::from(hyd_wallet_pubkey0.ark_key_id()).to_string()
        );

        let common_fields = hydra::txtype::CommonTransactionFields {
            network: &ciphersuite::secp256k1::hyd::Testnet,
            sender_public_key: hyd_wallet_pubkey0,
            nonce: 245,
            optional: Default::default(),
        };

        let transfer_common = hydra::txtype::CommonTransactionFields {
            optional: hydra::txtype::OptionalTransactionFields {
                amount: 3_141_593,
                manual_fee: Some(1_000_000),
                ..common_fields.optional.clone()
            },
            ..common_fields.clone()
        };
        let recipient_id = ciphersuite::secp256k1::SecpKeyId::from_p2pkh_addr(
            "tjseecxRmob5qBS2T3qc8frXDKz3YUGB8J",
            &ciphersuite::secp256k1::hyd::Testnet,
        )?;
        let transfer_tx =
            hydra::txtype::hyd_core::Transaction::transfer(transfer_common, &recipient_id);
        let mut transfer_tx_data = transfer_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut transfer_tx_data)?;
        show_tx_json("Transfer transaction:", vec![transfer_tx_data])?;

        let genesis_1_pubkey: ciphersuite::secp256k1::SecpPublicKey =
            "02ae6eaed36910a51807c9dfb51c2e2988abf9008381fe4e00995e01b6714e3db2".parse()?;

        let vote_tx =
            hydra::txtype::hyd_core::Transaction::vote(common_fields.clone(), &genesis_1_pubkey);
        let mut vote_tx_data = vote_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut vote_tx_data)?;
        show_tx_json("Vote transaction:", vec![vote_tx_data])?;

        let unvote_tx =
            hydra::txtype::hyd_core::Transaction::unvote(common_fields.clone(), &genesis_1_pubkey);
        let mut unvote_tx_data = unvote_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut unvote_tx_data)?;
        show_tx_json("Unvote transaction:", vec![unvote_tx_data])?;

        let reg_tx =
            hydra::txtype::hyd_core::Transaction::register_delegate(common_fields, "test-delegate");
        let mut reg_tx_data = reg_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut reg_tx_data)?;
        show_tx_json("Register delegate transaction:", vec![reg_tx_data])?;

        Ok(())
    }

    #[test]
    fn morpheus_tx_builder() -> anyhow::Result<()> {
        let unlock_password = "test";
        let mut vault = vault::Vault::create(None, vault::Seed::DEMO_PHRASE, "", unlock_password)?;

        let hyd_params = vault::hydra::Parameters::new(&ciphersuite::secp256k1::hyd::Testnet, 0);
        vault::hydra::Plugin::init(&mut vault, unlock_password, &hyd_params)?;
        let hydra_plugin = vault::hydra::Plugin::get(&vault, &hyd_params)?;
        let hyd_bip44_pubkey0 = hydra_plugin.public()?.key(0)?;
        let hyd_wallet_pubkey0 = hyd_bip44_pubkey0.to_public_key();
        let hydra_priv = hydra_plugin.private(&unlock_password)?;
        let hydra_signer = hydra_priv.key_by_pk(&hyd_wallet_pubkey0)?.to_private_key();

        println!("Hydra Wallet 0 Public Key: {}", hyd_wallet_pubkey0.to_string());
        println!("Hydra Wallet 0 Address: {}", hyd_bip44_pubkey0.to_p2pkh_addr());
        println!(
            "Hydra Wallet 0 Ark KeyId: {}",
            multicipher::MKeyId::from(hyd_wallet_pubkey0.ark_key_id()).to_string()
        );

        vault::morpheus::Plugin::init(&mut vault, &unlock_password)?;
        let morpheus_plugin = vault::morpheus::Plugin::get(&vault)?;
        let mph_private = morpheus_plugin.private(&unlock_password)?;
        let mph_bip32_privkey0 = mph_private.personas()?.key(0)?;
        let mph_bip32_pubkey0 = mph_bip32_privkey0.neuter();
        let mph_persona_pubkey0 = mph_bip32_pubkey0.public_key();
        let mph_persona_did0 = morpheus::data::Did::new(mph_persona_pubkey0.key_id());

        println!("Morpheus Persona 0 Public Key: {}", mph_persona_pubkey0.to_string());
        println!("Morpheus Persona 0 Did: {}", mph_persona_did0.to_string());

        let common_fields = hydra::txtype::CommonTransactionFields {
            network: &ciphersuite::secp256k1::hyd::Testnet,
            sender_public_key: hyd_wallet_pubkey0,
            nonce: 14,
            optional: Default::default(),
        };

        let reg_proof_attempt = morpheus::txtype::OperationAttempt::RegisterBeforeProof {
            content_id: "<<placeholder of your 3rd favourite wisdom>>".to_owned(),
        };
        let reg_proof_tx = hydra::txtype::morpheus::Transaction::new(
            common_fields.clone(),
            vec![reg_proof_attempt],
        );
        let mut reg_proof_tx_data: hydra::TransactionData = reg_proof_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut reg_proof_tx_data)?;
        show_tx_json("Register-before-proof transaction:", vec![reg_proof_tx_data])?;

        // let recipients = vec![hydra::txtype::hyd_core::PaymentsItem { amount: 1, recipient_id: "tBlah" }];
        // let multitransfer_tx = hydra::txtype::hyd_core::MultiTransferTransaction::new(core_tx_fields, recipients);

        let auth: morpheus::data::Authentication = "iez25N5WZ1Q6TQpgpyYgiu9gTX".parse()?;
        let last_tx_id =
            Some("88df06a4faa3401c35c82177dcbd6a27e56acde4155ff11adfe4fdbd7509ec65".to_owned());
        let addkey_attempt = morpheus::txtype::SignableOperationAttempt {
            did: mph_persona_did0.clone(),
            last_tx_id: last_tx_id.clone(),
            operation: morpheus::txtype::SignableOperationDetails::AddKey {
                auth: auth.clone(),
                expires_at_height: None,
            },
        };
        let addright_attempt = morpheus::txtype::SignableOperationAttempt {
            did: mph_persona_did0.clone(),
            last_tx_id: last_tx_id.clone(),
            operation: morpheus::txtype::SignableOperationDetails::AddRight {
                auth: auth.clone(),
                right: morpheus::data::Right::Impersonation.to_string(),
            },
        };

        let mph_persona_signer_privkey = mph_private.key_by_pk(&mph_persona_pubkey0)?;
        let morpheus_signer =
            morpheus::crypto::PrivateKeySigner::new(mph_persona_signer_privkey.private_key());

        let morpheus_tx1_signables = vec![
            addkey_attempt,
            addright_attempt,
            // _tombstone_attempt,
        ];

        let mph_op_attempts1 = morpheus::txtype::SignableOperation::new(morpheus_tx1_signables);
        let mph_signed_op_attempts1 = mph_op_attempts1.sign(&morpheus_signer)?;

        let did_ops_tx1 = hydra::txtype::morpheus::Transaction::new(
            common_fields.clone(),
            vec![morpheus::txtype::OperationAttempt::Signed(mph_signed_op_attempts1)],
        );

        let mut did_ops_tx1_data: hydra::TransactionData = did_ops_tx1.to_data();
        hydra_signer.sign_hydra_transaction(&mut did_ops_tx1_data)?;
        show_tx_json("Morpheus transaction 1:", vec![did_ops_tx1_data.clone()])?;

        // let last_tx_id = Some(did_ops_tx1_data.get_id()?);
        let last_tx_id =
            Some("88df06a4faa3401c35c82177dcbd6a27e56acde4155ff11adfe4fdbd7509ec65".to_string());
        let revokeright_attempt = morpheus::txtype::SignableOperationAttempt {
            did: mph_persona_did0.clone(),
            last_tx_id: last_tx_id.clone(),
            operation: morpheus::txtype::SignableOperationDetails::RevokeRight {
                auth: auth.clone(),
                right: morpheus::data::Right::Impersonation.to_string(),
            },
        };
        let revokekey_attempt = morpheus::txtype::SignableOperationAttempt {
            did: mph_persona_did0.clone(),
            last_tx_id: last_tx_id.clone(),
            operation: morpheus::txtype::SignableOperationDetails::RevokeKey { auth },
        };

        let morpheus_tx2_signables = vec![
            revokeright_attempt,
            revokekey_attempt,
            // _tombstone_attempt,
        ];

        let mph_op_attempts2 = morpheus::txtype::SignableOperation::new(morpheus_tx2_signables);
        let mph_signed_op_attempts2 = mph_op_attempts2.sign(&morpheus_signer)?;

        let common_fields2 =
            hydra::txtype::CommonTransactionFields { nonce: common_fields.nonce, ..common_fields };
        // let common_fields2 = common_fields;

        let did_ops_tx2 = hydra::txtype::morpheus::Transaction::new(
            common_fields2,
            vec![morpheus::txtype::OperationAttempt::Signed(mph_signed_op_attempts2)],
        );

        let _tombstone_attempt = morpheus::txtype::SignableOperationAttempt {
            did: mph_persona_did0,
            last_tx_id: last_tx_id.clone(),
            operation: morpheus::txtype::SignableOperationDetails::TombstoneDid {},
        };

        let mut did_ops_tx2_data: hydra::TransactionData = did_ops_tx2.to_data();
        hydra_signer.sign_hydra_transaction(&mut did_ops_tx2_data)?;
        show_tx_json("Morpheus transaction 2:", vec![did_ops_tx2_data.clone()])?;

        show_tx_json("Morpheus transaction batch:", vec![did_ops_tx1_data, did_ops_tx2_data])?;

        Ok(())
    }

    fn show_tx_json(message: &str, txs: Vec<hydra::TransactionData>) -> anyhow::Result<()> {
        let tx_batch = hydra::TxBatch { transactions: txs };
        let txs_json_str = serde_json::to_string(&tx_batch)?;
        println!("{}", message);
        println!("curl --header 'Content-Type: application/json' --request POST --data '{}' http://test.hydra.iop.global:4703/api/v2/transactions", txs_json_str);
        Ok(())
    }
}
