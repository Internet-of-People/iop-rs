mod delete;
mod register;
mod renew;
mod start_block;
mod transfer;
mod update;

pub use delete::*;
pub use register::*;
pub use renew::*;
pub use start_block::*;
pub use transfer::*;
pub use update::*;

use super::*;

pub(crate) trait Command {
    fn execute(self, state: &mut State) -> Result<UndoOperation>;
}

pub(crate) trait AuthorizedCommand: Command {
    fn validate_auth(&self, state: &State, pk: &MPublicKey) -> Result<()>;
}

pub trait Priced {
    fn get_price(&self) -> Price;
}

pub(crate) trait UndoCommand {
    fn execute(self, state: &mut State) -> Result<()>;
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

impl Command for UserOperation {
    fn execute(self, state: &mut State) -> Result<UndoOperation> {
        match self {
            Self::Register(op) => op.execute(state),
            Self::Update(op) => op.execute(state),
            Self::Renew(op) => op.execute(state),
            Self::Transfer(op) => op.execute(state),
            Self::Delete(op) => op.execute(state),
        }
    }
}

// TODO work out ecosystem for pricing model: naming system state (e.g. subdomain registration policies)
//      and operation properties (especially last_edge length) might influence prices
impl Priced for UserOperation {
    fn get_price(&self) -> Price {
        match self {
            Self::Register(_op) => Price::fee(100_000_000),
            Self::Update(_op) => Price::fee(10_000_000),
            Self::Renew(_op) => Price::fee(50_000_000),
            Self::Transfer(_op) => Price::fee(25_000_000),
            Self::Delete(_op) => Price::fee(1_000_000),
        }
    }
}

impl AuthorizedCommand for UserOperation {
    fn validate_auth(&self, state: &State, pk: &MPublicKey) -> Result<()> {
        match self {
            Self::Register(op) => op.validate_auth(state, pk),
            Self::Update(op) => op.validate_auth(state, pk),
            Self::Renew(op) => op.validate_auth(state, pk),
            Self::Transfer(op) => op.validate_auth(state, pk),
            Self::Delete(op) => op.validate_auth(state, pk),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SystemOperation {
    StartBlock(DoStartBlock),
}

impl SystemOperation {
    pub fn start_block(height: BlockHeight) -> Self {
        SystemOperation::StartBlock(DoStartBlock { height })
    }
}

impl Command for SystemOperation {
    fn execute(self, state: &mut State) -> Result<UndoOperation> {
        match self {
            Self::StartBlock(op) => op.execute(state),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Operation {
    System(SystemOperation),
    User(UserOperation),
}

impl Command for Operation {
    fn execute(self, state: &mut State) -> Result<UndoOperation> {
        match self {
            Self::System(op) => op.execute(state),
            Self::User(op) => op.execute(state),
        }
    }
}

impl From<SystemOperation> for Operation {
    fn from(op: SystemOperation) -> Self {
        Self::System(op)
    }
}

impl From<UserOperation> for Operation {
    fn from(op: UserOperation) -> Self {
        Self::User(op)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub(crate) enum UndoOperation {
    StartBlock(UndoStartBlock),
    Register(Box<UndoRegister>),
    Update(UndoUpdate),
    Renew(UndoRenew),
    Transfer(UndoTransfer),
    Delete(UndoDelete),
}

impl UndoCommand for UndoOperation {
    fn execute(self, state: &mut State) -> Result<()> {
        match self {
            Self::StartBlock(op) => op.execute(state),
            Self::Register(op) => op.execute(state),
            Self::Update(op) => op.execute(state),
            Self::Renew(op) => op.execute(state),
            Self::Transfer(op) => op.execute(state),
            Self::Delete(op) => op.execute(state),
        }
    }
}
