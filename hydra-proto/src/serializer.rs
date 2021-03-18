use super::*;

use crate::txtype::hyd_core::CoreTransactionType;
use crate::txtype::{
    hyd_core::{CoreAsset, CoreTransactionType as CoreTxType},
    TxTypeGroup, *,
};

pub fn to_bytes(
    tx: &TransactionData, skip_signature: bool, skip_second_signature: bool,
    skip_multisignatures: bool,
) -> Result<Vec<u8>> {
    let mut bytes = serialize_common(tx)?;
    serialize_vendor_field(tx, &mut bytes)?;

    match tx.typed_asset.type_group {
        TxTypeGroup::Core => serialize_core_type(tx, &mut bytes)?,
        TxTypeGroup::Iop => {
            if let txtype::Asset::Iop(ref iop_asset) = tx.typed_asset.asset {
                let framed_asset = frame_bytes(&iop_asset.to_bytes()?)?;
                bytes.write_all(&framed_asset)?
            } else {
                bail!("Implementation error: expected IOP transaction type");
            }
        }
    }

    serialize_signatures(
        tx,
        &mut bytes,
        skip_signature,
        skip_second_signature,
        skip_multisignatures,
    )?;

    Ok(bytes)
}

pub fn serialize_common(transaction: &TransactionData) -> Result<Vec<u8>> {
    let mut bytes = vec![];
    bytes.write_u8(0xff)?;
    bytes.write_u8(transaction.version.unwrap_or(0x02))?;
    bytes.write_u8(transaction.network.with_context(|| "Network is missing")?)?;

    bytes.write_u32::<LittleEndian>(transaction.typed_asset.type_group as u32)?;
    bytes.write_u16::<LittleEndian>(transaction.typed_asset.transaction_type as u16)?;
    let nonce: u64 = transaction.nonce.as_ref().with_context(|| "Nonce is missing")?.parse()?;
    bytes.write_u64::<LittleEndian>(nonce)?;

    bytes.write_all(&hex::decode(&transaction.sender_public_key)?)?;

    let fee: u64 = transaction.fee.parse()?;
    bytes.write_u64::<LittleEndian>(fee)?;
    Ok(bytes)
}

pub fn serialize_vendor_field(transaction: &TransactionData, bytes: &mut Vec<u8>) -> Result<()> {
    if let Some(ref vendor_field) = transaction.vendor_field {
        bytes.write_u8(vendor_field.len() as u8)?;
        bytes.write_all(vendor_field.as_bytes())?;
    } else {
        bytes.write_u8(0x00)?;
    }
    Ok(())
}

pub fn serialize_core_type(tx: &TransactionData, mut bytes: &mut Vec<u8>) -> Result<()> {
    ensure!(
        matches!(tx.typed_asset.type_group, TxTypeGroup::Core),
        "Implementation error: expecting Core transaction typeGroup"
    );
    let core_txtype =
        CoreTransactionType::from_u16(tx.typed_asset.transaction_type).with_context(|| {
            format!("Invalid core transaction type: {}", tx.typed_asset.transaction_type)
        })?;
    match core_txtype {
        CoreTxType::Transfer => serialize_transfer(tx, &mut bytes)?,
        CoreTxType::Vote => serialize_vote(tx, &mut bytes)?,
        CoreTxType::DelegateRegistration => serialize_delegate_registration(tx, &mut bytes)?,
        CoreTxType::DelegateResignation => (),
        CoreTxType::SecondSignatureRegistration => {
            unimplemented!()
            //serialize_second_signature_registration(transaction, &mut bytes)?
        }
        CoreTxType::MultiSignatureRegistration => {
            unimplemented!()
            // serialize_multi_signature_registration(transaction, &mut bytes)
        }
        CoreTxType::Ipfs => unimplemented!(),
        CoreTxType::TimelockTransfer => unimplemented!(),
        CoreTxType::MultiPayment => unimplemented!(),
    };
    Ok(())
}

fn serialize_transfer(transaction: &TransactionData, bytes: &mut Vec<u8>) -> Result<()> {
    let amount: u64 = transaction.amount.parse()?;
    bytes.write_u64::<LittleEndian>(amount)?;
    bytes.write_u32::<LittleEndian>(transaction.expiration.unwrap_or(0))?;

    let recipient =
        transaction.recipient_id.as_ref().with_context(|| "No recipient for transfer")?;
    let recipient_id = from_base58check(recipient)?;
    bytes.write_all(&recipient_id)?;
    Ok(())
}

fn serialize_vote(transaction: &TransactionData, bytes: &mut Vec<u8>) -> Result<()> {
    if let Asset::Core(CoreAsset::Votes(votes)) = &transaction.typed_asset.asset {
        let votes_hex: Vec<_> = votes
            .iter()
            .filter_map(|vote| {
                let prefix = if vote.starts_with('+') { "01" } else { "00" };
                vote.get(1..).map(|delegate| format!("{}{}", prefix, delegate))
            })
            .collect();

        bytes.write_u8(votes.len() as u8)?;
        bytes.write_all(&hex::decode(&votes_hex.join(""))?)?;
    }
    Ok(())
}

fn serialize_delegate_registration(tx: &TransactionData, bytes: &mut Vec<u8>) -> Result<()> {
    if let Asset::Core(CoreAsset::Delegate { username }) = &tx.typed_asset.asset {
        bytes.write_u8(username.len() as u8)?;
        bytes.write_all(&username.as_bytes())?;
    }
    Ok(())
}

// fn serialize_second_signature_registration(
//     transaction: &Transaction, bytes: &mut Vec<u8>,
// ) -> Result<()> {
//     if let Some(Asset::Signature { public_key }) = &transaction.asset {
//         let public_key_bytes = hex::decode(public_key)?;
//         bytes.write_all(&public_key_bytes)?;
//     }
//     Ok(())
// }
//
// fn serialize_multi_signature_registration(
//     transaction: &Transaction, bytes: &mut Vec<u8>,
// ) -> Result<()> {
//     if let Asset::MultiSignatureRegistration { min, keysgroup, lifetime } = &transaction.asset {
//         let keysgroup_string: String = keysgroup
//             .iter()
//             .map(|key| {
//                 if key.starts_with('+') {
//                     key.chars().skip(1).collect::<String>()
//                 } else {
//                     key.to_owned()
//                 }
//             })
//             .collect();
//
//         bytes.write_u8(*min)?;
//         bytes.write_u8(keysgroup.len() as u8)?;
//         bytes.write_u8(*lifetime)?;
//
//         bytes.write_all(&hex::decode(keysgroup_string)?)?;
//     }
//     Ok(())
// }

pub fn serialize_signatures(
    transaction: &TransactionData, bytes: &mut Vec<u8>, skip_signature: bool,
    skip_second_signature: bool, skip_multisignatures: bool,
) -> Result<()> {
    if !skip_signature {
        if let Some(ref signature) = transaction.signature {
            write_decoded_hex(signature, bytes)?;
        }
    }
    if !skip_second_signature {
        let second_signature =
            transaction.second_signature.as_ref().or_else(|| transaction.sign_signature.as_ref());
        if let Some(sec_sig) = second_signature {
            write_decoded_hex(sec_sig, bytes)?;
        }
    }
    if !skip_multisignatures && !transaction.signatures.is_empty() {
        write_decoded_hex(&transaction.signatures.join(""), bytes)?;
    }
    Ok(())
}

fn write_decoded_hex(signature: &str, bytes: &mut Vec<u8>) -> Result<()> {
    let signatures_bytes = hex::decode(&signature)?;
    bytes.write_all(&signatures_bytes)?;
    Ok(())
}
