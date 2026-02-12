//! # Mosaic DID Registry Pallet
//!
//! Substrate pallet implementing the `did:mosaic` DID method for the
//! Mosaic Trust Network. Derived from the IOP Morpheus SSI Stack.
//!
//! ## Overview
//!
//! This pallet provides:
//! - **DID Document management** — Keys, rights, deactivation
//! - **BeforeProof timestamping** — Proof-of-existence for Verifiable Credentials
//!
//! ## Anti-Censorship Design
//!
//! Nodes cannot censor DID/VC requests. The transaction submitter account != DID controller.
//! Authorization is verified via cryptographic signatures inside operations, not transaction origin.
//!
//! ## Extrinsics
//!
//! - `submit_did_operations` — Atomic batch of signed DID operations (the primary entry point)
//! - `register_before_proof` — Timestamp a content hash for proof-of-existence
//!
//! All convenience wrappers (register_did, update_did, tombstone_did) internally route
//! through `submit_did_operations`.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

mod validation;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    use mosaic_did_types::{
        BeforeProofRecord, ContentId, Did, DidOperation, DidOperationType,
        KeyId, KeyPurpose, KeyType, Right, SignedDidOperation, SsiOperation,
    };

    use crate::validation;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum operations per transaction.
        #[pallet::constant]
        type MaxOperationsPerTx: Get<u32>;

        /// Maximum operations stored per block per DID.
        #[pallet::constant]
        type MaxOpsPerBlock: Get<u32>;

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

    /// All operations for a DID, indexed by block height.
    /// This is the append-only audit log from which DID Documents are reconstructed.
    #[pallet::storage]
    #[pallet::getter(fn did_operations)]
    pub type DidOperations<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, Did,
        Blake2_128Concat, BlockNumberFor<T>,
        BoundedVec<DidOperation, T::MaxOpsPerBlock>,
        ValueQuery,
    >;

    /// Replay protection nonce per DID per key.
    #[pallet::storage]
    #[pallet::getter(fn did_key_nonce)]
    pub type DidKeyNonce<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, Did,
        Blake2_128Concat, Vec<u8>, // KeyId bytes
        u64,
        ValueQuery,
    >;

    /// Tombstoned DIDs (block at which deactivated).
    #[pallet::storage]
    #[pallet::getter(fn tombstoned_dids)]
    pub type TombstonedDids<T: Config> = StorageMap<
        _,
        Blake2_128Concat, Did,
        BlockNumberFor<T>,
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

    /// DID existence tracker — records which DIDs have been created.
    /// The value is the block height at which the DID was first created.
    #[pallet::storage]
    #[pallet::getter(fn did_created_at)]
    pub type DidCreatedAt<T: Config> = StorageMap<
        _,
        Blake2_128Concat, Did,
        BlockNumberFor<T>,
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

        /// Individual DID operation applied (emitted per-operation within batch).
        DidOperationApplied {
            did: Did,
            operation_type: DidOperationType,
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
    }

    // ========== EXTRINSICS ==========

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Submit a batch of SSI operations (DID operations and/or BeforeProof timestamps).
        ///
        /// **Atomicity Guarantee:** All operations succeed or all fail. No partial state changes.
        ///
        /// **Anti-Censorship:** Any account can submit for any DID; authorization is verified
        /// via cryptographic signatures inside each `SignedDidOperation`.
        ///
        /// # Parameters
        /// - `origin`: Transaction sender (pays fees; NOT necessarily the DID controller)
        /// - `operations`: Batch of SSI operations
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::submit_did_operations(operations.len() as u32))]
        pub fn submit_did_operations(
            origin: OriginFor<T>,
            operations: Vec<SsiOperation>,
        ) -> DispatchResult {
            let submitter = ensure_signed(origin)?;
            let block_height = <frame_system::Pallet<T>>::block_number();

            ensure!(
                operations.len() <= T::MaxOperationsPerTx::get() as usize,
                Error::<T>::TooManyOperations
            );

            // Phase 1: Validate all operations (dry run)
            for op in &operations {
                match op {
                    SsiOperation::Did(signed_op) => {
                        validation::validate_did_operation::<T>(signed_op, block_height)?;
                    }
                    SsiOperation::BeforeProof(content_id) => {
                        ensure!(
                            !BeforeProofs::<T>::contains_key(content_id),
                            Error::<T>::BeforeProofAlreadyExists
                        );
                    }
                }
            }

            // Phase 2: Apply all operations (commit)
            for op in &operations {
                match op {
                    SsiOperation::Did(signed_op) => {
                        Self::apply_did_operation(signed_op, block_height)?;
                    }
                    SsiOperation::BeforeProof(content_id) => {
                        Self::apply_before_proof(*content_id, block_height)?;
                    }
                }
            }

            // Emit batch event
            Self::deposit_event(Event::SsiBatchApplied {
                submitter,
                operation_count: operations.len() as u32,
                block_height,
            });

            Ok(())
        }

        /// Register a content hash for proof-of-existence (convenience wrapper).
        ///
        /// Before issuing a Verifiable Credential, the issuer registers its hash.
        /// This proves the document existed at block height N.
        ///
        /// # Parameters
        /// - `origin`: Transaction sender
        /// - `content_id`: BLAKE3 hash of the content being timestamped
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::register_before_proof())]
        pub fn register_before_proof(
            origin: OriginFor<T>,
            content_id: ContentId,
        ) -> DispatchResult {
            let submitter = ensure_signed(origin.clone())?;
            let block_height = <frame_system::Pallet<T>>::block_number();

            // Route through submit_did_operations for consistency
            Self::submit_did_operations(
                origin,
                sp_std::vec![SsiOperation::BeforeProof(content_id)],
            )
        }
    }

    // ========== INTERNAL METHODS ==========

    impl<T: Config> Pallet<T> {
        /// Apply a single signed DID operation to storage.
        fn apply_did_operation(
            signed_op: &SignedDidOperation,
            block_height: BlockNumberFor<T>,
        ) -> DispatchResult {
            let did = signed_op.operation.did();
            let op_type = DidOperationType::from(&signed_op.operation);

            // Store the operation in the audit log
            DidOperations::<T>::try_mutate(
                did,
                block_height,
                |ops| -> DispatchResult {
                    ops.try_push(signed_op.operation.clone())
                        .map_err(|_| Error::<T>::TooManyOperations)?;
                    Ok(())
                },
            )?;

            // Update nonce
            DidKeyNonce::<T>::mutate(
                did,
                signed_op.signer_key_id.as_bytes().to_vec(),
                |nonce| {
                    *nonce = signed_op.nonce;
                },
            );

            // Handle DID creation (implicit on first AddKey)
            if let DidOperation::AddKey { .. } = &signed_op.operation {
                if !DidCreatedAt::<T>::contains_key(did) {
                    DidCreatedAt::<T>::insert(did, block_height);
                }
            }

            // Handle tombstoning
            if let DidOperation::TombstoneDid { did } = &signed_op.operation {
                TombstonedDids::<T>::insert(did, block_height);
                Self::deposit_event(Event::DidTombstoned {
                    did: *did,
                    block_height,
                });
            }

            // Emit per-operation event
            Self::deposit_event(Event::DidOperationApplied {
                did: *did,
                operation_type: op_type,
                block_height,
            });

            Ok(())
        }

        /// Apply a BeforeProof timestamp registration.
        fn apply_before_proof(
            content_id: ContentId,
            block_height: BlockNumberFor<T>,
        ) -> DispatchResult {
            ensure!(
                !BeforeProofs::<T>::contains_key(&content_id),
                Error::<T>::BeforeProofAlreadyExists
            );

            BeforeProofs::<T>::insert(
                &content_id,
                BeforeProofRecord {
                    block_height: Self::block_number_to_u32(block_height),
                    extrinsic_index: 0, // TODO: get actual extrinsic index
                },
            );

            Self::deposit_event(Event::BeforeProofRegistered {
                content_id,
                block_height,
            });

            Ok(())
        }

        /// Helper to convert BlockNumber to u32.
        fn block_number_to_u32(block_number: BlockNumberFor<T>) -> u32 {
            use sp_runtime::traits::SaturatedConversion;
            block_number.saturated_into()
        }
    }
}
