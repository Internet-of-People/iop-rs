use super::*;

/// Public keys of a Hydra account in a vault.
///
/// @see HydraPlugin.pub
#[wasm_bindgen(js_name = HydraPublic)]
pub struct JsHydraPublic {
    inner: HydraPublic,
}

#[wasm_bindgen(js_class = HydraPublic)]
impl JsHydraPublic {
    /// Name of the network this account belongs to.
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.network().subtree().name().to_owned()
    }

    /// Calculates the receiving address having the given index and takes note that the address was already generated in the account.
    ///
    /// @see Bip44Account.key, Bip44Account.chain
    pub fn key(&mut self, idx: i32) -> Result<JsBip44PublicKey, JsValue> {
        let inner = self.inner.key_mut(idx).map_err_to_js()?;
        Ok(JsBip44PublicKey::from(inner))
    }

    /// The extended public key for auditing the whole Bip44 account or deriving new public keys outside the vault.
    #[wasm_bindgen(getter)]
    pub fn xpub(&self) -> Result<String, JsValue> {
        let res = self.inner().xpub().map_err_to_js()?;
        Ok(res)
    }

    /// How many receive addresses have been used in this {@link Bip44Account}
    #[wasm_bindgen(getter = receiveKeys)]
    pub fn receive_keys(&self) -> Result<u32, JsValue> {
        let res = self.inner().receive_keys().map_err_to_js()?;
        Ok(res)
    }

    /// How many change addresses have been used in this {@link Bip44Account}
    #[wasm_bindgen(getter = changeKeys)]
    pub fn change_keys(&self) -> Result<u32, JsValue> {
        let res = self.inner().change_keys().map_err_to_js()?;
        Ok(res)
    }

    /// Finds the {@link Bip44PublicKey} public api that belongs to the given P2PKH address. You can check the index of the key or
    /// get the actual {@link SecpPublicKey} from the returned object.
    ///
    /// Throws an error if the address is not in this account, which can also happen when the key was derived outside the vault and
    /// therefore the vault does not know it was already used. In that case, make sure to "touch" the last key index used by calling
    /// {@link key} before calling this method.
    #[wasm_bindgen(js_name = keyByAddress)]
    pub fn key_by_p2pkh_addr(&self, addr: &str) -> Result<JsBip44PublicKey, JsValue> {
        let inner = self.inner.key_by_p2pkh_addr(addr).map_err_to_js()?;
        Ok(JsBip44PublicKey::from(inner))
    }
}

impl From<HydraPublic> for JsHydraPublic {
    fn from(inner: HydraPublic) -> Self {
        Self { inner }
    }
}

impl Wraps<HydraPublic> for JsHydraPublic {
    fn inner(&self) -> &HydraPublic {
        &self.inner
    }
}
