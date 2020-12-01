use super::*;

use iop_hydra_proto::txtype::morpheus::{
    MorpheusAsset, OperationAttempt, SignableOperation, SignableOperationAttempt,
    SignableOperationDetails, SignedOperation, Transaction,
};

pub struct MorpheusOperationBuilder {
    did: Did,
    last_tx_id: Option<String>,
}

impl MorpheusOperationBuilder {
    fn op_to_attempt(&self, operation: SignableOperationDetails) -> SignableOperationAttempt {
        SignableOperationAttempt {
            did: self.did.to_owned(),
            last_tx_id: self.last_tx_id.to_owned(),
            operation,
        }
    }
}

#[no_mangle]
pub extern "C" fn delete_MorpheusOperationBuilder(builder: *mut MorpheusOperationBuilder) {
    delete(builder)
}

#[no_mangle]
pub extern "C" fn MorpheusOperationBuilder_new(
    did: *const Did, last_tx_id: *const raw::c_char,
) -> CPtrResult<MorpheusOperationBuilder> {
    let fun = || {
        let did = unsafe { convert::borrow_in(did) }.to_owned();
        let last_tx_id = unsafe { convert::str_in(last_tx_id) }.ok().map(|t| t.to_owned());
        let builder = MorpheusOperationBuilder { did, last_tx_id };
        Ok(convert::move_out(builder))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusOperationBuilder_add_key(
    builder: *mut MorpheusOperationBuilder, authentication: *const raw::c_char,
    expires_at_height: BlockHeight,
) -> CPtrResult<SignableOperationAttempt> {
    let fun = || {
        let builder = unsafe { convert::borrow_in(builder) };
        let auth = unsafe { convert::str_in(authentication) }?.parse()?;
        let expires_at_height = if expires_at_height == 0 { None } else { Some(expires_at_height) };
        let operation = SignableOperationDetails::AddKey { auth, expires_at_height };
        let attempt = builder.op_to_attempt(operation);
        Ok(convert::move_out(attempt))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusOperationBuilder_revoke_key(
    builder: *mut MorpheusOperationBuilder, authentication: *const raw::c_char,
) -> CPtrResult<SignableOperationAttempt> {
    let fun = || {
        let builder = unsafe { convert::borrow_in(builder) };
        let auth = unsafe { convert::str_in(authentication) }?.parse()?;
        let operation = SignableOperationDetails::RevokeKey { auth };
        let attempt = builder.op_to_attempt(operation);
        Ok(convert::move_out(attempt))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusOperationBuilder_add_right(
    builder: *mut MorpheusOperationBuilder, authentication: *const raw::c_char,
    right: *const raw::c_char,
) -> CPtrResult<SignableOperationAttempt> {
    let fun = || {
        let builder = unsafe { convert::borrow_in(builder) };
        let auth = unsafe { convert::str_in(authentication) }?.parse()?;
        let right = unsafe { convert::str_in(right) }?.to_owned();
        let operation = SignableOperationDetails::AddRight { auth, right };
        let attempt = builder.op_to_attempt(operation);
        Ok(convert::move_out(attempt))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusOperationBuilder_revoke_right(
    builder: *mut MorpheusOperationBuilder, authentication: *const raw::c_char,
    right: *const raw::c_char,
) -> CPtrResult<SignableOperationAttempt> {
    let fun = || {
        let builder = unsafe { convert::borrow_in(builder) };
        let auth = unsafe { convert::str_in(authentication) }?.parse()?;
        let right = unsafe { convert::str_in(right) }?.to_owned();
        let operation = SignableOperationDetails::RevokeRight { auth, right };
        let attempt = builder.op_to_attempt(operation);
        Ok(convert::move_out(attempt))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusOperationBuilder_tombstone_did(
    builder: *mut MorpheusOperationBuilder,
) -> *mut SignableOperationAttempt {
    let builder = unsafe { convert::borrow_in(builder) };
    let operation = SignableOperationDetails::TombstoneDid {};
    let attempt = builder.op_to_attempt(operation);
    convert::move_out(attempt)
}

#[no_mangle]
pub extern "C" fn delete_MorpheusOperation(attempt: *mut SignableOperationAttempt) {
    delete(attempt)
}

pub struct MorpheusOperationSigner {
    operations: Vec<SignableOperationAttempt>,
}

#[no_mangle]
pub extern "C" fn delete_MorpheusOperationSigner(signer: *mut MorpheusOperationSigner) {
    delete(signer)
}

#[no_mangle]
pub extern "C" fn MorpheusOperationSigner_new() -> *mut MorpheusOperationSigner {
    convert::move_out(MorpheusOperationSigner { operations: vec![] })
}

#[no_mangle]
pub extern "C" fn MorpheusOperationSigner_add(
    signer: *mut MorpheusOperationSigner, op: *mut SignableOperationAttempt,
) {
    let signer = unsafe { convert::borrow_mut_in(signer) };
    let op = unsafe { convert::borrow_in(op) };
    signer.operations.push(op.to_owned());
}

#[no_mangle]
pub extern "C" fn MorpheusOperationSigner_sign(
    signer: *mut MorpheusOperationSigner, private_key: *const MPrivateKey,
) -> CPtrResult<SignedOperation> {
    let fun = || {
        let signer = unsafe { convert::borrow_in(signer) };
        let private_key = unsafe { convert::borrow_in(private_key) };
        let signable_ops = SignableOperation::new(signer.operations.to_owned());
        let signer = PrivateKeySigner::new(private_key.to_owned());
        let signed_ops = signable_ops.sign(&signer)?;
        Ok(convert::move_out(signed_ops))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_MorpheusSignedOperation(operation: *mut SignedOperation) {
    delete(operation)
}

#[no_mangle]
pub extern "C" fn MorpheusSignedOperation_to_string(
    signed_op: *mut SignedOperation,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let signed_op = unsafe { convert::borrow_in(signed_op) };
        let json_str = serde_json::to_string(signed_op)?;
        Ok(convert::string_out(json_str))
    };
    cresult(fun())
}

pub struct MorpheusAssetBuilder {
    op_attempts: Vec<OperationAttempt>,
}

#[no_mangle]
pub extern "C" fn delete_MorpheusAssetBuilder(builder: *mut MorpheusAssetBuilder) {
    delete(builder)
}

#[no_mangle]
pub extern "C" fn MorpheusAssetBuilder_new() -> *mut MorpheusAssetBuilder {
    let builder = MorpheusAssetBuilder { op_attempts: Default::default() };
    convert::move_out(builder)
}

#[no_mangle]
pub extern "C" fn MorpheusAssetBuilder_add_signed(
    builder: *mut MorpheusAssetBuilder, signed_ops: *mut SignedOperation,
) {
    let builder = unsafe { convert::borrow_mut_in(builder) };
    let signed_ops = unsafe { convert::borrow_in(signed_ops) };
    builder.op_attempts.push(OperationAttempt::Signed(signed_ops.to_owned()));
}

#[no_mangle]
pub extern "C" fn MorpheusAssetBuilder_add_register_before_proof(
    builder: *mut MorpheusAssetBuilder, content_id: *const raw::c_char,
) -> CPtrResult<raw::c_void> {
    let fun = || {
        let builder = unsafe { convert::borrow_mut_in(builder) };
        let content_id = unsafe { convert::str_in(content_id) }?;
        let before_proof =
            OperationAttempt::RegisterBeforeProof { content_id: content_id.to_owned() };
        builder.op_attempts.push(before_proof);
        Ok(())
    };
    cresult_void(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusAssetBuilder_build(
    builder: *mut MorpheusAssetBuilder,
) -> *mut MorpheusAsset {
    let builder = unsafe { convert::borrow_mut_in(builder) };
    let asset = MorpheusAsset::new(builder.op_attempts.to_owned());
    convert::move_out(asset)
}

#[no_mangle]
pub extern "C" fn delete_MorpheusAsset(asset: *mut MorpheusAsset) {
    delete(asset)
}

#[no_mangle]
pub extern "C" fn MorpheusAsset_to_string(asset: *mut MorpheusAsset) -> CPtrResult<raw::c_char> {
    let fun = || {
        let asset = unsafe { convert::borrow_in(asset) };
        let json_str = serde_json::to_string(asset)?;
        Ok(convert::string_out(json_str))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusTxBuilder_build(
    network: *const raw::c_char, asset: *const MorpheusAsset,
    sender_public_key: *const SecpPublicKey, nonce: u64,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let network = unsafe { convert::str_in(network)? };
        let asset = unsafe { convert::borrow_in(asset) };
        let sender_public_key = unsafe { convert::borrow_in(sender_public_key) };
        let common_fields = CommonTransactionFields {
            network: Networks::by_name(network)?,
            sender_public_key: sender_public_key.to_owned(),
            nonce,
            optional: Default::default(),
        };

        let tx = Transaction::new(common_fields.to_owned(), asset.operation_attempts.to_owned());
        let tx_json = serde_json::to_string(&tx.to_data())?;
        Ok(convert::string_out(tx_json))
    };
    cresult(fun())
}
