//! # Mosaic DID Registry Pallet
//!
//! Substrate pallet implementing the `did:mosaic` DID method for the
//! Mosaic Trust Network. Derived from the IOP Morpheus SSI Stack.
//!
//! ## Architecture (v2 — Materialized State)
//!
//! DID document state is stored directly on-chain using `OnChainDidState`.
//! No event-log replay is needed for validation — the pallet reads and
//! mutates state in-place. Events carry full operation payloads for
//! off-chain indexers to reconstruct history.
//!
//! ## Anti-Censorship Design
//!
//! Any account can submit operations for any DID. Authorization is verified
//! via cryptographic signatures inside operations, not transaction origin.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

mod validation;
pub mod state;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    use mosaic_did_types::{
        BeforeProofRecord, ContentId, Did, DidOperation,
        KeyId, KeyPurpose, KeyType, Right, SignedDidOperation, SsiOperation,
    };

    use crate::state::{
        OnChainDidState, OnChainVerificationMethod, OnChainRightGrant,
        MAX_KEY_ID_LEN, MAX_PUBLIC_KEY_LEN, MAX_PURPOSES_PER_KEY,
    };

    use crate::validation;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum operations per transaction.
        #[pallet::constant]
        type MaxOperationsPerTx: Get<u32>;

        /// Weight information for extrinsics.
        type WeightInfo: WeightInfo;
    }

    /// Weight information trait for the pallet's extrinsics.
    pub trait WeightInfo {
        fn submit_did_operations(n: u32) -> Weight;
        fn register_before_proof() -> Weight;
    }

    /// Default weight implementation for development/testing.
    impl WeightInfo for () {
        fn submit_did_operations(n: u32) -> Weight {
            Weight::from_parts(10_000 * n as u64, 0)
        }
        fn register_before_proof() -> Weight {
            Weight::from_parts(10_000, 0)
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // ========== STORAGE ==========

    /// Materialized DID document state, keyed by DID.
    /// This is the single source of truth — read and mutated in place.
    #[pallet::storage]
    #[pallet::getter(fn did_state)]
    pub type DidStates<T: Config> = StorageMap<
        _,
        Blake2_128Concat, Did,
        OnChainDidState,
        OptionQuery,
    >;

    /// BeforeProof timestamps.
    #[pallet::storage]
    #[pallet::getter(fn before_proofs)]
    pub type BeforeProofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat, ContentId,
        BeforeProofRecord,
        OptionQuery,
    >;

    // ========== EVENTS ==========

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Atomic SSI batch successfully applied.
        SsiBatchApplied {
            submitter: T::AccountId,
            operation_count: u32,
            block_height: BlockNumberFor<T>,
        },

        /// A key was added to a DID document.
        KeyAdded {
            did: Did,
            key_id: Vec<u8>,
            key_type: KeyType,
            public_key: Vec<u8>,
            purposes: Vec<KeyPurpose>,
            expires_at_height: Option<u32>,
            block_height: BlockNumberFor<T>,
        },

        /// A key was revoked from a DID document.
        KeyRevoked {
            did: Did,
            key_id: Vec<u8>,
            block_height: BlockNumberFor<T>,
        },

        /// A right was granted to a key.
        RightAdded {
            did: Did,
            key_id: Vec<u8>,
            right: Right,
            expires_at_height: Option<u32>,
            block_height: BlockNumberFor<T>,
        },

        /// A right was revoked from a key.
        RightRevoked {
            did: Did,
            key_id: Vec<u8>,
            right: Right,
            block_height: BlockNumberFor<T>,
        },

        /// DID permanently deactivated.
        DidTombstoned {
            did: Did,
            block_height: BlockNumberFor<T>,
        },

        /// BeforeProof timestamp registered.
        BeforeProofRegistered {
            content_id: ContentId,
            block_height: BlockNumberFor<T>,
        },
    }

    // ========== ERRORS ==========

    #[pallet::error]
    pub enum Error<T> {
        /// The DID has been tombstoned and cannot be modified.
        DidTombstoned,
        /// The signer does not have the required right for this operation.
        InsufficientRight,
        /// The signature verification failed.
        InvalidSignature,
        /// The nonce is invalid (replay protection).
        InvalidNonce,
        /// The key being revoked does not exist.
        KeyNotFound,
        /// The key already exists.
        KeyAlreadyExists,
        /// The right is already granted.
        RightAlreadyGranted,
        /// The right does not exist.
        RightNotFound,
        /// Too many operations in a single transaction.
        TooManyOperations,
        /// BeforeProof content already exists.
        BeforeProofAlreadyExists,
        /// The DID does not exist (no keys registered).
        DidNotFound,
        /// Operation references a DID that doesn't match the derived identifier.
        DidMismatch,
        /// The public key is invalid or unsupported.
        InvalidPublicKey,
        /// Storage bounds exceeded (too many keys, rights, or nonces).
        StorageBoundsExceeded,
    }

    // ========== EXTRINSICS ==========

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Submit a batch of SSI operations atomically.
        ///
        /// **Anti-Censorship:** Any account can submit for any DID.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::submit_did_operations(operations.len() as u32))]
        pub fn submit_did_operations(
            origin: OriginFor<T>,
            operations: Vec<SsiOperation>,
        ) -> DispatchResult {
            let submitter = ensure_signed(origin)?;
            let block_height = <frame_system::Pallet<T>>::block_number();
            let height_u32: u32 = Self::block_number_to_u32(block_height);

            ensure!(
                operations.len() <= T::MaxOperationsPerTx::get() as usize,
                Error::<T>::TooManyOperations
            );

            // Phase 1: Validate all operations (dry run — reads state but doesn't mutate)
            for op in &operations {
                match op {
                    SsiOperation::Did(signed_op) => {
                        validation::validate_did_operation::<T>(signed_op, height_u32)?;
                    }
                    SsiOperation::BeforeProof(content_id) => {
                        ensure!(
                            !BeforeProofs::<T>::contains_key(content_id),
                            Error::<T>::BeforeProofAlreadyExists
                        );
                    }
                }
            }

            // Phase 2: Apply all operations (commit — mutates state)
            for op in &operations {
                match op {
                    SsiOperation::Did(signed_op) => {
                        Self::apply_did_operation(signed_op, height_u32, block_height)?;
                    }
                    SsiOperation::BeforeProof(content_id) => {
                        Self::apply_before_proof(*content_id, height_u32, block_height)?;
                    }
                }
            }

            Self::deposit_event(Event::SsiBatchApplied {
                submitter,
                operation_count: operations.len() as u32,
                block_height,
            });

            Ok(())
        }

        /// Register a content hash for proof-of-existence (convenience wrapper).
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::register_before_proof())]
        pub fn register_before_proof(
            origin: OriginFor<T>,
            content_id: ContentId,
        ) -> DispatchResult {
            Self::submit_did_operations(
                origin,
                sp_std::vec![SsiOperation::BeforeProof(content_id)],
            )
        }
    }

    // ========== INTERNAL METHODS ==========

    impl<T: Config> Pallet<T> {
        /// Apply a single signed DID operation by mutating `OnChainDidState` in place.
        fn apply_did_operation(
            signed_op: &SignedDidOperation,
            height_u32: u32,
            block_height: BlockNumberFor<T>,
        ) -> DispatchResult {
            let did = *signed_op.operation.did();

            match &signed_op.operation {
                DidOperation::AddKey { did: _, key_id, key_type, public_key, purposes, expires_at_height } => {
                    let key_id_bytes: BoundedVec<u8, ConstU32<MAX_KEY_ID_LEN>> =
                        key_id.as_bytes().to_vec().try_into()
                            .map_err(|_| Error::<T>::StorageBoundsExceeded)?;
                    let pk_bytes: BoundedVec<u8, ConstU32<MAX_PUBLIC_KEY_LEN>> =
                        public_key.clone().try_into()
                            .map_err(|_| Error::<T>::InvalidPublicKey)?;
                    let bounded_purposes: BoundedVec<KeyPurpose, ConstU32<MAX_PURPOSES_PER_KEY>> =
                        purposes.clone().try_into()
                            .map_err(|_| Error::<T>::StorageBoundsExceeded)?;

                    let vm = OnChainVerificationMethod {
                        id: key_id_bytes,
                        key_type: *key_type,
                        controller: did,
                        public_key: pk_bytes,
                        purposes: bounded_purposes,
                        added_at_height: height_u32,
                        expires_at_height: *expires_at_height,
                        revoked_at_height: None,
                    };

                    DidStates::<T>::try_mutate(did, |maybe_state| -> DispatchResult {
                        let state = maybe_state.get_or_insert_with(|| OnChainDidState::new(did));
                        state.keys.try_push(vm)
                            .map_err(|_| Error::<T>::StorageBoundsExceeded)?;

                        // Update nonce
                        Self::update_nonce(state, &signed_op.signer_key_id, signed_op.nonce)?;
                        Ok(())
                    })?;

                    Self::deposit_event(Event::KeyAdded {
                        did,
                        key_id: key_id.as_bytes().to_vec(),
                        key_type: *key_type,
                        public_key: public_key.clone(),
                        purposes: purposes.clone(),
                        expires_at_height: *expires_at_height,
                        block_height,
                    });
                }

                DidOperation::RevokeKey { did: _, key_id } => {
                    DidStates::<T>::try_mutate(did, |maybe_state| -> DispatchResult {
                        let state = maybe_state.as_mut().ok_or(Error::<T>::DidNotFound)?;
                        let key = state.find_key_mut(key_id.as_bytes())
                            .ok_or(Error::<T>::KeyNotFound)?;
                        key.revoked_at_height = Some(height_u32);
                        Self::update_nonce(state, &signed_op.signer_key_id, signed_op.nonce)?;
                        Ok(())
                    })?;

                    Self::deposit_event(Event::KeyRevoked {
                        did,
                        key_id: key_id.as_bytes().to_vec(),
                        block_height,
                    });
                }

                DidOperation::AddRight { did: _, key_id, right, expires_at_height } => {
                    let key_id_bytes: BoundedVec<u8, ConstU32<MAX_KEY_ID_LEN>> =
                        key_id.as_bytes().to_vec().try_into()
                            .map_err(|_| Error::<T>::StorageBoundsExceeded)?;

                    let grant = OnChainRightGrant {
                        key_id: key_id_bytes,
                        right: *right,
                        granted_at_height: height_u32,
                        expires_at_height: *expires_at_height,
                        revoked_at_height: None,
                    };

                    DidStates::<T>::try_mutate(did, |maybe_state| -> DispatchResult {
                        let state = maybe_state.as_mut().ok_or(Error::<T>::DidNotFound)?;
                        state.rights.try_push(grant)
                            .map_err(|_| Error::<T>::StorageBoundsExceeded)?;
                        Self::update_nonce(state, &signed_op.signer_key_id, signed_op.nonce)?;
                        Ok(())
                    })?;

                    Self::deposit_event(Event::RightAdded {
                        did,
                        key_id: key_id.as_bytes().to_vec(),
                        right: *right,
                        expires_at_height: *expires_at_height,
                        block_height,
                    });
                }

                DidOperation::RevokeRight { did: _, key_id, right } => {
                    DidStates::<T>::try_mutate(did, |maybe_state| -> DispatchResult {
                        let state = maybe_state.as_mut().ok_or(Error::<T>::DidNotFound)?;
                        // Find most recent active grant for this key+right
                        let grant = state.rights.iter_mut().rev().find(|rg| {
                            rg.key_id.as_slice() == key_id.as_bytes()
                                && rg.right == *right
                                && rg.revoked_at_height.is_none()
                        }).ok_or(Error::<T>::RightNotFound)?;
                        grant.revoked_at_height = Some(height_u32);
                        Self::update_nonce(state, &signed_op.signer_key_id, signed_op.nonce)?;
                        Ok(())
                    })?;

                    Self::deposit_event(Event::RightRevoked {
                        did,
                        key_id: key_id.as_bytes().to_vec(),
                        right: *right,
                        block_height,
                    });
                }

                DidOperation::TombstoneDid { did: _ } => {
                    DidStates::<T>::try_mutate(did, |maybe_state| -> DispatchResult {
                        let state = maybe_state.as_mut().ok_or(Error::<T>::DidNotFound)?;
                        state.tombstoned_at_height = Some(height_u32);
                        Self::update_nonce(state, &signed_op.signer_key_id, signed_op.nonce)?;
                        Ok(())
                    })?;

                    Self::deposit_event(Event::DidTombstoned {
                        did,
                        block_height,
                    });
                }
            }

            Ok(())
        }

        /// Apply a BeforeProof timestamp registration.
        fn apply_before_proof(
            content_id: ContentId,
            height_u32: u32,
            block_height: BlockNumberFor<T>,
        ) -> DispatchResult {
            ensure!(
                !BeforeProofs::<T>::contains_key(&content_id),
                Error::<T>::BeforeProofAlreadyExists
            );

            BeforeProofs::<T>::insert(
                &content_id,
                BeforeProofRecord {
                    block_height: height_u32,
                    extrinsic_index: 0, // TODO: get actual extrinsic index
                },
            );

            Self::deposit_event(Event::BeforeProofRegistered {
                content_id,
                block_height,
            });

            Ok(())
        }

        /// Update the nonce for a signer key in the DID state.
        fn update_nonce(
            state: &mut OnChainDidState,
            signer_key_id: &KeyId,
            nonce: u64,
        ) -> DispatchResult {
            let key_bytes: BoundedVec<u8, ConstU32<MAX_KEY_ID_LEN>> =
                signer_key_id.as_bytes().to_vec().try_into()
                    .map_err(|_| Error::<T>::StorageBoundsExceeded)?;

            // Try to update existing entry first
            if let Some(existing) = state.nonces.get_mut(&key_bytes) {
                *existing = nonce;
            } else {
                state.nonces.try_insert(key_bytes, nonce)
                    .map_err(|_| Error::<T>::StorageBoundsExceeded)?;
            }
            Ok(())
        }

        /// Helper to convert BlockNumber to u32.
        pub(crate) fn block_number_to_u32(block_number: BlockNumberFor<T>) -> u32 {
            use sp_runtime::traits::SaturatedConversion;
            block_number.saturated_into()
        }
    }
}
