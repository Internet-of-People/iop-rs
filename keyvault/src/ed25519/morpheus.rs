use super::*;
use crate::multicipher::{MPrivateKey, MPublicKey};

/// There can be several usages of DIDs differentiated inside the wallet
/// invisible externally, e.g. on a blockchain.
/// Each represents a separate subtree under the Morpheus root node of the HD wallet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum DidKind {
    /// A certain aspect of your personal life worth separating.
    /// E.g. business, family, education, hobby, dating, etc.
    Persona = 0,
    /// A handle of a persona paired with a single device only.
    /// Useful for using the same persona on multiple devices.
    Device = 1,
    /// An identifier intrinsically handled by multiple personas.
    /// E.g. legal entities, organizations, families, etc.
    Group = 2,
    /// An identifier that represents some resource or property in the real world.
    /// E.g. smart lock, real estate, NFT, etc.
    Resource = 3,
}

impl DidKind {
    /// The canonical BIP32 derivation path of the node that stores all identifiers of the given kind.
    pub fn bip32_path(self) -> bip32::Path {
        Morpheus.bip32_path().append(ChildIndex::Hardened(self as i32))
    }

    /// All variants of the enumeration
    pub fn all() -> &'static [DidKind] {
        &[DidKind::Persona, DidKind::Device, DidKind::Group, DidKind::Resource]
    }
}

impl FromStr for DidKind {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        match input.to_lowercase().as_ref() {
            "persona" => Ok(DidKind::Persona),
            "device" => Ok(DidKind::Device),
            "group" => Ok(DidKind::Group),
            "resource" => Ok(DidKind::Resource),
            _ => bail!("Unknown DID kind {}", input),
        }
    }
}

/// Configuration of Bip32 key derivation for Morpheus.
pub struct MorpheusSubtree;

impl Subtree for MorpheusSubtree {
    type Suite = Ed25519;

    fn name(&self) -> &'static str {
        "morpheus"
    }
    fn master(&self, seed: &Seed) -> EdExtPrivateKey {
        Ed25519::master(seed)
    }
    fn key_id(&self, pk: &EdPublicKey) -> EdKeyId {
        pk.key_id()
    }
}

#[derive(Clone, Copy, Debug)]
/// Starting point for deriving all Morpheus related keys.
pub struct Morpheus;

impl Morpheus {
    /// Morpheus is not a coin as defined by Bip44, so it needs a separate Bip43 purpose
    /// to derive its root node.
    pub const BIP43_PURPOSE: i32 = 0x1F4A4;

    /// Calculate the root node of the Morpheus subtree in the HD wallet.
    pub fn root(self, seed: &Seed) -> Result<MorpheusRoot> {
        let node = Bip32.master(&seed, &MorpheusSubtree).derive_hardened(Self::BIP43_PURPOSE)?;
        Ok(MorpheusRoot { node })
    }

    /// The canonical BIP32 derivation path of the root Morpheus node.
    pub fn bip32_path(self) -> bip32::Path {
        Bip43Path::purpose(Morpheus::BIP43_PURPOSE).bip32_path()
    }
}

#[derive(Clone, Debug)]
/// Representation of the root node of the Morpheus subtree in the HD wallet.
pub struct MorpheusRoot {
    node: Bip32Node<Ed25519>,
}

impl MorpheusRoot {
    /// To match the `node.path().bip32_path()` pattern, the `Morpheus` sizeless struct has the `bip32_path` method and this method returns
    /// a reference to `Morpheus`.
    pub fn path(&self) -> &Morpheus {
        &Morpheus
    }

    /// Returns a reference to the underlying BIP32 node.
    pub fn node(&self) -> &Bip32Node<ed25519::Ed25519> {
        &self.node
    }

    /// Creates an admin API for the Morpheus root node.
    pub fn admin(&self) -> MorpheusVaultAdmin {
        MorpheusVaultAdmin { parent: self.clone() }
    }

    /// Alias for kind(Persona).
    pub fn personas(&self) -> Result<MorpheusKind> {
        self.kind(DidKind::Persona)
    }

    /// Alias for kind(Device).
    pub fn devices(&self) -> Result<MorpheusKind> {
        self.kind(DidKind::Device)
    }

