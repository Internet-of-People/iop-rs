use super::*;

pub type Version = u64;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct TxnStatus {
    pub version_before_txn: Version,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct State {
    corrupted: bool,
    root: Domain,
    last_seen_height: BlockHeight,
    version_of_first_undo_operation: Version,
    undo_operations: Vec<UndoOperation>,
    nonces: HashMap<MPublicKey, Nonce>,
    txn_statuses: HashMap<String, TxnStatus>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            corrupted: false,
            root: Domain::new_root(),
            last_seen_height: Default::default(),
            version_of_first_undo_operation: Default::default(),
            undo_operations: Default::default(),
            nonces: Default::default(),
            txn_statuses: Default::default(),
        }
    }
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_corrupted(&self) -> bool {
        self.corrupted
    }

    pub fn ensure_not_corrupted(&self) -> Result<()> {
        if self.corrupted {
            bail!("Coeus state is corrupt. All incoming changes will be ignored.");
        }
        Ok(())
    }

    pub fn root(&self) -> &Domain {
        &self.root
    }

    pub fn last_seen_height(&self) -> BlockHeight {
        self.last_seen_height
    }

    pub(crate) fn set_last_seen_height(&mut self, height: BlockHeight) {
        self.last_seen_height = height;
    }

    pub fn version(&self) -> Version {
        self.undo_operations.len() as Version + self.version_of_first_undo_operation
    }

    pub fn block_applying(&mut self, height: BlockHeight) -> Result<()> {
        self.ensure_not_corrupted()?;
        self.apply_operations(vec![SystemOperation::start_block(height)]).map(|_| ())
    }

    pub fn block_reverted(&mut self, height: BlockHeight) -> Result<()> {
        self.ensure_not_corrupted()?;
        let height_before_revert = self.last_seen_height;
        self.set_corrupted_on_err(|state| {
            ensure!(
                height_before_revert == height,
                "Cannot revert block at height {}, because currently the state is at height {}",
                height,
                height_before_revert,
            );
            state.undo_operation(state.version() - 1)?;
            ensure!(height_before_revert > state.last_seen_height, "Cannot revert block at height {}, because the operation undone was not reducing the block height.", height);
            Ok(())
        })
    }

    pub fn apply_transaction(&mut self, txid: &str, asset: CoeusAsset) -> Result<()> {
        self.ensure_not_corrupted()?;

        let version_before_txn = self.version();

        for bundle in asset.bundles {
            if let Err(e) = self.apply_signed_bundle(bundle) {
                self.undo_operations(version_before_txn)?;
                self.txn_statuses
                    .insert(txid.to_owned(), TxnStatus { version_before_txn, success: false });
                return Err(e);
            }
        }
        self.txn_statuses.insert(txid.to_owned(), TxnStatus { version_before_txn, success: true });
        Ok(())
    }

    pub fn revert_transaction(&mut self, txid: &str, asset: CoeusAsset) -> Result<()> {
        self.ensure_not_corrupted()?;
        self.set_corrupted_on_err(|state| match state.txn_statuses.remove(txid) {
            None => {
                bail!("Transaction has not been applied previously.");
            }
            Some(status) => {
                let current_version = state.version();
                let version_before_txn = status.version_before_txn;
                let operation_count: usize =
                    asset.bundles.iter().map(|signed| signed.bundle.operations.len()).sum();
                ensure!(
                    version_before_txn + operation_count as u64 == current_version,
                    "Number of operations in transaction do not match our previous records."
                );
                state.undo_operations(version_before_txn)?;
                return Ok(());
            }
        })
    }

    pub(crate) fn apply_signed_bundle(&mut self, ops: SignedBundle) -> Result<Version> {
        ensure!(ops.verify(), "Invalid signature or the operations were tampered with");
        self.authorize_operations(&ops.bundle.operations, &ops.public_key)?;
        self.apply_nonced_bundle(ops.bundle, ops.public_key)
    }

    fn authorize_operations(&mut self, ops: &[UserOperation], pk: &MPublicKey) -> Result<()> {
        ops.iter().try_for_each(|op| op.validate_auth(self, pk))
    }

    pub fn validate_domain_owner(&self, name: &DomainName, pk: &MPublicKey) -> Result<()> {
        let domain = self.domain(name)?;
        domain.owner().validate_impersonation(pk)
    }

    fn apply_nonced_bundle(&mut self, bundle: NoncedBundle, pk: MPublicKey) -> Result<Version> {
        let old_nonce = self.nonces.get(&pk).copied().unwrap_or_default();
        ensure!(
            bundle.nonce == old_nonce + 1,
            "Invalid nonce {}, expected {}",
            bundle.nonce,
            old_nonce + 1
        );

        let version = self.apply_operations(bundle.operations)?;

        self.nonces.insert(pk, old_nonce + 1);

        Ok(version)
    }

    pub(crate) fn apply_operations(&mut self, mut ops: Vec<impl Command>) -> Result<Version> {
        let mut undos = vec![];
        let res = ops.drain(..).try_fold(&mut undos, |undos, op| {
            let undo = op.execute(self)?;
            undos.push(undo);
            Ok(undos)
        });
        match res {
            Err(e) => {
                // TODO Corrupt state if next line fails
                undos.drain(..).rev().try_for_each(|op| op.execute(self))?;
                Err(e)
            }
            Ok(_) => {
                self.undo_operations.extend_from_slice(&undos);
                Ok(self.version())
            }
        }
    }

    pub fn domain(&self, name: &DomainName) -> Result<&Domain> {
        name.iter()
            .try_fold(&self.root, |dom, e| dom.child(e))
            .with_context(|| format!("Cannot find domain with name {}", name))
    }

    pub(crate) fn domain_mut(&mut self, name: &DomainName) -> Result<&mut Domain> {
        name.iter()
            .try_fold(&mut self.root, |dom, e| dom.child_mut(e))
            .with_context(|| format!("Cannot find domain with name {}", name))
    }

    pub fn nonce(&self, pk: &MPublicKey) -> Nonce {
        self.nonces.get(pk).copied().unwrap_or(0)
    }

    pub fn get_txn_status(&self, txid: &str) -> Result<&TxnStatus> {
        self.txn_statuses.get(txid).with_context(|| format!("Cannot find txn with id {}", txid))
    }

    pub fn resolve_data(&self, name: &DomainName) -> Result<&DynamicContent> {
        let domain = name.iter().try_fold(self.root(), |dom, edge| {
            dom.child(edge)
                .with_context(|| format!("Edge {} was not found for domain {}", edge, name))
                .and_then(|child| {
                    if child.is_expired_at(self.last_seen_height) {
                        bail!("Edge {} in domain {} expired", edge, name)
                    } else {
                        Ok(child)
                    }
                })
        })?;
        Ok(domain.data())
    }

    pub fn validate_subtree_policies(&self, domain_name: &DomainName) -> Result<()> {
        let domain_after_op = self.domain(domain_name)?;
        let mut policy_domain = self.root();
        policy_domain.validate_subtree_policies(self, domain_after_op)?;

        for edge in domain_name.edges() {
            policy_domain = policy_domain
                .child(edge)
                .expect("Implementation error: validating nonexisting domain data");
            policy_domain.validate_subtree_policies(self, domain_after_op)?;
        }

        Ok(())
    }

    fn set_corrupted_on_err<R>(&mut self, func: impl FnOnce(&mut Self) -> Result<R>) -> Result<R> {
        match func(self) {
            Err(e) => {
                self.corrupted = true;
                Err(e)
            }
            Ok(r) => Ok(r),
        }
    }

    fn undo_operations(&mut self, to_version: Version) -> Result<()> {
        for version in (to_version..self.version()).rev() {
            self.undo_operation(version)?;
        }
        Ok(())
    }

    fn undo_operation(&mut self, to_version: Version) -> Result<()> {
        self.set_corrupted_on_err(|state| {
            let undo_op = state
                .undo_operations
                .pop()
                .with_context(|| format!("Cannot undo to version {} anymore", to_version))?;
            undo_op.execute(state)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    trait StateExt {
        fn apply_operation(&mut self, op: impl Command) -> Result<Version>;
    }

    impl StateExt for State {
        fn apply_operation(&mut self, op: impl Command) -> Result<Version> {
            self.apply_operations(vec![op])
        }
    }

    #[test]
    fn empty_state() {
        let state = State::new();

        let root = state.root();
        assert!(root.name().is_root());
        assert_eq!(root.owner(), &Principal::system());

        let child_names = root.child_names();
        let schema_edge = Edge::new("schema").unwrap();
        assert_eq!(child_names.len(), 1);
        assert_eq!(child_names[0], &schema_edge);

        let schema_domain = root.child(&schema_edge).unwrap();
        assert!(schema_domain.child_names().is_empty());
        assert!(!schema_domain.name().is_root());
        assert_eq!(schema_domain.owner(), &Principal::system());

        assert!(state.undo_operations.is_empty());
        assert_eq!(state.last_seen_height(), 0);
        assert_eq!(state.version(), 0);
    }

    fn name_resolves_to(
        state: &State, name: &DomainName, expected_data: &DynamicContent,
    ) -> Result<()> {
        let data = state.resolve_data(name)?;
        assert_eq!(data, expected_data);
        Ok(())
    }

    fn name_does_not_resolve(state: &State, domain_name: &DomainName) -> Result<()> {
        let last_edge = domain_name.last_edge().unwrap();
        let err_msg = format!("Edge {} in domain {} expired", last_edge, domain_name);

        let err = state.resolve_data(domain_name).unwrap_err();
        assert_eq!(err.to_string(), err_msg);
        Ok(())
    }

    #[test]
    fn expiry() {
        let mut state = State::new();

        let domain_name = domain_name(".schema.decentralizers");
        let domain_data = json!({"data": "We're gonna rule the world"});
        state
            .apply_operation(UserOperation::register(
                domain_name.clone(),
                domain_owner(),
                no_policies(),
                Default::default(),
                domain_data.clone(),
                42,
            ))
            .unwrap();

        name_resolves_to(&state, &domain_name, &domain_data).unwrap();

        state.block_applying(41).unwrap();
        name_resolves_to(&state, &domain_name, &domain_data).unwrap();

        state.block_applying(42).unwrap();
        name_does_not_resolve(&state, &domain_name).unwrap();

        state.apply_operation(UserOperation::renew(domain_name.clone(), 43)).unwrap();
        name_resolves_to(&state, &domain_name, &domain_data).unwrap();

        state.block_applying(1234567890).unwrap();
        name_does_not_resolve(&state, &domain_name).unwrap();
        assert_eq!(state.version(), 5);

        state.block_reverted(1234567890).unwrap(); // undo block 1234567890
        name_resolves_to(&state, &domain_name, &domain_data).unwrap();

        state.undo_operation(3).unwrap(); // undo renew
        name_does_not_resolve(&state, &domain_name).unwrap();

        state.block_reverted(42).unwrap(); // undo block 42
        name_resolves_to(&state, &domain_name, &domain_data).unwrap();
    }

    fn no_policies() -> SubtreePolicies {
        SubtreePolicies::new() // json!({})
    }

    fn data(data: &str) -> serde_json::Value {
        json!({ "data": data })
    }

    fn domain_name(name: &str) -> DomainName {
        name.parse().unwrap()
    }

    fn domain_owner_pk() -> MPublicKey {
        let sk = crate::signed::test::ark_sk();
        sk.public_key()
    }

    fn domain_owner() -> Principal {
        Principal::PublicKey(domain_owner_pk())
    }

    fn sign_ops(ops: NoncedBundle) -> SignedBundle {
        let sk = crate::signed::test::ark_sk_from(
            "scout try doll stuff cake welcome random taste load town clerk ostrich",
        );
        ops.sign(&sk).unwrap()
    }

    fn sign_ops_by_wrong_key(ops: NoncedBundle) -> SignedBundle {
        let sk = crate::signed::test::ark_sk_from(
            "not scout try doll stuff cake welcome random taste load town clerk ostrich",
        );
        ops.sign(&sk).unwrap()
    }

    fn check_domain_exists(
        state: &State, name: &DomainName, expected_data: &serde_json::Value,
        expected_owner: &Principal,
    ) {
        let edge = name.last_edge().unwrap();
        let parent = state.domain(&name.parent().unwrap()).unwrap();
        assert!(parent.child_names().contains(&edge));

        let registered_domain = parent.child(edge).unwrap();
        assert_eq!(registered_domain.name(), name);
        assert_eq!(registered_domain.owner(), expected_owner);
        assert_eq!(registered_domain.subtree_policies(), &no_policies());
        assert_eq!(registered_domain.data(), expected_data);
        assert!(registered_domain.child_names().is_empty());
    }

    fn check_domain_missing(state: &State, name: &DomainName) {
        let edge = name.last_edge().unwrap();
        let parent = state.domain(&name.parent().unwrap()).unwrap();
        assert!(!parent.child_names().contains(&edge));
    }

    fn execute_tld_register_system_domain() -> State {
        let mut state = State::new();

        let register_operation = UserOperation::register(
            domain_name(".wallet"),
            Principal::system(),
            no_policies(),
            RegistrationPolicy::Owner,
            data("a"),
            ExpirationPolicy::YEAR,
        );
        let version = state.apply_operation(register_operation).unwrap();

        assert_eq!(version, 1);

        state
    }

    #[test]
    fn execute_checks_registration_policy() {
        let mut state = execute_tld_register_system_domain();

        let pk = "pezDj6ea4tVfNRUTMyssVDepAAzPW67Fe3yHtuHL6ZNtcfJ".parse().unwrap();
        let register_operation = UserOperation::register(
            domain_name(".wallet.wigy"),
            Principal::public_key(&pk),
            no_policies(),
            Default::default(),
            data("a"),
            ExpirationPolicy::YEAR,
        );
        let err = state.apply_operation(register_operation).unwrap_err();

        assert_eq!(err.to_string(), "Only system can register a child of .wallet");
    }

    #[test]
    fn signed_register_cannot_impersonate_system() {
        let mut state = State::new();
        let sk = signed::test::ark_sk();

        let register_operation = UserOperation::register(
            domain_name(".schema.system"),
            Principal::system(),
            no_policies(),
            Default::default(),
            data("a"),
            ExpirationPolicy::YEAR,
        );
        let signed_ops = NoncedBundle::new(vec![register_operation], 42).sign(&sk).unwrap();
        let err = state.apply_signed_bundle(signed_ops).unwrap_err();

        assert_eq!(err.to_string(), "System principal cannot be impersonated");
    }

    #[test]
    fn serde_roundtrip() {
        let mut state = State::new();

        let register_operation = UserOperation::register(
            domain_name(".schema.a"),
            domain_owner(),
            no_policies(),
            Default::default(),
            data("a"),
            ExpirationPolicy::YEAR,
        );
        state.apply_operation(register_operation).unwrap();

        let update_operation = UserOperation::update(domain_name(".schema.a"), data("b"));
        state.apply_operation(update_operation).unwrap();

        let register_operation = UserOperation::register(
            domain_name(".schema.ageover"),
            domain_owner(),
            no_policies(),
            Default::default(),
            data("c"),
            ExpirationPolicy::YEAR,
        );
        state.apply_operation(register_operation).unwrap();

        let serialized = serde_json::to_string(&state).unwrap();

        // println!("{}", serde_json::to_string_pretty(&state).unwrap());

        let deserialized: State = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, state);
    }

    #[test]
    fn register_update_transfer_delete_domain() {
        let name = ".schema.a";
        let domain_name = || domain_name(name);

        let mut state = State::new();

        let register_operation = UserOperation::register(
            domain_name(),
            domain_owner(),
            no_policies(),
            Default::default(),
            data("top level"),
            ExpirationPolicy::YEAR,
        );
        state.apply_operation(register_operation).unwrap();

        assert_eq!(state.version(), 1);
        check_domain_exists(&state, &domain_name(), &data("top level"), &domain_owner());

        let update_operation = UserOperation::update(domain_name(), data("cool, heh?"));
        state.apply_operation(update_operation).unwrap();

        assert_eq!(state.version(), 2);
        check_domain_exists(&state, &domain_name(), &data("cool, heh?"), &domain_owner());

        let pk = "pezDj6ea4tVfNRUTMyssVDepAAzPW67Fe3yHtuHL6ZNtcfJ".parse().unwrap();
        let transfer_to = Principal::public_key(&pk);
        let transfer_operation = UserOperation::transfer(domain_name(), transfer_to.clone());
        state.apply_operation(transfer_operation).unwrap();

        assert_eq!(state.version(), 3);
        check_domain_exists(&state, &domain_name(), &data("cool, heh?"), &transfer_to);

        state.apply_operation(UserOperation::delete(domain_name())).unwrap();

        check_domain_missing(&state, &domain_name());
        assert_eq!(state.last_seen_height(), 0);
        assert_eq!(state.version(), 4);

        state.undo_operation(3).unwrap(); // undo delete

        check_domain_exists(&state, &domain_name(), &data("cool, heh?"), &transfer_to);

        state.undo_operation(2).unwrap(); // undo transfer

        check_domain_exists(&state, &domain_name(), &data("cool, heh?"), &domain_owner());

        state.undo_operation(1).unwrap(); // undo update

        check_domain_exists(&state, &domain_name(), &data("top level"), &domain_owner());
        assert_eq!(state.version(), 1);

        state.undo_operation(0).unwrap(); // undo register

        check_domain_missing(&state, &domain_name());
        assert_eq!(state.version(), 0);
    }

    fn schema_policy(schema: Schema) -> SubtreePolicies {
        SubtreePolicies::new().with_schema(schema)
    }

    #[test]
    fn schema_validation() {
        let mut state = State::new();

        let reg_badschema = UserOperation::register(
            domain_name(".schema.badschema"),
            domain_owner(),
            schema_policy(json!({"properties": "invalid"})),
            Default::default(),
            json!({}),
            ExpirationPolicy::YEAR,
        );
        assert_eq!(
            state.apply_operation(reg_badschema).unwrap_err().to_string(),
            "Domain .schema.badschema has invalid schema"
        );

        let reg_baddata = UserOperation::register(
            domain_name(".schema.baddata"),
            domain_owner(),
            schema_policy(json!( {
                "properties": {
                    "someProperty": {
                        "type": "string",
                    },
                },
                "additionalProperties": false,
            })),
            Default::default(),
            json!({"data": "notmatching"}),
            ExpirationPolicy::YEAR,
        );
        assert_eq!(
            state.apply_operation(reg_baddata).unwrap_err().to_string(),
            "Domain .schema.baddata data does not match schema of .schema.baddata"
        );

        let reg_schema_empty = UserOperation::register(
            domain_name(".schema.empty"),
            domain_owner(),
            schema_policy(json!({"additionalProperties": false,})),
            Default::default(),
            json!({}),
            ExpirationPolicy::YEAR,
        );
        state.apply_operation(reg_schema_empty).unwrap();

        let upd_schema_empty =
            UserOperation::update(domain_name(".schema.empty"), json!({ "bad": "data"}));
        assert_eq!(
            state.apply_operation(upd_schema_empty).unwrap_err().to_string(),
            "Domain .schema.empty data does not match schema of .schema.empty"
        );
    }

    #[test]
    fn authorization() {
        let name = ".schema.a";
        let domain_name = || domain_name(name);

        let mut state = State::new();

        let reg_op = UserOperation::register(
            domain_name(),
            domain_owner(),
            no_policies(),
            Default::default(),
            data("top level"),
            10,
        );
        // TODO Need to implement domain policy first to register under system domains
        state.apply_operation(reg_op).unwrap();

        assert_eq!(state.version(), 1);
        assert_eq!(state.nonce(&domain_owner_pk()), 0);
        check_domain_exists(&state, &domain_name(), &data("top level"), &domain_owner());
        name_resolves_to(&state, &domain_name(), &data("top level")).unwrap();

        state.block_applying(10).unwrap();
        assert_eq!(state.version(), 2);
        name_does_not_resolve(&state, &domain_name()).unwrap();

        let update_op = UserOperation::update(domain_name(), data("cool, heh?"));
        let renew_op = UserOperation::renew(domain_name(), 20);
        let nonced_ops = NoncedBundle::new(vec![renew_op.clone(), update_op.clone()], 1);

        let bad_nonce_ops = NoncedBundle::new(vec![update_op.clone(), renew_op.clone()], 2);
        let signed_ops_bad_nonce = sign_ops(bad_nonce_ops);
        let bad_nonce_err = state.apply_signed_bundle(signed_ops_bad_nonce).unwrap_err();
        assert_eq!(bad_nonce_err.to_string(), "Invalid nonce 2, expected 1");
        assert_eq!(state.version(), 2);
        name_does_not_resolve(&state, &domain_name()).unwrap();

        let bad_signed_ops = sign_ops_by_wrong_key(nonced_ops.clone());
        let bad_signer_err = state.apply_signed_bundle(bad_signed_ops).unwrap_err();
        assert_eq!(
            bad_signer_err.to_string(),
            "PublicKey principal psz291QGsvwafGPkKMu6MUsXThWRcBRzRf6pcVPM1Pst6WgW cannot be impersonated by pszcYyCB1iBEWSD9xFGzFYYQnJvYyvaENgRS9TnjJPNqfkz"
        );
        name_does_not_resolve(&state, &domain_name()).unwrap();

        let bad_order_ops = NoncedBundle::new(vec![update_op, renew_op], 1);
        let bad_order_signed_ops = sign_ops(bad_order_ops);
        let bad_order_err = state.apply_signed_bundle(bad_order_signed_ops).unwrap_err();
        assert_eq!(bad_order_err.to_string(), "Domain .schema.a expired");

        let signed_ops = sign_ops(nonced_ops);
        state.apply_signed_bundle(signed_ops).unwrap();

        assert_eq!(state.version(), 4);
        assert_eq!(state.nonce(&domain_owner_pk()), 1);
        check_domain_exists(&state, &domain_name(), &data("cool, heh?"), &domain_owner());
        name_resolves_to(&state, &domain_name(), &data("cool, heh?")).unwrap();
    }
}
