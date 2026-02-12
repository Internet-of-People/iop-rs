//! Unit tests for the Mosaic DID pallet (v2 — materialized state).

#[cfg(test)]
mod unit_tests {
    use mosaic_did_types::*;

    // ===== Type-level tests (no runtime needed) =====

    #[test]
    fn did_creation_from_public_key() {
        let pk = b"ed25519-public-key-32-bytes-long";
        let did = Did::from_public_key(pk);
        let did2 = Did::from_public_key(pk);
        assert_eq!(did, did2);

        let did3 = Did::from_public_key(b"different-key-different-did-now!");
        assert_ne!(did, did3);
    }

    #[test]
    fn did_string_roundtrip() {
        let pk = b"ed25519-public-key-32-bytes-long";
        let did = Did::from_public_key(pk);
        let did_str = did.to_did_string();
        assert!(did_str.starts_with("did:mosaic:z"));
        let parsed = Did::from_did_string(&did_str).unwrap();
        assert_eq!(did, parsed);
    }

    #[test]
    fn content_id_deterministic() {
        let content = b"Invoice #2026-001 from ELMU";
        let cid1 = ContentId::from_content(content);
        let cid2 = ContentId::from_content(content);
        assert_eq!(cid1, cid2);
    }

    #[test]
    fn operation_creates_correct_types() {
        let pk = b"ed25519-public-key-32-bytes-long";
        let did = Did::from_public_key(pk);
        let key_id = KeyId::from_str_id("key-1").unwrap();

        let add_key = DidOperation::AddKey {
            did,
            key_id,
            key_type: KeyType::Ed25519,
            public_key: pk.to_vec(),
            purposes: vec![KeyPurpose::Authentication, KeyPurpose::AssertionMethod],
            expires_at_height: None,
        };

        assert_eq!(*add_key.did(), did);
        assert_eq!(add_key.required_right(), Right::Update);
        assert_eq!(DidOperationType::from(&add_key), DidOperationType::AddKey);
    }

    #[test]
    fn rights_system() {
        let pk = b"ed25519-public-key-32-bytes-long";
        let did = Did::from_public_key(pk);
        let key_id = KeyId::from_str_id("key-1").unwrap();

        let add_right = DidOperation::AddRight {
            did,
            key_id: key_id.clone(),
            right: Right::Issue,
            expires_at_height: None,
        };
        assert_eq!(add_right.required_right(), Right::Delegate);

        let revoke_right = DidOperation::RevokeRight {
            did,
            key_id,
            right: Right::Issue,
        };
        assert_eq!(revoke_right.required_right(), Right::Delegate);
    }

    #[test]
    fn tombstone_requires_update_right() {
        let did = Did::from_public_key(b"test-key-for-tombstone-testing!!");
        let tombstone = DidOperation::TombstoneDid { did };
        assert_eq!(tombstone.required_right(), Right::Update);
    }

    #[test]
    fn before_proof_record_structure() {
        let record = BeforeProofRecord {
            block_height: 12345,
            extrinsic_index: 3,
        };
        assert_eq!(record.block_height, 12345);
        assert_eq!(record.extrinsic_index, 3);
    }

    #[test]
    fn multi_signature_types() {
        let ed_sig = MultiSignature::ed25519([42u8; 64]);
        assert_eq!(ed_sig.algorithm(), "Ed25519");
        assert_eq!(ed_sig.as_bytes().len(), 64);

        let secp_sig = MultiSignature::secp256k1([43u8; 65]);
        assert_eq!(secp_sig.algorithm(), "secp256k1");
        assert_eq!(secp_sig.as_bytes().len(), 65);
    }

    #[test]
    fn signed_operation_construction() {
        let pk = b"ed25519-public-key-32-bytes-long";
        let did = Did::from_public_key(pk);
        let key_id = KeyId::from_str_id("key-1").unwrap();

        let signed_op = SignedDidOperation {
            operation: DidOperation::AddKey {
                did,
                key_id: key_id.clone(),
                key_type: KeyType::Ed25519,
                public_key: pk.to_vec(),
                purposes: vec![KeyPurpose::Authentication],
                expires_at_height: None,
            },
            signer_key_id: key_id,
            signature: MultiSignature::ed25519([1u8; 64]),
            nonce: 1,
        };

        assert_eq!(*signed_op.operation.did(), did);
        assert_eq!(signed_op.nonce, 1);
    }

