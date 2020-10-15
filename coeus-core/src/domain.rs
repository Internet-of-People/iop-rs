use super::*;

pub type Schema = serde_json::Value;
pub type DynamicContent = serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    name: DomainName,
    owner: Principal,
    children: HashMap<Edge, Domain>,
    subtree_policies: SubtreePolicies,
    registration_policy: RegistrationPolicy,
    data: DynamicContent,
    expires_at_height: BlockHeight,
}

impl Domain {
    pub fn name(&self) -> &DomainName {
        &self.name
    }

    pub fn owner(&self) -> &Principal {
        &self.owner
    }

    pub fn set_owner(&mut self, owner: Principal) {
        self.owner = owner
    }

    pub fn child(&self, edge: &Edge) -> Option<&Domain> {
        self.children.get(edge)
    }

    pub fn child_mut(&mut self, edge: &Edge) -> Option<&mut Domain> {
        self.children.get_mut(edge)
    }

    pub fn child_names(&self) -> Vec<&Edge> {
        self.children.keys().collect()
    }

    pub fn insert_or_replace_child(&mut self, domain: Domain) -> Result<Option<Domain>> {
        ensure!(!domain.name.is_root(), "Attempt to insert root node as child entry");
        let edge = domain
            .name
            .last_edge()
            .with_context(|| "Implementation error: already checked that domain is not root")?;
        let old_domain = self.children.insert(edge.to_owned(), domain);
        Ok(old_domain)
    }

    pub fn remove_child(&mut self, edge: &Edge) -> Result<Domain> {
        let domain = self
            .children
            .remove(edge)
            .with_context(|| format!("Attempt to delete nonexisting child: {}", edge))?;
        Ok(domain)
    }

    pub fn data(&self) -> &DynamicContent {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut DynamicContent {
        &mut self.data
    }

    pub fn set_data(&mut self, data: DynamicContent) {
        self.data = data;
    }

    pub fn is_expired_at(&self, height: BlockHeight) -> bool {
        self.expires_at_height <= height
    }

    pub fn expires_at_height(&self) -> BlockHeight {
        self.expires_at_height
    }

    pub fn set_expires_at_height(&mut self, height: BlockHeight) {
        self.expires_at_height = height
    }

    pub fn is_grace_period_over(&self, at_height: BlockHeight) -> bool {
        // TODO this should come from some DomainPolicy instead
        const GRACE_PERIOD_BLOCKS: BlockHeight = 5 * 60 * 24 * 30; // about a month by default
        self.expires_at_height + GRACE_PERIOD_BLOCKS <= at_height
    }

    pub fn subtree_policies(&self) -> &SubtreePolicies {
        &self.subtree_policies
    }

    pub(crate) fn registration_policy(&self) -> &RegistrationPolicy {
        &self.registration_policy
    }

    pub(crate) fn validate_subtree_policies(
        &self, state: &State, domain_after_op: &Domain,
    ) -> Result<()> {
        self.subtree_policies.validate(state, self, domain_after_op)?;
        Ok(())
    }
    pub(crate) fn new_root() -> Self {
        let to_edge = |e: &&str| {
            Edge::new(e).unwrap_or_else(|_| {
                panic!("Implementation error creating root: {} is not a valid edge name", e)
            })
        };
        let name = |edges: &[&str]| {
            let edges = edges.iter().map(to_edge).collect();
            DomainName::new(edges)
        };
        let schema = Self {
            name: name(&["schema"]),
            owner: Principal::system(),
            children: Default::default(),
            subtree_policies: SubtreePolicies::new().with_schema(Self::json_schema_draft6()),
            registration_policy: RegistrationPolicy::any(),
            data: json!({}),
            expires_at_height: BlockHeight::max_value(),
        };
        let mut root = Self {
            name: name(&[]),
            owner: Principal::system(),
            children: Default::default(),
            // TODO fill in schema and root data
            subtree_policies: SubtreePolicies::new().with_expiration(2 * ExpirationPolicy::YEAR),
            registration_policy: Default::default(),
            data: json!({}),
            expires_at_height: BlockHeight::max_value(),
        };
        root.insert_or_replace_child(schema).unwrap();
        root
    }

    pub(crate) fn new(
        name: DomainName, owner: Principal, subtree_policies: SubtreePolicies,
        registration_policy: RegistrationPolicy, data: DynamicContent,
        expires_at_height: BlockHeight,
    ) -> Self {
        Self {
            name,
            owner,
            children: Default::default(),
            subtree_policies,
            registration_policy,
            data,
            expires_at_height,
        }
    }

    fn json_schema_draft6() -> Schema {
        // Valico supports Json Schema Draft 6, contents were extracted from
        // https://json-schema.org/draft-06/schema
        let schema = include_str!("../json-schema-draft6.json");
        serde_json::from_str(&schema).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn name(name: &str) -> DomainName {
        name.parse().unwrap()
    }

    fn data(content: &str) -> DynamicContent {
        serde_json::Value::String(content.to_owned())
    }

    #[test]
    fn serde() {
        let wallet_schema = json![{"type": "string"}];
        let wallet_expiration = 2_628_000;
        let mut wallet = Domain::new(
            name(".wallet"),
            Principal::system(),
            SubtreePolicies::new().with_schema(wallet_schema).with_expiration(wallet_expiration),
            RegistrationPolicy::any(),
            data("hello"),
            42,
        );
        wallet
            .insert_or_replace_child(Domain::new(
                name(".wallet.joe"),
                "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6".parse().unwrap(),
                SubtreePolicies::new(),
                Default::default(),
                data("world!"),
                69,
            ))
            .unwrap();
        let serialized = serde_json::to_value(&wallet).unwrap();

        println!("{}", serde_json::to_string_pretty(&wallet).unwrap());

        // TODO: Seems like the `name` field in `Domain` is redundant in the current implementation
        let expected = json!( {
          "name": [
            "wallet"
          ],
          "owner": "system",
          "children": {
            "joe": {
              "name": [
                "wallet",
                "joe"
              ],
              "owner": "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6",
              "children": {},
              "subtreePolicies": {},
              "registrationPolicy": "owner",
              "data": "world!",
              "expiresAtHeight": 69
            }
          },
          "subtreePolicies": {
            "schema": { "type": "string" },
            "expiration": 2_628_000,
          },
          "registrationPolicy": "any",
          "data": "hello",
          "expiresAtHeight": 42
        });

        assert_eq!(serialized, expected);

        let deserialized: Domain = serde_json::from_value(expected).unwrap();

        assert_eq!(deserialized, wallet);
    }
}
