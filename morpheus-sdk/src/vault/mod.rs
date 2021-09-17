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

    use iop_keyvault::{PublicKey, Seed};
    use iop_morpheus_proto::data::Did;
    use iop_vault::Vault;

    #[test]
    fn api() -> Result<()> {
        let unlock_password = "correct horse battery staple";
        let mut vault = Vault::create(None, Seed::DEMO_PHRASE, "", unlock_password)?;
        Plugin::init(&mut vault, unlock_password)?;

        let morpheus = Plugin::get(&vault)?;
        let morpheus_priv = morpheus.private(unlock_password)?;
        let personas = morpheus_priv.personas()?;
        let persona_0 = personas.key(0)?;
        let pub_0 = persona_0.neuter();
        let pk0 = pub_0.public_key();

        assert_eq!(&pk0.to_string(), "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6");

        let id0 = pk0.key_id();

        assert_eq!(&id0.to_string(), "iezqztJ6XX6GDxdSgdiySiT3J");

        let persona_0_by_pk = morpheus_priv.key_by_pk(&pk0)?;

        assert_eq!(persona_0_by_pk.path().idx(), 0);

        let group_2_pk = morpheus_priv.kind(DidKind::Group)?.key_mut(2)?.neuter().public_key();

        assert_eq!(&group_2_pk.to_string(), "pezDh7bgpK7xvSXyiTUAMBmbfByEUjeikTB4DEYg5BeeSMo");

        let resource_20_pk = morpheus_priv.resources()?.key_mut(20)?.neuter().public_key();

        assert_eq!(&resource_20_pk.to_string(), "pezDf6Ev4nNAjY7XTsPPY2bxiqE7ts6XxBAA9z9NiYGcBRS");

        println!("{}", serde_json::to_string_pretty(&vault)?);

        let err = Plugin::init(&mut vault, unlock_password).unwrap_err();
        assert!((&err.to_string()).contains("was already added"));

        Ok(())
    }

    const DEMO_VAULT_DAT: &str = r#"
    {
        "encryptedSeed": "uKOE-HCgv-CUHFuL6jCUHMdXrfgGX-nsUM2FwE-5JY0GhSxOFTQSGB4F_N6VwuDYPQ8-q0Q_eQVCpgOsjRzqJAnr8nhyV32yNtpCsGYimpnEjr_enZDOd4jajLjt7b48J7V5yDKKVyp8",
        "plugins": [
            {
                "pluginName": "Morpheus",
                "parameters": {},
                "publicState": {
                    "personas": [
                        "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6",
                        "pezDj6ea4tVfNRUTMyssVDepAAzPW67Fe3yHtuHL6ZNtcfJ",
                        "pezsfLDb1fngso3J7TXU6jP3nSr2iubcJZ4KXanxrhs9gr"
                    ],
                    "groups": [
                        "pezChZUmPH4gcrtE37Rqy2gNGGUp8fScoWUNGjP9FxewxhN",
                        "pezFpP9psVFm6NpnKgaW63Unxg1YygFcV58ZN2CDNKkZKv9"
                    ]
                }
            }
        ]
    }"#;

    #[test]
    fn serialize() -> Result<()> {
        let unlock_password = "correct horse battery staple";
        let vault: Vault = serde_json::from_str(DEMO_VAULT_DAT)?;

        let m = vault::Plugin::get(&vault)?;

        let m_private = m.private(unlock_password)?;
        let m_pk_persona0: MPublicKey =
            "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6".parse()?;
        let persona0 = m_private.key_by_pk(&m_pk_persona0)?;
        let did = Did::from(persona0.neuter().public_key().key_id());

        assert_eq!(&did.to_string(), "did:morpheus:ezqztJ6XX6GDxdSgdiySiT3J");

        let m_pk_group2: MPublicKey = "pezFpP9psVFm6NpnKgaW63Unxg1YygFcV58ZN2CDNKkZKv9".parse()?;
        let m_id_group2 = m_pk_group2.key_id();

        let found_pk = m_private.public().key_by_id(&m_id_group2)?;

        assert_eq!(m_pk_group2, found_pk);

        Ok(())
    }
}
