use super::*;

/// Parameters of a Hydra account added to a {@link Vault}
#[wasm_bindgen(js_name = HydraParameters)]
pub struct JsHydraParameters {
    inner: HydraParameters,
}

#[wasm_bindgen(js_class = HydraParameters)]
impl JsHydraParameters {
    /// Creates a parameter object for a Hydra account. The network name needs to be one of {@link allNetworkNames} and the account
    /// index must be an 31-bit non-negative number. Most wallets use only a few accounts, and there is no way yet to name the account
    /// in the current version.
    ///
    /// Note that there is a negligable chance that the given account cannot be used with the given seed, in which case an error is
    /// thrown and another index needs to be tried.
    #[wasm_bindgen(constructor)]
    pub fn new(network: &str, account: i32) -> Result<JsHydraParameters, JsValue> {
        let network = Networks::by_name(network).map_err_to_js()?;
        let inner = HydraParameters::new(network, account);
        Ok(JsHydraParameters::from(inner))
    }
}

impl From<HydraParameters> for JsHydraParameters {
    fn from(inner: HydraParameters) -> Self {
        Self { inner }
    }
}

impl Wraps<HydraParameters> for JsHydraParameters {
    fn inner(&self) -> &HydraParameters {
        &self.inner
    }
}
