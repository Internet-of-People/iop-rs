use super::*;

/// Private keys of a Hydra account in a vault.
///
/// @see HydraPlugin.priv
#[wasm_bindgen(js_name = HydraPrivate)]
pub struct JsHydraPrivate {
    inner: HydraPrivate,
}

#[wasm_bindgen(js_class = HydraPrivate)]
impl JsHydraPrivate {
    /// Access to the public keys of the account. Same as {@link HydraPlugin.pub} would return.
    #[wasm_bindgen(getter = pub)]
    pub fn public(&self) -> JsHydraPublic {
        let inner = self.inner.public();
        JsHydraPublic::from(inner)
    }

    /// Name of the network this account belongs to.
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.network().subtree().name().to_owned()
    }

    /// Calculates the receiving address having the given index and takes note that the address was already generated in the account.
    ///
    /// @see Bip44Account.key, Bip44Account.chain
    pub fn key(&mut self, idx: i32) -> Result<JsBip44Key, JsValue> {
        let inner = self.inner.key_mut(idx).map_err_to_js()?;
        Ok(JsBip44Key::from(inner))
    }

    /// Finds the {@link Bip44Key} private api that belongs to the given {@link SecpPublicKey}. You can check the index of the key or
    /// get the actual {@link SecpPrivateKey} from the returned object.
    ///
    /// Throws an error if the public key is not in this account, which can also happen when the key was derived outside the vault and
    /// therefore the vault does not know it was already used. In that case, make sure to "touch" the last key index used by calling
    /// {@link key} before calling this method.
    #[wasm_bindgen(js_name = keyByPublicKey)]
    pub fn key_by_pk(&self, id: &JsSecpPublicKey) -> Result<JsBip44Key, JsValue> {
        let inner = self.inner.key_by_pk(id.inner()).map_err_to_js()?;
        Ok(JsBip44Key::from(inner))
    }

    /// The extended public key for auditing the whole Bip44 account or deriving new public keys outside the vault.
    #[wasm_bindgen(getter)]
    pub fn xpub(&self) -> Result<String, JsValue> {
        let res = self.inner().xpub().map_err_to_js()?;
        Ok(res)
    }

    /// The extended private key for the whole account. This is only for exporting into other BIP32 compatible wallets.
    ///
    /// This is a secret that must not be kept unencrypted in transit or in rest!
    #[wasm_bindgen(getter)]
    pub fn xprv(&self) -> String {
        self.inner().xprv()
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

    /// Signs the Hydra transaction with the private key that belongs to the given P2PKH address.
    ///
    /// Fills in signature and id fields, so those can be missing in the unsigned input, but the public key needs to be already
    /// properly set to the one matching the signer address.
    ///
    /// Throws an error if the address is not in this account, which can also happen when the key was derived outside the vault and
    /// therefore the vault does not know it was already used. In that case, make sure to "touch" the last key index used by calling
    /// {@link key} before calling this method.
    #[wasm_bindgen(js_name = signHydraTransaction)]
    pub fn sign_hydra_transaction(&self, hyd_addr: &str, tx: &JsValue) -> Result<JsValue, JsValue> {
        let mut tx: HydraTransactionData = from_value(tx.clone())?;
        self.inner
            .sign_hydra_transaction(hyd_addr, &mut tx)
            .context("Signing ITransactionData")
            .map_err_to_js()?;
        let signed_tx = to_value(&tx)?;
        Ok(signed_tx)
    }
}

impl From<HydraPrivate> for JsHydraPrivate {
    fn from(inner: HydraPrivate) -> Self {
        Self { inner }
    }
}

impl Wraps<HydraPrivate> for JsHydraPrivate {
    fn inner(&self) -> &HydraPrivate {
        &self.inner
    }
}
