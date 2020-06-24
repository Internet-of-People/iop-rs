mod plugin;
mod private;
mod public;
mod types;

pub use plugin::*;
pub use private::*;
pub use public::*;
pub use types::*;

use super::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn api() -> Fallible<()> {
        let unlock_password = "correct horse battery staple";
        let mut vault = Vault::create(Seed::DEMO_PHRASE, "", unlock_password)?;
        let parameters = Parameters::new(&hyd::Testnet, 0);
        hydra::Plugin::rewind(&mut vault, unlock_password, &parameters)?;

        let hyd = hydra::Plugin::get(&vault, &parameters)?;
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

        let err = Plugin::rewind(&mut vault, unlock_password, &parameters).unwrap_err();
        assert!((&err.to_string()).contains("was already added"));

        Ok(())
    }
}
