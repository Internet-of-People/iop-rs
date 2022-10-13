use super::*;

use iop_hydra_proto::txtype::{coeus, Aip29Transaction, CommonTransactionFields};
use iop_keyvault::{secp256k1::Secp256k1, Network, Networks};

#[wasm_bindgen(js_name = CoeusTxBuilder)]
pub struct JsCoeusTxBuilder {
    network: &'static dyn Network<Suite = Secp256k1>,
}

#[wasm_bindgen(js_class = CoeusTxBuilder)]
impl JsCoeusTxBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(network_name: &str) -> Result<JsCoeusTxBuilder, JsValue> {
        let network = Networks::by_name(network_name).map_err_to_js()?;
        Ok(Self { network })
    }

    // TODO support multiple signed operations
    pub fn build(
        &self, ops: &JsSignedBundle, sender_pubkey: &JsSecpPublicKey, nonce: Nonce,
    ) -> Result<JsValue, JsValue> {
        let common_fields = CommonTransactionFields {
            network: self.network,
            sender_public_key: sender_pubkey.inner().to_owned(),
            nonce,
            optional: Default::default(),
        };

        let tx = coeus::Transaction::new(common_fields, vec![ops.inner().clone()]);
        let res = to_value(&tx.to_data().clone())?;
        Ok(res)
    }
}
