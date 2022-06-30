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
        JwtBuilder { content_id: None, time_to_live: Duration::minutes(5), created_at: Utc::now() }
    }

    pub fn with_content_id(content_id: ContentId) -> Self {
        JwtBuilder { content_id: Some(content_id), ..Self::new() }
    }

    pub fn sign(&self, sk: &MPrivateKey) -> Result<String> {
        let pk = sk.public_key();
        let header = Header::default().with_key_id(pk.to_string());
        let options = TimeOptions::new(Duration::seconds(0), || self.created_at);
        let claims = Claims::new(JwtClaim { content_id: self.content_id.clone() })
            .set_duration(&options, self.time_to_live)
            .set_not_before(self.created_at);
        let token = JwtMultiCipher.token(header, &claims, sk)?;
        Ok(token)
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
        let options =
            TimeOptions::new(Duration::seconds(0), || current_time.unwrap_or_else(Utc::now));
        token.claims().validate_expiration(&options)?.validate_maturity(&options)?;

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
        *claims.expiration.as_ref().unwrap() - *claims.not_before.as_ref().unwrap()
    }

    // new would fail on validate_maturity if not_before was missing
    pub fn created_at(&self) -> &DateTime<Utc> {
        self.0.claims().not_before.as_ref().unwrap()
    }

    pub fn content_id(&self) -> Option<&ContentId> {
        self.0.claims().custom.content_id.as_ref()
    }
}
