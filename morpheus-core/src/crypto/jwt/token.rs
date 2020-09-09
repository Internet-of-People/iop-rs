use serde::{Deserialize, Serialize};

use super::hash::ContentId;

use super::*;
use anyhow::Context;

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

#[cfg(test)]
mod test {
    use chrono::{TimeZone as _, Timelike as _};

    use super::*;

    use hd::{morpheus::Plugin as MorpheusPlugin, Vault};
    use iop_keyvault::{ed25519::MorpheusPrivateKey, Seed};

    const TOKEN: &str = "eyJhbGciOiJNdWx0aWNpcGhlciIsImtpZCI6InBlejJDTGtCVWpIQjh3OEc4N0QzWWtSRWpwUnVpcVB1NkJyUnNnSE1ReTJQenQ2In0.eyJleHAiOjE1OTYxOTU1NjcsIm5iZiI6MTU5NjE5NTI2NywianRpIjoiY2p1cHFxdVJSYWcybEtUV0FqZS1mRGdvcllVQkVuNE5pNks4Uk11TmhYV05hOCJ9.c2V6NmlTQWU0TGE1NllveHhHREdod2NOYzZNZFZQOWhIUzdTN2g4ZU1WUW9jNTVMS1RrZ0pTUU52eG5VNHV2RGV2YXhWRjN2Q2MyWHYyY1hYekp5YmZNQ3FBMg";
    const CONTENT_ID: &str = "cjupqquRRag2lKTWAje-fDgorYUBEn4Ni6K8RMuNhXWNa8";

    fn test_now() -> DateTime<Utc> {
        Utc.timestamp(1596195267, 0)
    }

    fn persona() -> Result<MorpheusPrivateKey> {
        let unlock_pw = "correct horse battery staple";
        let word25 = "";
        let mut vault = Vault::create(Some("en"), Seed::DEMO_PHRASE, word25, unlock_pw)?;
        MorpheusPlugin::rewind(&mut vault, unlock_pw)?;
        let morpheus = MorpheusPlugin::get(&vault)?;
        let persona0 = morpheus.private(unlock_pw)?.personas()?.key(0)?;

        Ok(persona0)
    }

    #[test]
    fn builder() -> Result<()> {
        let mut builder = JwtBuilder::with_content_id(CONTENT_ID.to_owned());
        builder.created_at = test_now();
        let token = builder.sign(&persona()?.private_key())?;

        assert_eq!(token, TOKEN);

        Ok(())
    }

    #[test]
    fn parser() -> Result<()> {
        let token = JwtParser::new(TOKEN, Some(test_now()))?;

        assert_eq!(token.public_key(), persona()?.neuter().public_key());
        assert_eq!(token.time_to_live(), Duration::minutes(5));
        assert_eq!(token.created_at(), &test_now());
        assert_eq!(token.content_id().unwrap(), CONTENT_ID);

        Ok(())
    }

    #[test]
    fn roundtrip() -> Result<()> {
        let persona = persona()?;
        let builder = JwtBuilder::default();
        let serialized = builder.sign(&persona.private_key())?;
        let deserialized = JwtParser::new(serialized, None)?;

        assert_eq!(deserialized.public_key(), persona.neuter().public_key());
        assert_eq!(deserialized.time_to_live(), builder.time_to_live);
        assert_eq!(deserialized.created_at(), &builder.created_at.with_nanosecond(0).unwrap());
        assert_eq!(deserialized.content_id(), None);

        Ok(())
    }
}
