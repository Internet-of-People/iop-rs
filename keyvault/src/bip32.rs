//! Generic data structures and algorithms for [BIP-0032](
//! https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) and
//! [SLIP-0010](https://github.com/satoshilabs/slips/blob/master/slip-0010.md) compatible
//! child-key derivation for building hierarchical deterministic wallets.

use std::str::FromStr;

use super::*;

/// Entry point to generate extended private keys in a hierarchical deterministic wallet starting from a seed based
/// on the BIP-0032 standard (and the SLIP-0010 for crypto suites other than Secp256k1).
#[derive(Clone, Debug)]
pub struct Bip32;

impl Bip32 {
    /// Calculates the master extended private key based on the crypto suite used by a given subtree.
    pub fn master<C: KeyDerivationCrypto>(
        &self, seed: &Seed, subtree: &'static dyn Subtree<Suite = C>,
    ) -> Bip32Node<C> {
        let path = Default::default();
        let xsk = subtree.master(seed);
        Bip32Node { path, xsk, subtree }
    }
}

#[derive(Clone)]
/// In BIP-0032 each extended private key has the same operations, independently from the actual path. This struct represents such an extended private
/// key in a given subtree.
pub struct Bip32Node<C: KeyDerivationCrypto + 'static> {
    path: bip32::Path,
    xsk: <C as KeyDerivationCrypto>::ExtendedPrivateKey,
    subtree: &'static dyn Subtree<Suite = C>,
}

impl<C: KeyDerivationCrypto + 'static> fmt::Debug for Bip32Node<C> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        formatter
            .debug_struct("Bip32Node")
            .field("path", &self.path)
            .field("xsk", &"...")
            .field("subtree", &self.subtree)
            .finish()
    }
}

impl<C: KeyDerivationCrypto + 'static> Bip32Node<C> {
    /// Backdoor for advanced or fringe non-standard use-cases. Use [`Bip32.master`] instead for normal use-cases.
    pub fn new(
        path: bip32::Path, xsk: <C as KeyDerivationCrypto>::ExtendedPrivateKey,
        subtree: &'static dyn Subtree<Suite = C>,
    ) -> Self {
        Self { path, xsk, subtree }
    }

    /// Accessor for the BIP32 path of this node
    pub fn path(&self) -> &bip32::Path {
        &self.path
    }

    /// Accessor for the extended private key of this node
    pub fn xsk(&self) -> &<C as KeyDerivationCrypto>::ExtendedPrivateKey {
        &self.xsk
    }

    /// Accessor for the subtree of this node
    pub fn subtree(&self) -> &'static dyn Subtree<Suite = C> {
        self.subtree
    }

    /// Removes the ability to sign and derive hardened keys. The public node it returns is still able to provide
    /// normal derivation and signature verifications.
    pub fn neuter(&self) -> Bip32PublicNode<C> {
        let xpk = self.xsk.neuter();
        Bip32PublicNode::new(self.path.clone(), xpk, self.subtree)
    }

    /// Create a new node with normal (public) derivation with the given index.
    pub fn derive_normal(&self, idx: i32) -> Result<Bip32Node<C>> {
        let path = self.path.append(ChildIndex::Normal(idx));
        let xsk = self.xsk.derive_normal_child(idx)?;
        let subtree = self.subtree;
        Ok(Self { path, xsk, subtree })
    }

    /// Create a new node with hardened (private) derivation with the given index.
    pub fn derive_hardened(&self, idx: i32) -> Result<Bip32Node<C>> {
        let path = self.path.append(ChildIndex::Hardened(idx));
        let xsk = self.xsk.derive_hardened_child(idx)?;
        let subtree = self.subtree;
        Ok(Self { path, xsk, subtree })
    }

    /// Creates the private key that belongs to this node for authenticating actions.
    pub fn private_key(&self) -> C::PrivateKey {
        self.xsk.private_key()
    }
}

#[derive(Clone)]
/// In BIP-0032 a neutered extended private key is an extended public key. This struct represents
/// such an extended public key in a given subtree. It is able to do normal (public) derivation,
/// signature verification, creating and validating key identifiers
pub struct Bip32PublicNode<C: KeyDerivationCrypto + 'static> {
    path: bip32::Path,
    xpk: <C as KeyDerivationCrypto>::ExtendedPublicKey,
    subtree: &'static dyn Subtree<Suite = C>,
}

