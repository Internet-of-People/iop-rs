pub mod vault;

// imports from standard library

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// imports from 3rd party crates

use anyhow::{bail, ensure, Context, Result};
//use log::*;
use serde::{Deserialize, Serialize};

// imports from own crates

use iop_keyvault::{
    ed25519::{DidKind, Ed25519, Morpheus, MorpheusKind, MorpheusPrivateKey, MorpheusRoot},
    multicipher::{MKeyId, MPublicKey},
    Bip32Node, PublicKey as _, Seed,
};
use iop_vault::{BoundPlugin, PluginPrivate, PluginPublic, State, Vault, VaultPlugin};

#[cfg(test)]
mod test {
    use super::*;

    use chrono::{DateTime, Duration, TimeZone as _, Timelike as _, Utc};

    use crate::vault::Plugin as MorpheusPlugin;
    use iop_keyvault::{ed25519::MorpheusPrivateKey, Seed};
    use iop_morpheus_core::crypto::jwt::*;
    use iop_vault::Vault;

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