    #[test]
    fn ssi_operation_variants() {
        let pk = b"ssi-op-test-key-32-bytes-long!!!";
        let did = Did::from_public_key(pk);
        let key_id = KeyId::from_str_id("key-1").unwrap();

        let did_op = SsiOperation::Did(SignedDidOperation {
            operation: DidOperation::AddKey {
                did,
                key_id: key_id.clone(),
                key_type: KeyType::Ed25519,
                public_key: pk.to_vec(),
                purposes: vec![KeyPurpose::Authentication],
                expires_at_height: None,
            },
            signer_key_id: key_id,
            signature: MultiSignature::ed25519([1u8; 64]),
            nonce: 1,
        });

        let before_proof_op = SsiOperation::BeforeProof(
            ContentId::from_content(b"test document content")
        );

        match &did_op {
            SsiOperation::Did(op) => assert_eq!(*op.operation.did(), did),
            _ => panic!("Expected Did variant"),
        }

        match &before_proof_op {
            SsiOperation::BeforeProof(cid) => assert_ne!(*cid, ContentId::default()),
            _ => panic!("Expected BeforeProof variant"),
        }
    }

    // ===== On-chain state tests (using OnChainDidState directly) =====

    mod on_chain_state_tests {
        use mosaic_did_types::*;
        use crate::state::*;
        use frame_support::BoundedVec;
        use frame_support::pallet_prelude::ConstU32;

        fn make_key_id(s: &str) -> BoundedVec<u8, ConstU32<MAX_KEY_ID_LEN>> {
            s.as_bytes().to_vec().try_into().unwrap()
        }

        fn make_test_state() -> OnChainDidState {
            let pk = b"ed25519-public-key-32-bytes-long";
            let did = Did::from_public_key(pk);
            let mut state = OnChainDidState::new(did);

            let vm = OnChainVerificationMethod {
                id: make_key_id("key-1"),
                key_type: KeyType::Ed25519,
                controller: did,
                public_key: pk.to_vec().try_into().unwrap(),
                purposes: vec![KeyPurpose::Authentication, KeyPurpose::AssertionMethod]
                    .try_into().unwrap(),
                added_at_height: 1,
                expires_at_height: None,
                revoked_at_height: None,
            };
            state.keys.try_push(vm).unwrap();
            state
        }

        #[test]
        fn new_state_is_empty() {
            let state = OnChainDidState::new(Did::default());
            assert!(!state.exists());
            assert!(!state.is_tombstoned_at(0));
        }

        #[test]
        fn state_with_key_exists() {
            let state = make_test_state();
            assert!(state.exists());
        }

        #[test]
        fn key_validity_at_height() {
            let vm = OnChainVerificationMethod {
                id: make_key_id("key-1"),
                key_type: KeyType::Ed25519,
                controller: Did::default(),
                public_key: vec![0u8; 32].try_into().unwrap(),
                purposes: BoundedVec::default(),
                added_at_height: 10,
                expires_at_height: Some(100),
                revoked_at_height: None,
            };

            assert!(!vm.is_valid_at(9));
            assert!(vm.is_valid_at(10));
            assert!(vm.is_valid_at(50));
            assert!(vm.is_valid_at(99));
            assert!(!vm.is_valid_at(100));
        }

        #[test]
        fn key_revoked_at_height() {
            let vm = OnChainVerificationMethod {
                id: make_key_id("key-1"),
                key_type: KeyType::Ed25519,
                controller: Did::default(),
                public_key: vec![0u8; 32].try_into().unwrap(),
                purposes: BoundedVec::default(),
                added_at_height: 10,
                expires_at_height: None,
                revoked_at_height: Some(50),
            };

            assert!(vm.is_valid_at(10));
            assert!(vm.is_valid_at(49));
            assert!(!vm.is_valid_at(50));
        }

        #[test]
        fn tombstoned_did() {
            let mut state = make_test_state();
            state.tombstoned_at_height = Some(50);

            assert!(!state.is_tombstoned_at(49));
            assert!(state.is_tombstoned_at(50));
            assert!(state.is_tombstoned_at(100));
        }

