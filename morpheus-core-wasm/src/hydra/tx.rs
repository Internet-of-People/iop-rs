use super::*;

use iop_keyvault::{secp256k1::Secp256k1, Network};
use iop_morpheus_core::hydra::txtype::{hyd_core, Aip29Transaction, CommonTransactionFields};

#[wasm_bindgen(js_name = HydraTxBuilder)]
pub struct JsHydraTxBuilder {
    network: &'static dyn Network<Suite = Secp256k1>,
}

#[wasm_bindgen(js_class = HydraTxBuilder)]
impl JsHydraTxBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(network_name: &str) -> Result<JsHydraTxBuilder, JsValue> {
        let network = Networks::by_name(network_name).map_err_to_js()?;
        Ok(Self { network })
    }

    // TDOO consider recipient SecpKeyId vs String
    pub fn transfer(
        &self, recipient_id: &JsSecpKeyId, sender_pubkey: JsSecpPublicKey, amount_flake: u64,
        nonce: u64,
    ) -> Result<JsValue, JsValue> {
        let common_fields = CommonTransactionFields {
            network: self.network,
            sender_public_key: sender_pubkey.stringify(),
            amount: amount_flake,
            nonce,
            ..Default::default()
        };

        let recipient_id = recipient_id.inner().to_p2pkh_addr(self.network.p2pkh_addr());
        let transfer = hyd_core::Transaction::new_transfer(common_fields, recipient_id);
        JsValue::from_serde(&transfer.to_data()).map_err_to_js()
    }

    pub fn vote(
        &self, vote: &str, sender_pubkey: JsSecpPublicKey, nonce: u64,
    ) -> Result<JsValue, JsValue> {
        let common_fields = CommonTransactionFields {
            network: self.network,
            sender_public_key: sender_pubkey.stringify(),
            nonce,
            ..Default::default()
        };

        let vote = hyd_core::Transaction::new_vote(common_fields, vote);
        JsValue::from_serde(&vote.to_data()).map_err_to_js()
    }
}
