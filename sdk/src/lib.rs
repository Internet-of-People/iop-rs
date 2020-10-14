#[cfg(test)]
mod test {
    //use super::*;

    use anyhow::Result;

    use iop_hydra_proto::{
        transaction::{TransactionData, TxBatch},
        txtype::{
            hyd_core, morpheus, morpheus::OperationAttempt, Aip29Transaction,
            CommonTransactionFields,
        },
    };
    use iop_hydra_sdk::vault::{self as hydra, HydraSigner};
    use iop_keyvault::secp256k1::{SecpKeyId, SecpPublicKey};
    use iop_keyvault::{multicipher::MKeyId, secp256k1::hyd, PublicKey, Seed};
    use iop_morpheus_core::{
        crypto::sign::PrivateKeySigner,
        data::{auth::Authentication, did::Did, diddoc::Right},
    };
    use iop_morpheus_sdk::vault::Plugin as MorpheusPlugin;
    use iop_vault::Vault;

    #[test]
    fn hydra_tx_builder() -> Result<()> {
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

        let common_fields = CommonTransactionFields {
            sender_public_key: hyd_wallet_pubkey0,
            nonce: 245,
            ..Default::default()
        };

        let transfer_common = CommonTransactionFields {
            amount: 3_141_593,
            manual_fee: Some(1_000_000),
            ..common_fields.clone()
        };
        let recipient_id =
            SecpKeyId::from_p2pkh_addr("tjseecxRmob5qBS2T3qc8frXDKz3YUGB8J", &hyd::Testnet)?;
        let transfer_tx = hyd_core::Transaction::transfer(transfer_common, &recipient_id);
        let mut transfer_tx_data = transfer_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut transfer_tx_data)?;
        show_tx_json("Transfer transaction:", vec![transfer_tx_data])?;

        let genesis_1_pubkey: SecpPublicKey =
            "02ae6eaed36910a51807c9dfb51c2e2988abf9008381fe4e00995e01b6714e3db2".parse()?;

        let vote_tx = hyd_core::Transaction::vote(common_fields.clone(), &genesis_1_pubkey);
        let mut vote_tx_data = vote_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut vote_tx_data)?;
        show_tx_json("Vote transaction:", vec![vote_tx_data])?;

        let unvote_tx = hyd_core::Transaction::unvote(common_fields.clone(), &genesis_1_pubkey);
        let mut unvote_tx_data = unvote_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut unvote_tx_data)?;
        show_tx_json("Unvote transaction:", vec![unvote_tx_data])?;

        let reg_tx = hyd_core::Transaction::register_delegate(common_fields, "test-delegate");
        let mut reg_tx_data = reg_tx.to_data();
        hydra_signer.sign_hydra_transaction(&mut reg_tx_data)?;
        show_tx_json("Register delegate transaction:", vec![reg_tx_data])?;

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
