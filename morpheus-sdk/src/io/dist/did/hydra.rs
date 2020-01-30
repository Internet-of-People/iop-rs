use async_trait::async_trait;
use failure::{err_msg, Fallible};
use hyper::{
    client::{Client, HttpConnector},
    Body, Request, Response, StatusCode,
};

use super::*;
use crate::crypto::{
    hash::ContentId,
    sign::{AfterEnvelope, Signable, Signed},
};
use crate::data::{did::Did, diddoc::DidDocument};
use keyvault::multicipher::MKeyId;

pub struct HydraDidLedger {
    url: String,
    client: Client<HttpConnector>,
}

impl HydraDidLedger {
    /// You can instantiate a light client for the Hydra chain passing in a URL to the wallet.
    ///
    /// - local testnet: http://127.0.0.1:4703
    /// - IoP testnet: http://35.187.56.222:4703
    /// - An IoP devnet node: http://35.240.62.119:4703
    /// - An IoP mainnet node: http://35.195.150.223:4703
    pub fn new(url: impl AsRef<str>) -> Self {
        let url = url.as_ref().to_owned();
        let client = Client::default();
        Self { url, client }
    }
}

#[async_trait(?Send)]
impl LedgerQueries for HydraDidLedger {
    async fn validate<T: Signable>(
        &self, _on_behalf_of: &Did, _signer_id: Option<MKeyId>, _signed: &Signed<T>,
    ) -> Fallible<ValidationStatus> {
        todo!()
    }

    async fn validate_timeproofed<T: Signable>(
        &self, _on_behalf_of: &Did, _signer_id: Option<MKeyId>, _signed: &AfterEnvelope<T>,
    ) -> Fallible<ValidationStatus> {
        todo!()
    }

    async fn before_proof_exists(&self, _content: &ContentId) -> Fallible<bool> {
        todo!()
    }

    async fn document(&self, did: &Did) -> Fallible<DidDocument> {
        let endpoint = format!("{}/did/{}/document", self.url, did);

        let request = Request::get(endpoint)
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap();

        let response: Response<Body> = self.client.request(request).await?;
        let (header, body) = response.into_parts();

        if header.status != StatusCode::OK {
            return Err(err_msg(format!(
                "GET document {} failed with status {}",
                did, header.status
            )));
        }
        let bytes = hyper::body::to_bytes(body).await?;

        let doc: DidDocument = serde_json::from_slice(&bytes)?;

        Ok(doc)
    }
}

#[async_trait(?Send)]
impl LedgerOperations for HydraDidLedger {
    async fn send_transaction(
        &self, _operations: &[OperationAttempt],
    ) -> Fallible<Box<dyn PooledLedgerTransaction>> {
        todo!()
    }
}