    /// Alias for kind(Group).
    pub fn groups(&self) -> Result<MorpheusKind> {
        self.kind(DidKind::Group)
    }

    /// Alias for kind(Resource).
    pub fn resources(&self) -> Result<MorpheusKind> {
        self.kind(DidKind::Resource)
    }

    /// Derive a separate HD wallet subtree of the given kind.
    pub fn kind(&self, kind: DidKind) -> Result<MorpheusKind> {
        let node = self.node.derive_hardened(kind as i32)?;
        Ok(MorpheusKind { kind, node })
    }
}

#[derive(Clone, Debug)]
/// The admin node for the Morpheus root node will be used for self-encrypting administrative data on a storage for data not derived from the seed itself.
pub struct MorpheusVaultAdmin {
    parent: MorpheusRoot,
}

impl MorpheusVaultAdmin {
    /// To match the `node.path().bip32_path()` pattern, the `Morpheus` sizeless struct has the `bip32_path` method and this method returns
    /// a reference to `Morpheus`.
    pub fn path(&self) -> &Morpheus {
        &Morpheus
    }

    /// Returns a reference to the underlying BIP32 node.
    pub fn node(&self) -> &Bip32Node<ed25519::Ed25519> {
        &self.parent.node()
    }
}

#[derive(Clone, Debug)]
/// The admin node for a Morpheus kind node will be used for self-encrypting administrative data on a storage for the collection of identifiers of that kind.
pub struct MorpheusKindAdmin {
    parent: MorpheusKind,
}

impl MorpheusKindAdmin {
    /// Accessor for the kind of subtree this node is an admin for.
    pub fn path(&self) -> DidKind {
        self.parent.path()
    }

    /// Returns a reference to the underlying BIP32 node.
    pub fn node(&self) -> &Bip32Node<ed25519::Ed25519> {
        &self.parent.node()
    }
}

/// Root node of a specific kind of DIDs. The kind is invisible outside the wallet.
#[derive(Clone, Debug)]
pub struct MorpheusKind {
    kind: DidKind,
    node: Bip32Node<Ed25519>,
}

impl MorpheusKind {
    /// Accessor for the kind of identifiers generated by this node.
    pub fn path(&self) -> DidKind {
        self.kind
    }

    /// Returns a reference to the underlying BIP32 node.
    pub fn node(&self) -> &Bip32Node<ed25519::Ed25519> {
        &self.node
    }

    /// Creates an admin API for the Morpheus kind node.
    pub fn admin(&self) -> MorpheusKindAdmin {
        MorpheusKindAdmin { parent: self.clone() }
    }

    /// The private key of the child node with given index under this subtree.
    /// E.g. 5th persona, 3rd device, or 0th group, etc.
    pub fn key(&self, idx: i32) -> Result<MorpheusPrivateKey> {
        let path = MorpheusKeyPath { kind: self.kind, idx };
        let node = self.node.derive_hardened(idx)?;
        Ok(MorpheusPrivateKey { path, node })
    }
}

#[derive(Clone, Debug)]
/// A Morpheus path describing a position of a node in the HD wallet without being bound to a given seed. Will be useful for hardware wallet
/// integrations in the future.
pub struct MorpheusKeyPath {
    kind: DidKind,
    idx: i32,
}

impl MorpheusKeyPath {
    /// The kind of identifiers in this subtree
    pub fn kind(&self) -> DidKind {
        self.kind
    }

    /// The index of the identifier
    pub fn idx(&self) -> i32 {
        self.idx
    }

    /// The canonical BIP32 derivation path of the identifier.
    pub fn bip32_path(&self) -> bip32::Path {
        self.kind.bip32_path().append(ChildIndex::Hardened(self.idx))
    }
}

#[derive(Clone, Debug)]
/// The operations on an identifier that require the private key to be available in memory.
pub struct MorpheusPrivateKey {
    path: MorpheusKeyPath,
    node: Bip32Node<ed25519::Ed25519>,
}

impl MorpheusPrivateKey {
    /// Created the public interface of the node that does not need the private key in memory.
    pub fn neuter(&self) -> MorpheusPublicKey {
        let node = self.node.neuter();
        MorpheusPublicKey { path: self.path.clone(), node }
    }

