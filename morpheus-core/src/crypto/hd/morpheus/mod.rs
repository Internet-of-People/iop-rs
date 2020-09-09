mod plugin;
mod private;
mod private_kind;
mod public;
mod public_kind;
mod types;

pub use plugin::*;
pub use private::*;
pub use private_kind::*;
pub use public::*;
pub use public_kind::*;
pub use types::*;

use super::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn api() -> Result<()> {
        let unlock_password = "correct horse battery staple";
        let mut vault = Vault::create(None, Seed::DEMO_PHRASE, "", unlock_password)?;
        Plugin::rewind(&mut vault, unlock_password)?;

        let morpheus = Plugin::get(&vault)?;
        let morpheus_priv = morpheus.private(unlock_password)?;
        let mut personas = morpheus_priv.personas()?;
        let persona_0 = personas.key(0)?;
        let pub_0 = persona_0.neuter();
        let pk0 = pub_0.public_key();

        assert_eq!(&pk0.to_string(), "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6");

        let id0 = pk0.key_id();

        assert_eq!(&id0.to_string(), "iezqztJ6XX6GDxdSgdiySiT3J");

        let persona_0_by_pk = morpheus_priv.key_by_pk(&pk0)?;

        assert_eq!(persona_0_by_pk.path().idx(), 0);

        let err = Plugin::rewind(&mut vault, unlock_password).unwrap_err();
        assert!((&err.to_string()).contains("was already added"));

        Ok(())
    }
}
