mod plugin;
mod private;
mod public;
mod sign;
mod types;

pub use plugin::*;
pub use private::*;
pub use public::*;
pub use sign::*;
pub use types::*;

use super::*;

#[cfg(test)]
mod test {
    use super::*;

    use iop_keyvault::{secp256k1::hyd, Seed};
    use iop_vault::Vault;

    #[test]
    fn api() -> Result<()> {
        let unlock_password = "correct horse battery staple";
        let mut vault = Vault::create(None, Seed::DEMO_PHRASE, "", unlock_password)?;
        let parameters = Parameters::new(&hyd::Testnet, 0);
        vault::Plugin::init(&mut vault, unlock_password, &parameters)?;

        let hyd = vault::Plugin::get(&vault, &parameters)?;
        let mut hyd_priv = hyd.private(unlock_password)?;
        let priv_key_0 = hyd_priv.key(0)?;
        let pub_key_0 = priv_key_0.neuter();
        let pk0 = pub_key_0.to_public_key();

        assert_eq!(
            &pk0.to_string(),
            "02db11c07afd6ec05980284af58105329d41e9882947188022350219cca9baa3e7"
        );

        let addr0 = pub_key_0.to_p2pkh_addr();

        assert_eq!(&addr0, "tjMvaU79mMJ8fKwoLjFLn7rCTthpY6KxTx");

        let priv_key_0_by_pk = hyd_priv.key_by_pk(&pk0)?;

        assert_eq!(priv_key_0_by_pk.bip44_path().key(), 0);

        let err = Plugin::init(&mut vault, unlock_password, &parameters).unwrap_err();
        assert!((&err.to_string()).contains("was already added"));

        Ok(())
    }

    const DEMO_VAULT_DAT: &str = r#"
    {
        "encryptedSeed": "uKOE-HCgv-CUHFuL6jCUHMdXrfgGX-nsUM2FwE-5JY0GhSxOFTQSGB4F_N6VwuDYPQ8-q0Q_eQVCpgOsjRzqJAnr8nhyV32yNtpCsGYimpnEjr_enZDOd4jajLjt7b48J7V5yDKKVyp8",
        "plugins": [
            {
                "pluginName": "Hydra",
                "parameters": {
                    "network": "HYD testnet",
                    "account": 0
                },
                "publicState": {
                    "xpub": "hydtVxG6GvapCX2X1YxnwKWGzh8tKy6X56gQUN2KRVpqXkgZQYDE7jNw24ZK23ZXEow4cfJz41fBpRj1wV5mbLBYfdpcRgZuS4mSZ22LsVugPZFK",
                    "receiveKeys": 2,
                    "changeKeys": 0
                }
            }
        ]
    }"#;

    #[test]
    fn serialize() -> Result<()> {
        let unlock_password = "correct horse battery staple";
        let vault: Vault = serde_json::from_str(DEMO_VAULT_DAT)?;

        let hyd = vault::Plugin::get(&vault, &vault::Parameters::new(&hyd::Testnet, 0))?;

        let hyd_private = hyd.private(unlock_password)?;
        let hyd_pk: SecpPublicKey =
            "02db11c07afd6ec05980284af58105329d41e9882947188022350219cca9baa3e7".parse()?;
        let hyd0 = hyd_private.key_by_pk(&hyd_pk)?;
        let addr = hyd0.neuter().to_p2pkh_addr();

        assert_eq!(&addr, "tjMvaU79mMJ8fKwoLjFLn7rCTthpY6KxTx");

        Ok(())
    }
}