        #[test]
        fn initial_key_has_implicit_rights() {
            let state = make_test_state();
            let key_id = b"key-1";

            assert!(state.has_right_at(key_id, Right::Update, 1));
            assert!(state.has_right_at(key_id, Right::Impersonate, 1));
            assert!(!state.has_right_at(key_id, Right::Delegate, 1));
            assert!(!state.has_right_at(key_id, Right::Issue, 1));
        }

        #[test]
        fn explicit_right_grant() {
            let mut state = make_test_state();
            let key_id_bytes = make_key_id("key-1");

            let grant = OnChainRightGrant {
                key_id: key_id_bytes,
                right: Right::Delegate,
                granted_at_height: 10,
                expires_at_height: Some(100),
                revoked_at_height: None,
            };
            state.rights.try_push(grant).unwrap();

            assert!(!state.has_right_at(b"key-1", Right::Delegate, 9));
            assert!(state.has_right_at(b"key-1", Right::Delegate, 10));
            assert!(state.has_right_at(b"key-1", Right::Delegate, 99));
            assert!(!state.has_right_at(b"key-1", Right::Delegate, 100));
        }

        #[test]
        fn nonce_management() {
            let state = make_test_state();
            assert_eq!(state.nonce_for_key(b"key-1"), 0);
            assert_eq!(state.nonce_for_key(b"key-nonexistent"), 0);
        }

        #[test]
        fn key_rotation_flow() {
            let did = Did::from_public_key(b"rotation-test-key-32-bytes!!!!!!");
            let mut state = OnChainDidState::new(did);

            // Add old key at height 1
            state.keys.try_push(OnChainVerificationMethod {
                id: make_key_id("key-old"),
                key_type: KeyType::Ed25519,
                controller: did,
                public_key: vec![1u8; 32].try_into().unwrap(),
                purposes: vec![KeyPurpose::Authentication, KeyPurpose::AssertionMethod]
                    .try_into().unwrap(),
                added_at_height: 1,
                expires_at_height: None,
                revoked_at_height: None,
            }).unwrap();

            // Add new key at height 50
            state.keys.try_push(OnChainVerificationMethod {
                id: make_key_id("key-new"),
                key_type: KeyType::Ed25519,
                controller: did,
                public_key: vec![2u8; 32].try_into().unwrap(),
                purposes: vec![KeyPurpose::Authentication, KeyPurpose::AssertionMethod]
                    .try_into().unwrap(),
                added_at_height: 50,
                expires_at_height: None,
                revoked_at_height: None,
            }).unwrap();

            // Revoke old key at height 50
            state.keys[0].revoked_at_height = Some(50);

            // Before rotation: only old key valid
            assert_eq!(state.valid_keys_at(49).len(), 1);
            assert_eq!(state.valid_keys_at(49)[0].id.as_slice(), b"key-old");

            // After rotation: only new key valid
            assert_eq!(state.valid_keys_at(50).len(), 1);
            assert_eq!(state.valid_keys_at(50)[0].id.as_slice(), b"key-new");
        }
    }

    // ===== Off-chain document state tests (DidDocumentState for RPC/SDK) =====

    #[test]
    fn did_document_state_reconstruction() {
        use mosaic_did_types::document::{DidDocumentState, VerificationMethod, RightGrant};

        let pk = b"ed25519-public-key-32-bytes-long";
        let did = Did::from_public_key(pk);
        let key_id = KeyId::from_str_id("key-1").unwrap();

        let mut state = DidDocumentState::new(did);
        assert!(!state.exists());

        state.keys.push(VerificationMethod {
            id: key_id.clone(),
            key_type: KeyType::Ed25519,
            controller: did,
            public_key: pk.to_vec(),
            purposes: vec![KeyPurpose::Authentication, KeyPurpose::AssertionMethod],
            added_at_height: 1,
            expires_at_height: None,
            revoked_at_height: None,
        });

        assert!(state.exists());
        assert_eq!(state.valid_keys_at(1).len(), 1);

        assert!(state.has_right_at(&key_id, Right::Update, 1));
        assert!(state.has_right_at(&key_id, Right::Impersonate, 1));
        assert!(!state.has_right_at(&key_id, Right::Delegate, 1));

        state.rights.push(RightGrant {
            key_id: key_id.clone(),
            right: Right::Delegate,
            granted_at_height: 5,
            expires_at_height: Some(100),
            revoked_at_height: None,
        });

        assert!(!state.has_right_at(&key_id, Right::Delegate, 4));
        assert!(state.has_right_at(&key_id, Right::Delegate, 5));
        assert!(state.has_right_at(&key_id, Right::Delegate, 99));
        assert!(!state.has_right_at(&key_id, Right::Delegate, 100));
    }

