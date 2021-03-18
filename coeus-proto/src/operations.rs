use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoDelete {
    #[serde(with = "serde_str")]
    pub name: DomainName,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoRegister {
    #[serde(with = "serde_str")]
    pub name: DomainName,
    pub owner: Principal,
    pub subtree_policies: SubtreePolicies,
    pub registration_policy: RegistrationPolicy,
    pub data: DynamicContent,
    pub expires_at_height: BlockHeight,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoRenew {
    #[serde(with = "serde_str")]
    pub name: DomainName,
    pub expires_at_height: BlockHeight,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoTransfer {
    #[serde(with = "serde_str")]
    pub name: DomainName,
    pub to_owner: Principal,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoUpdate {
    #[serde(with = "serde_str")]
    pub name: DomainName,
    pub data: DynamicContent,
}

pub trait Priced {
    fn get_price(&self) -> Price;
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UserOperation {
    Register(Box<DoRegister>),
    Update(DoUpdate),
    Renew(DoRenew),
    Transfer(DoTransfer),
    Delete(DoDelete),
}

impl UserOperation {
    pub fn register(
        name: DomainName, owner: Principal, subtree_policies: SubtreePolicies,
        registration_policy: RegistrationPolicy, data: DynamicContent,
        expires_at_height: BlockHeight,
    ) -> Self {
        Self::Register(Box::new(DoRegister {
            name,
            owner,
            subtree_policies,
            registration_policy,
            data,
            expires_at_height,
        }))
    }

    pub fn update(name: DomainName, data: DynamicContent) -> Self {
        Self::Update(DoUpdate { name, data })
    }

    pub fn renew(name: DomainName, expires_at_height: BlockHeight) -> Self {
        Self::Renew(DoRenew { name, expires_at_height })
    }

    pub fn transfer(name: DomainName, to_owner: Principal) -> Self {
        Self::Transfer(DoTransfer { name, to_owner })
    }

    pub fn delete(name: DomainName) -> Self {
        Self::Delete(DoDelete { name })
    }
}

impl Priced for UserOperation {
    fn get_price(&self) -> Price {
        // Register is sooo much bigger in its serialized form that we try to compensate other operations
        // with a small offset in addition to the size-based fee of the whole transaction
        match self {
            Self::Register(_op) => Price::fee(0),
            Self::Update(_op) => Price::fee(200_000),
            Self::Renew(_op) => Price::fee(200_000),
            Self::Transfer(_op) => Price::fee(200_000),
            Self::Delete(_op) => Price::fee(200_000),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serde() {
        let input = r#"{"type":"register","name":".schema.company","owner":"pszp9HBQY4qrx2yPGqM6biZeLmudJanMK6LXzXzLZGciLYA","subtreePolicies":{},"registrationPolicy":"owner","data":{},"expiresAtHeight":1000}"#;
        let op: UserOperation = serde_json::from_str(input).unwrap();

        assert!(matches!(op, UserOperation::Register(_)));
        if let UserOperation::Register(r) = &op {
            assert_eq!(r.name.to_string(), ".schema.company");
        }

        let output = serde_json::to_string(&op).unwrap();

        assert_eq!(output, input);
    }
}
