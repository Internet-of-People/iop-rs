use super::*;

pub type Label = String;
pub type Metadata = String;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HdRecord {
    pub(super) bip32_idx: i32,
    pub(super) public_key: MPublicKey,
    pub(super) label: Label,
    pub(super) metadata: Metadata,
}

impl HdRecord {
    pub(super) fn new(bip32_idx: i32, pubkey: MPublicKey, label: Label) -> Self {
        Self { bip32_idx, public_key: pubkey, label, metadata: Default::default() }
        // version: 0
        // document: DidDocument {}
    }

    pub fn bip32_idx(&self) -> i32 {
        self.bip32_idx
    }
    pub fn public_key(&self) -> MPublicKey {
        self.public_key.to_owned()
    }
    pub fn key_id(&self) -> MKeyId {
        self.public_key.key_id()
    }
    pub fn did(&self) -> Did {
        self.key_id().into()
    }
    pub fn label(&self) -> Label {
        self.label.to_owned()
    }
    pub fn set_label(&mut self, label: Label) {
        self.label = label;
    }
    pub fn metadata(&self) -> Metadata {
        self.metadata.to_owned()
    }
    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }
    // pub fn document(&self) -> DidDocument { self.document.to_owned() }
}