    #[test]
    fn did_document_tombstone_flow() {
        use mosaic_did_types::document::{DidDocumentState, VerificationMethod};

        let did = Did::from_public_key(b"tombstone-test-key-32-bytes!!!!!");
        let mut state = DidDocumentState::new(did);

        state.keys.push(VerificationMethod {
            id: KeyId::from_str_id("key-1").unwrap(),
            key_type: KeyType::Ed25519,
            controller: did,
            public_key: vec![0u8; 32],
            purposes: vec![KeyPurpose::Authentication],
            added_at_height: 1,
            expires_at_height: None,
            revoked_at_height: None,
        });

        assert!(!state.is_tombstoned_at(50));
        state.tombstoned_at_height = Some(50);

        assert!(!state.is_tombstoned_at(49));
        assert!(state.is_tombstoned_at(50));
        assert!(state.is_tombstoned_at(100));

        let doc = state.to_did_document_at(100);
        assert!(doc.deactivated);
    }

    #[test]
    fn w3c_did_document_structure() {
        use mosaic_did_types::document::{DidDocumentState, VerificationMethod};

        let pk = b"w3c-test-key-for-did-doc-struct!";
        let did = Did::from_public_key(pk);

        let mut state = DidDocumentState::new(did);
        state.keys.push(VerificationMethod {
            id: KeyId::from_str_id("key-1").unwrap(),
            key_type: KeyType::Ed25519,
            controller: did,
            public_key: pk.to_vec(),
            purposes: vec![
                KeyPurpose::Authentication,
                KeyPurpose::AssertionMethod,
                KeyPurpose::CapabilityDelegation,
            ],
            added_at_height: 1,
            expires_at_height: None,
            revoked_at_height: None,
        });

        let doc = state.to_did_document_at(1);

        assert_eq!(doc.context[0], "https://www.w3.org/ns/did/v1");
        assert_eq!(doc.context[1], "https://mosaic.network/ns/did/v1");
        assert!(doc.id.starts_with("did:mosaic:z"));
        assert_eq!(doc.verification_method.len(), 1);
        assert_eq!(doc.verification_method[0].type_, "Ed25519VerificationKey2020");
        assert!(doc.verification_method[0].public_key_multibase.starts_with('z'));
        assert_eq!(doc.authentication.len(), 1);
        assert_eq!(doc.assertion_method.len(), 1);
        assert_eq!(doc.capability_delegation.len(), 1);
        assert!(doc.capability_invocation.is_empty());
        assert!(!doc.deactivated);

        let json = serde_json::to_string_pretty(&doc).unwrap();
        assert!(json.contains("\"@context\""));
        assert!(json.contains("\"verificationMethod\""));
        assert!(json.contains("\"Ed25519VerificationKey2020\""));
    }

    #[test]
    fn eurosign_workflow_types() {
        let signer_pk = b"eurosign-signer-pk-32-bytes-ed25";
        let signer_did = Did::from_public_key(signer_pk);
        let key_id = KeyId::from_str_id("key-signing").unwrap();

        let create_op = SsiOperation::Did(SignedDidOperation {
            operation: DidOperation::AddKey {
                did: signer_did,
                key_id: key_id.clone(),
                key_type: KeyType::Ed25519,
                public_key: signer_pk.to_vec(),
                purposes: vec![KeyPurpose::Authentication, KeyPurpose::AssertionMethod],
                expires_at_height: None,
            },
            signer_key_id: key_id,
            signature: MultiSignature::ed25519([1u8; 64]),
            nonce: 1,
        });

        let document_hash = ContentId::from_content(b"Signed contract PDF content...");
        let before_proof_op = SsiOperation::BeforeProof(document_hash);

        let batch = vec![create_op, before_proof_op];
        assert_eq!(batch.len(), 2);
    }
}