impl<C: KeyDerivationCrypto + 'static> Bip32PublicNode<C> {
    /// Backdoor for advanced or fringe non-standard use-cases. Use [`Bip32.master`] instead for normal use-cases.
    pub fn new(
        path: bip32::Path, xpk: <C as KeyDerivationCrypto>::ExtendedPublicKey,
        subtree: &'static dyn Subtree<Suite = C>,
    ) -> Self {
        Self { path, xpk, subtree }
    }

    /// Accessor for the BIP32 path of this node
    pub fn path(&self) -> &bip32::Path {
        &self.path
    }

    /// Accessor for the extended public key of this node
    pub fn xpk(&self) -> &<C as KeyDerivationCrypto>::ExtendedPublicKey {
        &self.xpk
    }

    /// Accessor for the subtree of this node
    pub fn subtree(&self) -> &'static dyn Subtree<Suite = C> {
        self.subtree
    }

    /// Create a new node with normal (public) derivation with the given index.
    pub fn derive_normal(&self, idx: i32) -> Result<Bip32PublicNode<C>> {
        let path = self.path.append(ChildIndex::Normal(idx));
        let xpk = self.xpk.derive_normal_child(idx)?;
        let subtree = self.subtree;
        Ok(Self { path, xpk, subtree })
    }

    /// Creates the public key that belongs to this node for verifying authentications done by the private key.
    pub fn public_key(&self) -> C::PublicKey {
        self.xpk.public_key()
    }

    /// Creates the key identifier for the public key. This is an extra layer of security for single-use keys, so the
    /// revealing of the public key can be delayed to the point when the authenticated action (spending some coin or
    /// revoking access) makes the public key irrelevant after the action is successful.
    pub fn key_id(&self) -> C::KeyId {
        let pk = self.public_key();
        self.subtree.key_id(&pk)
    }
}

impl<C: KeyDerivationCrypto + 'static> fmt::Debug for Bip32PublicNode<C> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        formatter
            .debug_struct("Bip32PublicNode")
            .field("path", &self.path)
            .field("xpk", &"...")
            .field("subtree", &self.subtree)
            .finish()
    }
}

/// An item in the [BIP-0032](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
/// derivation [path](struct.Path.html). A combination of a 31-bit unsigned integer and a flag, which derivation
/// method (normal or hardened) to use.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChildIndex {
    /// Normal (aka. public) derivation allows deriving a child extended public key
    /// based on a parent extended public key.
    Normal(i32),
    /// Hardened (aka. private) derivation only allows deriving a child extended private key
    /// based on a parent extended private key, but having only an extended public key does
    /// not help deriving hardened children of any kind.
    Hardened(i32),
}

fn is_hardened_suffix_char(c: char) -> bool {
    ['\'', 'h', 'H'].contains(&c)
}

impl FromStr for ChildIndex {
    type Err = anyhow::Error;
    fn from_str(mut src: &str) -> Result<Self> {
        let hardened = src.ends_with(is_hardened_suffix_char);
        if hardened {
            src = &src[..src.len() - 1];
        };
        let idx = src.parse::<i32>()?;
        if idx < 0 {
            bail!("BIP32 derivation index cannot be negative");
        }
        Ok(if hardened { ChildIndex::Hardened(idx) } else { ChildIndex::Normal(idx) })
    }
}

impl fmt::Display for ChildIndex {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ChildIndex::Normal(idx) => formatter.write_fmt(format_args!("{}", idx)),
            ChildIndex::Hardened(idx) => formatter.write_fmt(format_args!("{}'", idx)),
        }
    }
}

/// An absolute [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) derivation
/// path that starts from the master keypair. This is useful to create a [hierarchical deterministic
/// tree](https://bitcoin.org/en/developer-guide#hierarchical-deterministic-key-creation) of keypairs
/// for [any cryptography](https://github.com/satoshilabs/slips/blob/master/slip-0010.md) that supports
/// child key derivation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Path {
    path: Vec<ChildIndex>,
}

