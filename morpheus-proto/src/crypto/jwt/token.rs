use anyhow::Context;
use serde::{Deserialize, Serialize};

use super::hash::ContentId;
use super::*;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
struct JwtClaim {
    #[serde(rename = "jti", skip_serializing_if = "Option::is_none", default)]
    content_id: Option<ContentId>,
}

pub struct JwtBuilder {
    pub content_id: Option<ContentId>,
    pub time_to_live: Duration,
    pub created_at: DateTime<Utc>,
}

impl JwtBuilder {
    pub fn new() -> Self {
        Self::create(None)
    }

    pub fn with_content_id(content_id: ContentId) -> Self {
        Self::create(Some(content_id))
    }

    pub fn sign(&self, sk: &MPrivateKey) -> Result<String> {
        let pk = sk.public_key();
        let header = Header { key_id: Some(pk.to_string()), ..Default::default() };
        let claims = Claims {
            expiration_date: Some(self.created_at + self.time_to_live),
            not_before: Some(self.created_at),
            issued_at: None,
            custom: JwtClaim { content_id: self.content_id.clone() },
        };
        let token = JwtMultiCipher.token(header, &claims, &sk)?;
        Ok(token)
    }

    fn create(content_id: Option<ContentId>) -> Self {
        JwtBuilder { content_id, time_to_live: Duration::minutes(5), created_at: Utc::now() }
    }
}

impl Default for JwtBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct JwtParser(Token<JwtClaim>);

impl JwtParser {
    pub fn new(token: impl AsRef<str>, current_time: Option<DateTime<Utc>>) -> Result<Self> {
        let untrusted = UntrustedToken::try_from(token.as_ref())?;
        let pk_str = untrusted
            .header()
            .key_id
            .as_ref()
            .with_context(|| "Publickey is missing from JWT kid header")?;
        let pk: MPublicKey = pk_str.parse()?;
        let token = JwtMultiCipher.validate_integrity::<JwtClaim>(&untrusted, &pk)?;
        let options = TimeOptions { current_time, ..Default::default() };
        token.claims().validate_expiration(options)?.validate_maturity(options)?;

        Ok(Self(token))
    }

    // new would fail if key_id would be missing or unparsable
    pub fn public_key(&self) -> MPublicKey {
        let pk_str = self.0.header().key_id.as_ref().unwrap();
        let pk: MPublicKey = pk_str.parse().unwrap();
        pk
    }

    // new would fail on validate_maturity or validate_expiration if either not_before or expiration_date were missing
    pub fn time_to_live(&self) -> Duration {
        let claims = self.0.claims();
        *claims.expiration_date.as_ref().unwrap() - *claims.not_before.as_ref().unwrap()
    }

    // new would fail on validate_maturity if not_before was missing
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.0.claims().not_before.as_ref().unwrap()
    }

    pub fn content_id(&self) -> Option<&ContentId> {
        self.0.claims().custom.content_id.as_ref()
    }
}
