use super::*;

/// Builder for core Hydra transactions on a given network.
#[wasm_bindgen(js_name = HydraTxBuilder)]
pub struct JsHydraTxBuilder {
    network: &'static dyn Network<Suite = Secp256k1>,
}

#[wasm_bindgen(js_class = HydraTxBuilder)]
impl JsHydraTxBuilder {
    /// Creates a transaction builder on the given network.
    ///
    /// @see allNetworkNames, validateNetworkName
    #[wasm_bindgen(constructor)]
    pub fn new(network_name: &str) -> Result<JsHydraTxBuilder, JsValue> {
        let network = Networks::by_name(network_name).map_err_to_js()?;
        Ok(Self { network })
    }

    /// Creates a token transfer transaction that moves amount of flakes (smallest denomination on the chain) from the balance that
    /// belongs to the sender {@SecpPublicKey} to the one that has the recipient address {@SecpKeyId}.
    ///
    /// The nonce of the sender needs to be known in advance and the next transaction must be 1 above the one of the last transaction
    /// made by the sender on-chain.
    ///
    /// Vendor field is a public memo attached to the transaction. The fee can be manually overriden, or the defaults will be
    /// calculated based on the size of the serialized transaction size and some offset based on the transaction type.
    ///
    /// @see SecpKeyId.fromAddress
    // TODO consider recipient SecpKeyId vs String
    pub fn transfer(
        &self, recipient_id: &JsSecpKeyId, sender_pubkey: &JsSecpPublicKey, amount_flake: u64,
        nonce: u64, vendor_field: Option<String>, manual_fee: Option<u64>,
    ) -> Result<JsValue, JsValue> {
        let common_fields = CommonTransactionFields {
            network: self.network,
            sender_public_key: sender_pubkey.inner().to_owned(),
            nonce,
            optional: OptionalTransactionFields { amount: amount_flake, vendor_field, manual_fee },
        };

        let transfer = hyd_core::Transaction::transfer(common_fields, recipient_id.inner());
        JsValue::from_serde(&transfer.to_data()).map_err_to_js()
    }

    /// Creates a vote transaction that empowers a delegate {@SecpPublicKey} to validate blocks and earn rewards for doing so.
    ///
    /// The nonce of the sender needs to be known in advance and the next transaction must be 1 above the one of the last transaction
    /// made by the sender on-chain.
    ///
    /// Vendor field is a public memo attached to the transaction. The fee can be manually overriden, or the defaults will be
    /// calculated based on the size of the serialized transaction size and some offset based on the transaction type.
    pub fn vote(
        &self, delegate: &JsSecpPublicKey, sender_pubkey: &JsSecpPublicKey, nonce: u64,
        vendor_field: Option<String>, manual_fee: Option<u64>,
    ) -> Result<JsValue, JsValue> {
        self.create_vote_tx(
            delegate,
            sender_pubkey,
            nonce,
            vendor_field,
            manual_fee,
            hyd_core::Transaction::vote,
        )
    }

    /// Creates an unvote transaction that revokes empowerment from a delegate {@SecpPublicKey} to validate blocks.
    ///
    /// The nonce of the sender needs to be known in advance and the next transaction must be 1 above the one of the last transaction
    /// made by the sender on-chain.
    ///
    /// Vendor field is a public memo attached to the transaction. The fee can be manually overriden, or the defaults will be
    /// calculated based on the size of the serialized transaction size and some offset based on the transaction type.
    pub fn unvote(
        &self, delegate: &JsSecpPublicKey, sender_pubkey: &JsSecpPublicKey, nonce: u64,
        vendor_field: Option<String>, manual_fee: Option<u64>,
    ) -> Result<JsValue, JsValue> {
        self.create_vote_tx(
            delegate,
            sender_pubkey,
            nonce,
            vendor_field,
            manual_fee,
            hyd_core::Transaction::unvote,
        )
    }

    /// Creates a transaction that registers a delegate so it can validate blocks and earn rewards for doing so. If there is not enough
    /// balance on the delegate's address, other addresses can vote for the delegate with their own balance and if the sum of these are
    /// in the top 53 (or the limit on the actual network), they can validate blocks in the coming rounds.
    ///
    /// The nonce of the sender needs to be known in advance and the next transaction must be 1 above the one of the last transaction
    /// made by the sender on-chain.
    ///
    /// Vendor field is a public memo attached to the transaction. The fee can be manually overriden, or the defaults will be
    /// calculated based on the size of the serialized transaction size and some offset based on the transaction type.
    #[wasm_bindgen(js_name = registerDelegate)]
    pub fn register_delegate(
        &self, sender_pubkey: &JsSecpPublicKey, delegate_name: &str, nonce: u64,
        vendor_field: Option<String>, manual_fee: Option<u64>,
    ) -> Result<JsValue, JsValue> {
        let common_fields = CommonTransactionFields {
            network: self.network,
            sender_public_key: sender_pubkey.inner().to_owned(),
            nonce,
            optional: OptionalTransactionFields { vendor_field, manual_fee, ..Default::default() },
        };

        let tx = hyd_core::Transaction::register_delegate(common_fields, delegate_name);
        JsValue::from_serde(&tx.to_data()).map_err_to_js()
    }

    fn create_vote_tx(
        &self, delegate: &JsSecpPublicKey, sender_pubkey: &JsSecpPublicKey, nonce: u64,
        vendor_field: Option<String>, manual_fee: Option<u64>,
        build_tx: fn(
            CommonTransactionFields<'static>,
            &SecpPublicKey,
        ) -> hyd_core::Transaction<'static>,
    ) -> Result<JsValue, JsValue> {
        let common_fields = CommonTransactionFields {
            network: self.network,
            sender_public_key: sender_pubkey.inner().to_owned(),
            nonce,
            optional: OptionalTransactionFields { vendor_field, manual_fee, ..Default::default() },
        };

        let vote = build_tx(common_fields, delegate.inner());
        JsValue::from_serde(&vote.to_data()).map_err_to_js()
    }
}
