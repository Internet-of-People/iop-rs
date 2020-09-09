use super::{
    secp256k1::{ark, btc, hyd, iop, Secp256k1},
    *,
};

/// A registry of all networks implemented in this crate.
pub struct Networks;

impl Networks {
    /// Returns all networks implemented in this crate.
    pub const ALL: &'static [&'static dyn Network<Suite = Secp256k1>] = &[
        &ark::Mainnet,
        &ark::Devnet,
        &ark::Testnet,
        &btc::Mainnet,
        &btc::Testnet,
        &hyd::Mainnet,
        &hyd::Devnet,
        &hyd::Testnet,
        &iop::Mainnet,
        &iop::Testnet,
    ];

    /// Looks up a single network by its name.
    pub fn by_name(name: &str) -> Result<&'static dyn Network<Suite = Secp256k1>> {
        Self::ALL
            .iter()
            .find(|n| n.name() == name)
            .copied()
            .ok_or_else(|| anyhow!("Could not find network with name {}.", name))
    }
}
