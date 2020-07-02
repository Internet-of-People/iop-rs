use super::*;

pub fn to_bytes(
    tx: &TransactionData, skip_signature: bool, skip_second_signature: bool,
    skip_multisignatures: bool,
) -> Fallible<Vec<u8>> {
    let mut bytes = serialize_common(tx)?;
    serialize_vendor_field(tx, &mut bytes)?;

    match tx.type_group {
        Some(hyd_core::TransactionType::TYPE_GROUP) => serialize_core_type(tx, &mut bytes)?,
        Some(morpheus::TransactionType::TYPE_GROUP) => {
            let asset = match tx.asset {
                Some(Asset::Morpheus(ref morpheus_asset)) => morpheus_asset,
                _ => bail!("Implementation error: handling wrong asset type"),
            };
            bytes.write_all(&asset.to_bytes()?)?;
        }
        _ => bail!("Unknown transaction type group: {:?}", tx.type_group),
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

pub fn serialize_common(transaction: &TransactionData) -> Fallible<Vec<u8>> {
    let mut bytes = vec![];
    bytes.write_u8(0xff)?;
    bytes.write_u8(transaction.version.unwrap_or(0x02))?;
    bytes.write_u8(transaction.network.ok_or_else(|| err_msg("Network is missing"))?)?;

    bytes.write_u32::<LittleEndian>(
        transaction.type_group.unwrap_or(hyd_core::TransactionType::TYPE_GROUP),
    )?;
    bytes.write_u16::<LittleEndian>(transaction.transaction_type.into_u16())?;
    let nonce: u64 =
        transaction.nonce.as_ref().ok_or_else(|| err_msg("Nonce is missing"))?.parse()?;
    bytes.write_u64::<LittleEndian>(nonce)?;

    bytes.write_all(&hex::decode(&transaction.sender_public_key)?)?;

    let fee: u64 = transaction.fee.parse()?;
    bytes.write_u64::<LittleEndian>(fee)?;
    Ok(bytes)
}

pub fn serialize_vendor_field(transaction: &TransactionData, bytes: &mut Vec<u8>) -> Fallible<()> {
    if let Some(ref vendor_field) = transaction.vendor_field {
        bytes.write_u8(vendor_field.len() as u8)?;
        bytes.write_all(vendor_field.as_bytes())?;
    } else {
        bytes.write_u8(0x00)?;
    }
    Ok(())
}

pub fn serialize_core_type(transaction: &TransactionData, mut bytes: &mut Vec<u8>) -> Fallible<()> {
    let core_type = match transaction.transaction_type {
        TransactionType::Core(core_type) => core_type,
        _ => bail!("Implementation error: handling wrong TX type"),
    };
    match core_type {
        hyd_core::TransactionType::Transfer => serialize_transfer(transaction, &mut bytes)?,
        hyd_core::TransactionType::SecondSignatureRegistration => {
            unimplemented!()
            //serialize_second_signature_registration(transaction, &mut bytes)?
        }
        hyd_core::TransactionType::DelegateRegistration => {
            unimplemented!()
            //serialize_delegate_registration(transaction, &mut bytes)?
        }
        hyd_core::TransactionType::Vote => {
            unimplemented!()
            //serialize_vote(transaction, &mut bytes)?
        }
        hyd_core::TransactionType::MultiSignatureRegistration => {
            unimplemented!()
            // serialize_multi_signature_registration(transaction, &mut bytes)
        }
        hyd_core::TransactionType::Ipfs => (),
        hyd_core::TransactionType::TimelockTransfer => (),
        hyd_core::TransactionType::MultiPayment => (),
        hyd_core::TransactionType::DelegateResignation => (),
    };
    Ok(())
}

fn serialize_transfer(transaction: &TransactionData, bytes: &mut Vec<u8>) -> Fallible<()> {
    let amount: u64 = transaction.amount.parse()?;
    bytes.write_u64::<LittleEndian>(amount)?;
    bytes.write_u32::<LittleEndian>(transaction.expiration.unwrap_or(0))?;

    let recipient =
        transaction.recipient_id.as_ref().ok_or_else(|| err_msg("No recipient for transfer"))?;
    let recipient_id = from_base58check(recipient)?;
    bytes.write_all(&recipient_id)?;
    Ok(())
}

// fn serialize_second_signature_registration(
//     transaction: &Transaction, bytes: &mut Vec<u8>,
// ) -> Fallible<()> {
//     if let Some(Asset::Signature { public_key }) = &transaction.asset {
//         let public_key_bytes = hex::decode(public_key)?;
//         bytes.write_all(&public_key_bytes)?;
//     }
//     Ok(())
// }
//
// fn serialize_delegate_registration(transaction: &Transaction, bytes: &mut Vec<u8>) -> Fallible<()> {
//     if let Some(Asset::Delegate { username }) = &transaction.asset {
//         bytes.write_u8(username.len() as u8)?;
//         bytes.write_all(&username.as_bytes())?;
//     }
//     Ok(())
// }
//
// fn serialize_vote(transaction: &Transaction, bytes: &mut Vec<u8>) -> Fallible<()> {
//     if let Some(Asset::Votes(votes)) = &transaction.asset {
//         let mut vote_bytes = vec![];
//
//         for vote in votes {
//             let prefix = if vote.starts_with('+') { "01" } else { "00" };
//             let _vote: String = vote.chars().skip(1).collect();
//             vote_bytes.push(format!("{}{}", prefix, _vote));
//         }
//
//         bytes.write_u8(votes.len() as u8)?;
//         bytes.write_all(&hex::decode(&vote_bytes.join(""))?)?;
//     }
//     Ok(())
// }
//
// fn serialize_multi_signature_registration(
//     transaction: &Transaction, bytes: &mut Vec<u8>,
// ) -> Fallible<()> {
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
) -> Fallible<()> {
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

fn write_decoded_hex(signature: &str, bytes: &mut Vec<u8>) -> Fallible<()> {
    let signatures_bytes = hex::decode(&signature)?;
    bytes.write_all(&signatures_bytes)?;
    Ok(())
}
