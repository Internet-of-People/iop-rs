use super::*;

#[wasm_bindgen(js_name = CoeusState)]
pub struct JsState {
    inner: State,
}

#[wasm_bindgen(js_class = CoeusState)]
impl JsState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsState, JsValue> {
        let inner = State::new();
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = resolveData)]
    pub fn resolve_data(&self, name: &JsDomainName) -> Result<JsValue, JsValue> {
        let data = self.inner.resolve_data(name.inner()).map_err_to_js()?;
        JsValue::from_serde(data).map_err_to_js()
    }

    #[wasm_bindgen(js_name = getMetadata)]
    pub fn get_metadata(&self, name: &JsDomainName) -> Result<JsValue, JsValue> {
        let domain = self.inner.domain(name.inner()).map_err_to_js()?;

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Metadata<'a> {
            owner: &'a Principal,
            subtree_policies: &'a SubtreePolicies,
            registration_policy: &'a RegistrationPolicy,
            expires_at_height: BlockHeight,
        }

        let metadata = Metadata {
            owner: domain.owner(),
            subtree_policies: domain.subtree_policies(),
            registration_policy: domain.registration_policy(),
            expires_at_height: domain.expires_at_height(),
        };

        JsValue::from_serde(&metadata).map_err_to_js()
    }

    #[wasm_bindgen(js_name = getChildren)]
    pub fn get_children(&self, name: &JsDomainName) -> Result<JsValue, JsValue> {
        let domain = self.inner.domain(name.inner()).map_err_to_js()?;
        JsValue::from_serde(&domain.child_names()).map_err_to_js()
    }

    #[wasm_bindgen(js_name = lastNonce)]
    pub fn last_nonce(&self, pk: &JsMPublicKey) -> u64 {
        return self.inner.nonce(pk.inner());
    }

    #[wasm_bindgen(js_name = applyTransaction)]
    pub fn apply_transaction(&mut self, txid: &str, asset: &JsCoeusAsset) -> Result<(), JsValue> {
        self.inner.apply_transaction(txid, asset.inner().to_owned()).map_err_to_js()
    }

    #[wasm_bindgen(js_name = revertTransaction)]
    pub fn revert_transaction(&mut self, txid: &str, asset: &JsCoeusAsset) -> Result<(), JsValue> {
        self.inner.revert_transaction(txid, asset.inner().to_owned()).map_err_to_js()
    }

    #[wasm_bindgen(js_name = blockApplying)]
    pub fn block_applying(&mut self, height: BlockHeight) -> Result<(), JsValue> {
        self.inner.block_applying(height).map_err_to_js()
    }

    #[wasm_bindgen(js_name = blockReverted)]
    pub fn block_reverted(&mut self, height: BlockHeight) -> Result<(), JsValue> {
        self.inner.block_reverted(height).map_err_to_js()
    }

    #[wasm_bindgen(getter = corrupted)]
    pub fn is_corrupted(&self) -> bool {
        self.inner.is_corrupted()
    }

    #[wasm_bindgen(getter)]
    pub fn version(&self) -> Version {
        self.inner.version()
    }

    #[wasm_bindgen(getter = lastSeenHeight)]
    pub fn last_seen_height(&self) -> BlockHeight {
        self.inner.last_seen_height()
    }

    #[wasm_bindgen(js_name = getTxnStatus)]
    pub fn get_txn_status(&self, txid: &str) -> Result<bool, JsValue> {
        let status = self.inner.get_txn_status(txid).map_err_to_js()?;
        Ok(status.success)
    }

    // #[wasm_bindgen(js_name = toString)]
    // pub fn stringify(&self) -> String {
    //     self.inner.to_string()
    // }
}

impl From<State> for JsState {
    fn from(inner: State) -> Self {
        Self { inner }
    }
}

impl Wraps<State> for JsState {
    fn inner(&self) -> &State {
        &self.inner
    }
}
