use super::*;

/// Entry point to generate extended private keys in a hierarchical deterministic wallet starting from a seed based
/// on the [BIP-0032](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) standard
/// (and the [SLIP-0010](https://github.com/satoshilabs/slips/blob/master/slip-0010.md) for crypto suites other than Secp256k1).
#[wasm_bindgen(js_name = Bip32)]
#[derive(Clone, Debug)]
pub struct JsBip32;

#[wasm_bindgen(js_class = Bip32)]
impl JsBip32 {
    /// Calculates the master extended private key based on the crypto suite used by the given network. (At the moment
    /// only Secp256k1-based networks are supported in the WASM wrappers)
    ///
    /// @see allNetworkNames, validateNetworkName
    pub fn master(seed: &JsSeed, name: &str) -> Result<JsBip32Node, JsValue> {
        let network = Networks::by_name(name).map_err_to_js()?;
        let node = Bip32.master(seed.inner(), network.subtree());
        Ok(JsBip32Node::from(node))
    }
}

/// In BIP-0032 each extended private key has the same operations, independently from the actual path. This struct represents such
/// an extended private key in a given subtree.
#[wasm_bindgen(js_name = Bip32Node)]
#[derive(Clone, Debug)]
pub struct JsBip32Node {
    inner: Bip32Node<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip32Node)]
impl JsBip32Node {
    /// Name of the network this node was generated for
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.subtree().name().to_owned()
    }

    /// The BIP32 path of this node
    #[wasm_bindgen(getter)]
    pub fn path(&self) -> String {
        self.inner.path().to_string()
    }

    /// Create a new node with normal (public) derivation with the given index.
    #[wasm_bindgen(js_name = deriveNormal)]
    pub fn derive_normal(&self, idx: i32) -> Result<JsBip32Node, JsValue> {
        let child = self.inner.derive_normal(idx).map_err_to_js()?;
        Ok(JsBip32Node::from(child))
    }

    /// Create a new node with hardened (private) derivation with the given index.
    #[wasm_bindgen(js_name = deriveHardened)]
    pub fn derive_hardened(&self, idx: i32) -> Result<JsBip32Node, JsValue> {
        let child = self.inner.derive_hardened(idx).map_err_to_js()?;
        Ok(JsBip32Node::from(child))
    }

    /// Creates the {@SecpPrivateKey} that belongs to this node for authenticating actions.
    #[wasm_bindgen(js_name = privateKey)]
    pub fn to_private_key(&self) -> JsSecpPrivateKey {
        JsSecpPrivateKey::from(self.inner.private_key())
    }

    /// Removes the ability to sign and derive hardened keys. The public node it returns is still able to provide
    /// normal derivation and signature verifications.
    #[wasm_bindgen(js_name = neuter)]
    pub fn neuter(&self) -> JsBip32PublicNode {
        let inner = self.inner.neuter();
        JsBip32PublicNode::from(inner)
    }

    // Secp specific methods...

    /// Returns the extended private key in the BIP32 readable format with the version bytes of the network.
    ///
    /// This is a secret that must not be kept unencrypted in transit or in rest!
    #[wasm_bindgen(js_name = toXprv)]
    pub fn to_xprv(&self, name: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(name).map_err_to_js()?;
        Ok(self.inner.to_xprv(network))
    }

    /// Returns the private key in the Wallet Import Format with the version byte of the network.
    ///
    /// This is a secret that must not be kept unencrypted in transit or in rest!
    ///
    /// @see SecpPrivateKey.toWif
    #[wasm_bindgen(js_name = toWif)]
    pub fn to_wif(&self, name: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(name).map_err_to_js()?;
        Ok(self.inner.to_wif(network))
    }
}

impl From<Bip32Node<Secp256k1>> for JsBip32Node {
    fn from(inner: Bip32Node<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip32Node<Secp256k1>> for JsBip32Node {
    fn inner(&self) -> &Bip32Node<Secp256k1> {
        &self.inner
    }
}

/// In BIP-0032 a neutered extended private key is an extended public key. This object represents
/// such an extended public key in a given subtree. It is able to do normal (public) derivation,
/// signature verification, creating and validating key identifiers
#[wasm_bindgen(js_name = Bip32PublicNode)]
#[derive(Clone, Debug)]
pub struct JsBip32PublicNode {
    inner: Bip32PublicNode<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip32PublicNode)]
impl JsBip32PublicNode {
    /// Name of the network this node was generated for
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.subtree().name().to_owned()
    }

    /// The BIP32 path of this node
    #[wasm_bindgen(getter)]
    pub fn path(&self) -> String {
        self.inner.path().to_string()
    }

    /// Create a new node with normal (public) derivation with the given index.
    #[wasm_bindgen(js_name = deriveNormal)]
    pub fn derive_normal(&self, idx: i32) -> Result<JsBip32PublicNode, JsValue> {
        let child = self.inner.derive_normal(idx).map_err_to_js()?;
        Ok(JsBip32PublicNode::from(child))
    }

    /// Creates the public key that belongs to this node for verifying authentications done by the corresponding private key.
    #[wasm_bindgen(js_name = publicKey)]
    pub fn to_public_key(&self) -> JsSecpPublicKey {
        JsSecpPublicKey::from(self.inner.public_key())
    }

    /// Creates the key identifier for the public key. This is an extra layer of security for single-use keys, so the
    /// revealing of the public key can be delayed to the point when the authenticated action (spending some coin or
    /// revoking access) makes the public key irrelevant after the action is successful.
    ///
    /// Ark (and therefore Hydra) uses a different algorithm for calculating key identifiers. That is only available at
    /// {@link SecpPublicKey.arkKeyId}
    #[wasm_bindgen(js_name = keyId)]
    pub fn to_key_id(&self) -> JsSecpKeyId {
        JsSecpKeyId::from(self.inner.key_id())
    }

    // Secp specific methods...

    /// Returns the extended public key in the BIP32 readable format with the version bytes of the network.
    #[wasm_bindgen(js_name = toXpub)]
    pub fn to_xpub(&self, name: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(name).map_err_to_js()?;
        Ok(self.inner.to_xpub(network))
    }

    /// Returns the P2PKH address that belongs to this node using the version byte of the network.
    #[wasm_bindgen(js_name = toP2pkh)]
    pub fn to_p2pkh_addr(&self, name: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(name).map_err_to_js()?;
        Ok(self.inner.to_p2pkh_addr(network))
    }
}

impl From<Bip32PublicNode<Secp256k1>> for JsBip32PublicNode {
    fn from(inner: Bip32PublicNode<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip32PublicNode<Secp256k1>> for JsBip32PublicNode {
    fn inner(&self) -> &Bip32PublicNode<Secp256k1> {
        &self.inner
    }
}