    /// Returns the Morpheus path for this identifier.
    pub fn path(&self) -> &MorpheusKeyPath {
        &self.path
    }

    /// Returns a reference to the underlying BIP32 node,
    pub fn node(&self) -> &Bip32Node<ed25519::Ed25519> {
        &self.node
    }

    /// Returns the multicipher private key that belongs to this identifier.
    pub fn private_key(&self) -> MPrivateKey {
        MPrivateKey::from(self.node.private_key())
    }
}

#[derive(Clone, Debug)]
/// The operations on an identifier that do not require the private key to be available in memory.
pub struct MorpheusPublicKey {
    path: MorpheusKeyPath,
    node: Bip32PublicNode<ed25519::Ed25519>,
}

impl MorpheusPublicKey {
    /// Returns the Morpheus path for this identifier.
    pub fn path(&self) -> &MorpheusKeyPath {
        &self.path
    }

    /// Returns a reference to the underlying BIP32 public node.
    pub fn node(&self) -> &Bip32PublicNode<ed25519::Ed25519> {
        &self.node
    }

    /// Returns the multicipher public key that belongs to this identifier.
    pub fn public_key(&self) -> MPublicKey {
        MPublicKey::from(self.node.public_key())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO Should we rename node() to bip32_node() in this module and Bip44 as well?
    #[test]
    fn api_usage() -> Result<()> {
        let seed = Bip39::new().phrase(Seed::DEMO_PHRASE)?.password(Seed::PASSWORD);
        let morpheus = Morpheus.root(&seed)?;
        assert_eq!(morpheus.path().bip32_path(), "m/128164'".parse()?);
        assert_eq!(morpheus.node().path(), &"m/128164'".parse()?);

        let vault_admin = morpheus.admin();
        assert_eq!(vault_admin.path().bip32_path(), "m/128164'".parse()?);
        assert_eq!(vault_admin.node().path(), &"m/128164'".parse()?);

        let groups = morpheus.groups()?;
        assert_eq!(groups.path(), DidKind::Group);
        assert_eq!(groups.path().bip32_path(), "m/128164'/2'".parse()?);
        assert_eq!(groups.node().path(), &"m/128164'/2'".parse()?);

        let groups_admin = groups.admin();
        assert_eq!(groups_admin.path().bip32_path(), "m/128164'/2'".parse()?);
        assert_eq!(groups_admin.node().path(), &"m/128164'/2'".parse()?);

        let group0 = groups.key(0)?;
        assert_eq!(group0.path().bip32_path(), "m/128164'/2'/0'".parse()?);
        assert_eq!(group0.node().path(), &"m/128164'/2'/0'".parse()?);
        assert_eq!(group0.path().kind(), DidKind::Group);
        assert_eq!(group0.path().idx(), 0);

        let group0_pub = group0.neuter();
        assert_eq!(group0_pub.path().bip32_path(), "m/128164'/2'/0'".parse()?);
        assert_eq!(group0_pub.node().path(), &"m/128164'/2'/0'".parse()?);
        let group0_ed_pk = group0_pub.node().public_key();
        assert_eq!(
            hex::encode(group0_ed_pk.to_bytes()),
            "10634a63a4ab84d40170079f2c538d969d4693de9e399f32b8bd29e4583e2e42"
        );
        let group0_pk = group0_pub.public_key();
        assert_eq!(group0_pk.to_string(), "pez26yLWBBR78PjHvMVWZWJK8BC8fQ4KyUMmvNVdpvdKCN5");

        let group0_ed_sk = group0.node().private_key();
        assert_eq!(
            hex::encode(group0_ed_sk.to_bytes()),
            "9f3dbcd653e8825e765c5c2c40b105e73f1eb088ec18247550ff12f2a0733947"
        );
        let group0_sk = group0.private_key();
        assert_eq!(
            group0_sk.public_key().to_string(),
            "pez26yLWBBR78PjHvMVWZWJK8BC8fQ4KyUMmvNVdpvdKCN5"
        );

        // TODO vault_admin.encrypt(plain_data_vec);
        // TODO vault_admin.decrypt(ecnrypted_data_vec);
        // TODO group_admin.encrypt(plain_data_vec);
        // TODO group_admin.decrypt(ecnrypted_data_vec);

        Ok(())
    }
}
