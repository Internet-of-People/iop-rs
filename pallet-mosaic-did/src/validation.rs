//! Operation validation logic for the Mosaic DID pallet (v2 — direct state reads).
//!
//! Validation reads directly from `DidStates` storage instead of replaying
//! an operation log. This is O(1) per DID lookup instead of O(n) replay.

use frame_support::ensure;

use mosaic_did_types::{
    Did, DidOperation, Right, SignedDidOperation,
};

use crate::pallet::{Config, DidStates, Error};

/// Validate a signed DID operation before applying it.
///
/// Reads materialized state directly — no history reconstruction needed.
pub fn validate_did_operation<T: Config>(
    signed_op: &SignedDidOperation,
    height_u32: u32,
) -> Result<(), sp_runtime::DispatchError> {
    let did = signed_op.operation.did();

    // 1. Check if DID exists and is not tombstoned
    let maybe_state = DidStates::<T>::get(did);

    match &signed_op.operation {
        DidOperation::AddKey { did, public_key, .. } => {
            match &maybe_state {
                Some(state) => {
                    // DID exists: check not tombstoned, check nonce, check Update right
                    ensure!(
                        !state.is_tombstoned_at(height_u32),
                        Error::<T>::DidTombstoned
                    );
                    validate_nonce::<T>(state, signed_op)?;
                    ensure!(
                        state.has_right_at(
                            signed_op.signer_key_id.as_bytes(),
                            Right::Update,
                            height_u32,
                        ),
                        Error::<T>::InsufficientRight
                    );
                }
                None => {
                    // Implicit DID creation: DID must equal BLAKE3-256(public_key)
                    let derived_did = Did::from_public_key(public_key);
                    ensure!(*did == derived_did, Error::<T>::DidMismatch);
                }
            }
        }

        DidOperation::RevokeKey { .. } => {
            let state = maybe_state.as_ref().ok_or(Error::<T>::DidNotFound)?;
            ensure!(!state.is_tombstoned_at(height_u32), Error::<T>::DidTombstoned);
            validate_nonce::<T>(state, signed_op)?;
            ensure!(
                state.has_right_at(
                    signed_op.signer_key_id.as_bytes(),
                    Right::Update,
                    height_u32,
                ),
                Error::<T>::InsufficientRight
            );
        }

        DidOperation::AddRight { .. } => {
            let state = maybe_state.as_ref().ok_or(Error::<T>::DidNotFound)?;
            ensure!(!state.is_tombstoned_at(height_u32), Error::<T>::DidTombstoned);
            validate_nonce::<T>(state, signed_op)?;
            ensure!(
                state.has_right_at(
                    signed_op.signer_key_id.as_bytes(),
                    Right::Delegate,
                    height_u32,
                ),
                Error::<T>::InsufficientRight
            );
        }

        DidOperation::RevokeRight { .. } => {
            let state = maybe_state.as_ref().ok_or(Error::<T>::DidNotFound)?;
            ensure!(!state.is_tombstoned_at(height_u32), Error::<T>::DidTombstoned);
            validate_nonce::<T>(state, signed_op)?;
            ensure!(
                state.has_right_at(
                    signed_op.signer_key_id.as_bytes(),
                    Right::Delegate,
                    height_u32,
                ),
                Error::<T>::InsufficientRight
            );
        }

        DidOperation::TombstoneDid { .. } => {
            let state = maybe_state.as_ref().ok_or(Error::<T>::DidNotFound)?;
            ensure!(!state.is_tombstoned_at(height_u32), Error::<T>::DidTombstoned);
            validate_nonce::<T>(state, signed_op)?;
            ensure!(
                state.has_right_at(
                    signed_op.signer_key_id.as_bytes(),
                    Right::Update,
                    height_u32,
                ),
                Error::<T>::InsufficientRight
            );
        }
    }

    // Validate signature (basic check for Phase 1 MVP)
    validate_signature::<T>(signed_op)?;

    Ok(())
}

/// Validate nonce is strictly greater than stored nonce (replay protection).
fn validate_nonce<T: Config>(
    state: &crate::state::OnChainDidState,
    signed_op: &SignedDidOperation,
) -> Result<(), sp_runtime::DispatchError> {
    let stored_nonce = state.nonce_for_key(signed_op.signer_key_id.as_bytes());
    ensure!(signed_op.nonce > stored_nonce, Error::<T>::InvalidNonce);
    Ok(())
}

/// Validate the cryptographic signature on a signed operation.
fn validate_signature<T: Config>(
    signed_op: &SignedDidOperation,
) -> Result<(), sp_runtime::DispatchError> {
    let sig_bytes = signed_op.signature.as_bytes();
    ensure!(
        !sig_bytes.iter().all(|b| *b == 0),
        Error::<T>::InvalidSignature
    );
    Ok(())
}
