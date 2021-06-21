use super::*;

#[wasm_bindgen(js_name = MorpheusState)]
pub struct JsMorpheusState {
    inner: MorpheusState,
}

#[wasm_bindgen(js_class = MorpheusState)]
impl JsMorpheusState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsMorpheusState, JsValue> {
        let inner = MorpheusState::new();
        Ok(Self { inner })
    }

    #[wasm_bindgen(getter = corrupted)]
    pub fn is_corrupted(&self) -> bool {
        self.inner.is_corrupted()
    }

    #[wasm_bindgen(js_name = lastBlockHeight)]
    pub fn last_seen_height(&self) -> Result<BlockHeight, JsValue> {
        let state = self.inner.state().map_err_to_js()?;
        Ok(state.last_seen_height())
    }

    #[wasm_bindgen(js_name = isConfirmed)]
    pub fn is_confirmed(&self, txid: &str) -> Result<Option<bool>, JsValue> {
        let state = self.inner.state().map_err_to_js()?;
        Ok(state.is_confirmed(txid))
    }

    #[wasm_bindgen(js_name = beforeProofExistsAt)]
    pub fn before_proof_exists_at(
        &self, content_id: &str, height_opt: Option<BlockHeight>,
    ) -> Result<bool, JsValue> {
        if let Some(height) = height_opt {
            Self::check_height(height)?;
        }
        let state = self.inner.state().map_err_to_js()?;
        Ok(state.before_proof_exists_at(content_id, height_opt))
    }

    #[wasm_bindgen(js_name = beforeProofHistory)]
    pub fn before_proof_history(&self, content_id: &str) -> Result<JsValue, JsValue> {
        let state = self.inner.state().map_err_to_js()?;
        let history = state.before_proof_history(content_id);
        let js_history = JsValue::from_serde(&history).map_err_to_js()?;
        Ok(js_history)
    }

    #[wasm_bindgen(js_name = getTransactionHistory)]
    pub fn get_tx_ids(
        &self, did: &str, include_attempts: bool, from_height_inc: BlockHeight,
        until_height_inc: Option<BlockHeight>,
    ) -> Result<JsValue, JsValue> {
        if let Some(height) = until_height_inc {
            Self::check_height(height)?;
        }
        let state = self.inner.state().map_err_to_js()?;
        let js_vec_opt = state
            .get_tx_ids(did, include_attempts, from_height_inc, until_height_inc)
            .map(|a| JsValue::from_serde(&a.collect::<Vec<_>>()))
            .unwrap_or_else(|| JsValue::from_serde(&([] as [TransactionIdWithHeight; 0])));
        js_vec_opt.map_err_to_js()
    }

    #[wasm_bindgen(js_name = lastTxId)]
    pub fn last_tx_id(&self, did: &str) -> Result<Option<String>, JsValue> {
        let state = self.inner.state().map_err_to_js()?;
        let height_opt = state.last_tx_id(did).map(|t| t.transaction_id.clone());
        Ok(height_opt)
    }

    #[wasm_bindgen(js_name = getDidDocumentAt)]
    pub fn get_doc_at(
        &self, did_data: &str, height_opt: Option<BlockHeight>,
    ) -> Result<JsValue, JsValue> {
        if let Some(height) = height_opt {
            Self::check_height(height)?;
        }
        let state = self.inner.state().map_err_to_js()?;
        let doc = state.get_doc_at(did_data, height_opt).map_err_to_js()?;
        let js_doc = JsValue::from_serde(&doc).map_err_to_js()?;
        Ok(js_doc)
    }

    #[wasm_bindgen(js_name = dryRun)]
    pub fn dry_run(&self, asset: &JsValue) -> Result<Vec<JsValue>, JsValue> {
        let asset: MorpheusAsset = asset.into_serde().map_err_to_js()?;
        let errs = self.inner.dry_run(&asset).map_err_to_js()?;
        let js_errs = errs
            .iter()
            .try_fold(
                Vec::<JsValue>::with_capacity(errs.len()),
                |mut v, err| -> serde_json::Result<_> {
                    v.push(JsValue::from_serde(err)?);
                    Ok(v)
                },
            )
            .map_err_to_js()?;
        Ok(js_errs)
    }

    fn check_height(height: BlockHeight) -> Result<(), JsValue> {
        if height > i32::MAX as u32 {
            return Err(JsValue::from(format!("Blockheight cannot be negative: {}", height)));
        }
        Ok(())
    }

    #[wasm_bindgen(js_name = blockApplying)]
    pub fn block_applying(&mut self, height: BlockHeight) -> Result<(), JsValue> {
        Self::check_height(height)?;
        self.inner.block_applying(height).map_err_to_js()
    }

    #[wasm_bindgen(js_name = applyTransaction)]
    pub fn apply_transaction(&mut self, txid: &str, asset: &JsValue) -> Result<(), JsValue> {
        let asset: MorpheusAsset = asset.into_serde().map_err_to_js()?;
        self.inner.apply_transaction(txid, &asset).map_err_to_js()
    }

    #[wasm_bindgen(js_name = blockReverting)]
    pub fn block_reverting(&mut self, height: BlockHeight) -> Result<(), JsValue> {
        Self::check_height(height)?;
        self.inner.block_reverting(height).map_err_to_js()
    }

    #[wasm_bindgen(js_name = revertTransaction)]
    pub fn revert_transaction(&mut self, txid: &str, asset: &JsValue) -> Result<(), JsValue> {
        let asset: MorpheusAsset = asset.into_serde().map_err_to_js()?;
        self.inner.revert_transaction(txid, &asset).map_err_to_js()
    }
}

impl Wraps<MorpheusState> for JsMorpheusState {
    fn inner(&self) -> &MorpheusState {
        &self.inner
    }
}

impl From<MorpheusState> for JsMorpheusState {
    fn from(inner: MorpheusState) -> Self {
        Self { inner }
    }
}