impl Path {
    /// Creates a new path that has a new derivation index appended to the current path
    pub fn append(&self, child: ChildIndex) -> Self {
        let mut path = self.path.clone();
        path.push(child);
        Self { path }
    }
}

impl FromStr for Path {
    type Err = anyhow::Error;
    fn from_str(src: &str) -> Result<Self> {
        let mut pieces = src.split('/');

        let first_opt = pieces.next();
        if let Some(first) = first_opt {
            if first != "m" && first != "M" {
                bail!("BIP32 derivation path needs to start with 'm'");
            }
        } else {
            bail!("BIP32 derivation path cannot be empty");
        }

        let (mut successes, errors): (Vec<_>, Vec<_>) =
            pieces.map(|p: &str| (p, p.parse::<ChildIndex>())).partition(|(_p, i)| i.is_ok());

        if !errors.is_empty() {
            bail!("BIP32 derivation path contains invalid child indices: {:?}", errors);
        }

        // because of the above partitioning, successes only contain parse results
        // that can be unwrapped without causing a panic
        let path = successes.drain(..).map(|(_p, i)| i.unwrap()).collect();
        Ok(Path { path })
    }
}

impl From<Vec<ChildIndex>> for Path {
    fn from(path: Vec<ChildIndex>) -> Self {
        Self { path }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use fmt::Write as _;
        formatter.write_char('m')?;
        for c in &self.path {
            write!(formatter, "/{}", c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn childidx_fromstr() {
        assert_eq!("0".parse::<ChildIndex>().unwrap(), ChildIndex::Normal(0));
        assert_eq!("0h".parse::<ChildIndex>().unwrap(), ChildIndex::Hardened(0));
        assert_eq!("0H".parse::<ChildIndex>().unwrap(), ChildIndex::Hardened(0));
        assert_eq!("0'".parse::<ChildIndex>().unwrap(), ChildIndex::Hardened(0));
        assert_eq!("2147483647".parse::<ChildIndex>().unwrap(), ChildIndex::Normal(2_147_483_647));
        assert_eq!(
            "2147483647'".parse::<ChildIndex>().unwrap(),
            ChildIndex::Hardened(2_147_483_647)
        );
        assert!("2147483648".parse::<ChildIndex>().is_err());
        assert!("-1".parse::<ChildIndex>().is_err());
        assert!("-2147483648".parse::<ChildIndex>().is_err());
        assert!("522147483648".parse::<ChildIndex>().is_err());
        assert!("h".parse::<ChildIndex>().is_err());
        assert!("-h".parse::<ChildIndex>().is_err());
        assert!("0a".parse::<ChildIndex>().is_err());
        assert!("a".parse::<ChildIndex>().is_err());
    }

    #[test]
    fn path_fromstr() {
        assert_eq!("m".parse::<Path>().unwrap(), Path { path: Default::default() });
        assert_eq!("M".parse::<Path>().unwrap(), Path { path: vec![] });
        assert_eq!("m/0".parse::<Path>().unwrap(), Path { path: vec![ChildIndex::Normal(0)] });
        assert_eq!("M/44'".parse::<Path>().unwrap(), Path { path: vec![ChildIndex::Hardened(44)] });
        assert_eq!(
            "m/44'/0h/0H/0".parse::<Path>().unwrap(),
            Path {
                path: vec![
                    ChildIndex::Hardened(44),
                    ChildIndex::Hardened(0),
                    ChildIndex::Hardened(0),
                    ChildIndex::Normal(0)
                ]
            }
        );
        assert_eq!(
            "m/2147483647'/2147483647".parse::<Path>().unwrap(),
            Path {
                path: vec![ChildIndex::Hardened(2_147_483_647), ChildIndex::Normal(2_147_483_647)]
            }
        );
        assert!("".parse::<Path>().is_err());
        assert!("m/".parse::<Path>().is_err());
        assert!("m/m".parse::<Path>().is_err());
        assert!("m/2147483648".parse::<Path>().is_err());
        assert!("m/522147483648".parse::<Path>().is_err());
    }

    // macro_rules! assert_fmt {
    //     ($actual:expr, $($arg:tt)+) => {
    //         assert_eq!(format!("{:?}", $actual), format!($($arg)+));
    //     }
    // }

    // fn test_path(path_str: &str) {
    //     let seed = crate::Seed::generate_new();
    //     let master = Bip32.master(&seed, &TestNetwork);
    //     let path = path_str.parse::<Path>().unwrap();
    //     assert_fmt!(
    //         TestCrypto::bip32_ext_priv_key(&master, &path).unwrap(),
    //         "xprv(sk({}))",
    //         path_str
    //     );
    //     assert_fmt!(
    //         TestCrypto::bip32_ext_pub_key(&master, &path).unwrap(),
    //         "xpub(pk({}))",
    //         path_str
    //     );
    //     assert_fmt!(TestCrypto::bip32_priv_key(&master, &path).unwrap(), "sk({})", path_str);
    //     assert_fmt!(TestCrypto::bip32_pub_key(&master, &path).unwrap(), "pk({})", path_str);
    //     assert_fmt!(TestCrypto::bip32_key_id(&master, &path).unwrap(), "id({})", path_str);
    // }

    // #[test]
    // fn apply_path() {
    //     test_path("m");
    //     test_path("m/0'");
    //     test_path("m/44'/0'/0'/0/0");
    // }

    #[test]
    fn derivation() -> Result<()> {
        let phrase = "blast cargo razor option vote shoe stock cruel mansion boy spot never album crop reflect kangaroo blouse slam empty shoot cable vital crane manual";
        let seed = Bip39::new().phrase(phrase)?.password(Seed::PASSWORD);
        let net = &secp256k1::hyd::Mainnet;
        let master = Bip32.master(&seed, net);
        let bip44 = master.derive_hardened(44)?;
        let hyd = bip44.derive_hardened(4_741_444)?;
        let account = hyd.derive_hardened(0)?;
        let receives = account.derive_normal(0)?;
        let key = receives.derive_normal(0)?;

        assert_eq!(master.to_xprv(net), "HYDMVzMRRudwQ4sYtEeUUTuvgSjfFdA4eEEs9tnexVi7wzLh1biJ23KpwowPwpanAcppzuoUuQnDyPG41BV6Haecn1Zzy5fBnEqvqm3EfYsJHfm2");
        assert_eq!(master.neuter().to_xpub(net), "hydmVzu5j2FCm7zqZBeSU9i6QAU5wQTJCFZyU5nP2w1m7fzrFG9gn3CQhgnchDCuEF5LTsdEgqZsnATTisx8sY5bvf2xqgDbkHtw4o3qxKvjxz5X");
        assert_eq!(master.path(), &"m".parse()?);
        assert_eq!(bip44.to_xprv(net), "HYDMVzQELuRPBY1JTQZ1LQD3oJmX9X3atztB57mEaCQEWZMAstsvMp36KGHyPSDR5dfCY1Fc4DLcLk5WwSTwvmwHwioV5dRd6kvqnewQPBBCoWjD");
        assert_eq!(bip44.neuter().to_xpub(net), "hydmVzwte22eYb8b8MYyL61DX2VwqJLpT2DHPJkxedhsgF1L7ZKK7oug599C8ppMqKWqznGdxp2q28s3TQykNkJE6YFWNjuRc686X6XT9Yojtm9E");
        assert_eq!(bip44.path(), &"m/44'".parse()?);
        assert_eq!(hyd.to_xprv(net), "HYDMVzSE83q9tomV2q935JF5mRZXUB37urS6ZpAvi17d5tFNK7K45ddZmjMZJKYQ8yLjKrq4HFewhXuL5AjDzb9Ft5efNT8upEy1ftARyhmxRFZH");
        assert_eq!(hyd.neuter().to_xpub(net), "hydmVzytRASRFrtmhn914z3FV9Hx9xLMTsmCt1AenSRGFZuXYmkSqdW9XcCn3iAWcCZgEap3diAfhYPXRa5mpHuuxvWehM4j2ZS3V1rZpRF8xTr5");
        assert_eq!(hyd.path(), &"m/44'/4741444'".parse()?);
        assert_eq!(account.to_xprv(net), "HYDMVzUVgP7S8GNPrKWhvoFPivfS25QnhfKi1iydA7jbWRwVuMJTVZzBQvBV86zpNJg83rrtvj6SWsftT3nNg5PQ9kwEdzTSpEDH5KbZsjbKBbhs");
        assert_eq!(account.neuter().to_xpub(net), "hydmW129yVihVKVgXGWfvV3ZSePrhri2FgepKuyMEZ3Eg7bf91jrFZrmAo2hsVcS49dTRP5wZM7A8vudxWk5n8J2Ci12CSNvLy76CsnRaMK4A2b5");
        assert_eq!(account.path(), &"m/44'/4741444'/0'".parse()?);
        assert_eq!(receives.to_xprv(net), "HYDMVzVXwQGdZsas8KUKcmTzXtR4Le1fioCn4r8aMFa1CLvCzXJJa28ogEUPdZD1BrSTuBLDGRv5SaPHStH7ZYABhdD6Tf6dCPLd5eaJ9aArC8po");
        assert_eq!(receives.neuter().to_xpub(net), "hydmW13CEWstvvi9oGUHcTGAFc9V2RJuGpXtP38JRgseN2aNEBjhL21PS7KcNwqbz2jh787bMSPNkd7sYsdVm5rzdeyweau32iL6qzCgpn79R4o2");
        assert_eq!(receives.path(), &"m/44'/4741444'/0'/0".parse()?);
        assert_eq!(key.to_xprv(net), "HYDMVzXDVKq5jELcWwpPea9GzS7R8sPmoVo6MjhvGYhenJFBrjzgLw5LPoLzEZY1GyKRqbpPBxZNEvP6TnAq9Qpdi9wo3Bhb9NJNMC79egmr486W");
        assert_eq!(key.neuter().to_xpub(net), "hydmW14snSSM6HTuBtpMeFwSi9qqpeh1MX8CfvheLz1HwyuM6QS56vwv9gCCyx9GMfhYfMxnn89ynj3y3STp2UxpwT3kFfeGenAkzJRHsuYCLMJW");
        assert_eq!(key.path(), &"m/44'/4741444'/0'/0/0".parse()?);

        Ok(())
    }

    #[test]
    fn parsing() -> Result<()> {
        let account_xprv = "HYDMVzUVgP7S8GNPrKWhvoFPivfS25QnhfKi1iydA7jbWRwVuMJTVZzBQvBV86zpNJg83rrtvj6SWsftT3nNg5PQ9kwEdzTSpEDH5KbZsjbKBbhs";
        let account_xpub = "hydmW129yVihVKVgXGWfvV3ZSePrhri2FgepKuyMEZ3Eg7bf91jrFZrmAo2hsVcS49dTRP5wZM7A8vudxWk5n8J2Ci12CSNvLy76CsnRaMK4A2b5";
        let account_path = "m/44'/4741444'/0'";
        let net = &secp256k1::hyd::Mainnet;

        let private = Bip32Node::from_xprv(account_path.parse()?, account_xprv, net)?;
        assert_eq!(private.neuter().to_xpub(net), account_xpub);

        let public = Bip32PublicNode::from_xpub(account_path.parse()?, account_xpub, net)?;
        let receives = public.derive_normal(0)?;
        let key = receives.derive_normal(0)?;

        assert_eq!(receives.to_xpub(net), "hydmW13CEWstvvi9oGUHcTGAFc9V2RJuGpXtP38JRgseN2aNEBjhL21PS7KcNwqbz2jh787bMSPNkd7sYsdVm5rzdeyweau32iL6qzCgpn79R4o2");
        assert_eq!(receives.path(), &"m/44'/4741444'/0'/0".parse()?);
        assert_eq!(key.to_xpub(net), "hydmW14snSSM6HTuBtpMeFwSi9qqpeh1MX8CfvheLz1HwyuM6QS56vwv9gCCyx9GMfhYfMxnn89ynj3y3STp2UxpwT3kFfeGenAkzJRHsuYCLMJW");
        assert_eq!(key.path(), &"m/44'/4741444'/0'/0/0".parse()?);

        Ok(())
    }
}
