//! SLIP-0010 and BIP-0032 compatible Secp256k1 cryptography that allows child key derivation.

mod bip32;
mod bip44;
mod ext_pk;
mod ext_sk;
mod id;
mod networks;
mod pk;
mod sig;
mod sk;

use super::*;

use digest::generic_array::{typenum::U20, GenericArray};
use digest::FixedOutput;
use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};

fn hash160<B: AsRef<[u8]>>(input: B) -> GenericArray<u8, U20> {
    let mut inner_hasher = Sha256::default();
    inner_hasher.update(input);
    let mut outer_hasher = Ripemd160::default();
    outer_hasher.update(inner_hasher.finalize_fixed());
    outer_hasher.finalize_fixed()
}

const CHECKSUM_LEN: usize = 4;

/// Encoding binary data with BASE58 after adding a 4-byte checksum pops up in the Bitcoin
/// ecosystem on several places. Addresses, wallet-import-format, extended public and private
/// key serialization formats. So this transformation is pulled up here as a free function.
pub fn to_base58check<D: AsRef<[u8]>>(data: D) -> String {
    let data = data.as_ref();
    let mut inner_hasher = Sha256::default();
    inner_hasher.update(data);
    let mut outer_hasher = Sha256::default();
    outer_hasher.update(inner_hasher.finalize_fixed());
    let hash = outer_hasher.finalize_fixed();
    let checksum = &hash[..CHECKSUM_LEN];
    let mut bytes = Vec::with_capacity(data.len() + checksum.len());
    bytes.extend_from_slice(data);
    bytes.extend_from_slice(checksum);

    // we do not need the multibase prefix, but want to conform to multibase otherwise.
    // Prefix is always a single character.
    let prefixed_enc = multibase::encode(multibase::Base::Base58Btc, &bytes);
    prefixed_enc[1..].to_owned()
}

/// Decoding string with BASE58 into binary data and verify if the 4-byte checksum at the end
/// matches the rest of the data. Only the decoded data without checksum will be returned.
pub fn from_base58check<S: AsRef<str>>(s: S) -> Result<Vec<u8>> {
    let mut to_decode = String::new();
    to_decode.push(multibase::Base::Base58Btc.code());
    to_decode += s.as_ref();
    let (_base, checked_data) = multibase::decode(&to_decode)?;
    let (data, actual_checksum) = checked_data.split_at(checked_data.len() - CHECKSUM_LEN);

    let mut inner_hasher = Sha256::default();
    inner_hasher.update(data);
    let mut outer_hasher = Sha256::default();
    outer_hasher.update(inner_hasher.finalize_fixed());
    let hash = outer_hasher.finalize_fixed();
    let expected_checksum = &hash[..CHECKSUM_LEN];

    ensure!(expected_checksum == actual_checksum, "Incorrect checksum");

    Ok(data.to_vec())
}

/// This elliptic curve cryptography implements both the [AsymmetricCrypto](AsymmetricCrypto) and
/// [KeyDerivationCrypto](KeyDerivationCrypto) traits so for BTC, ETH and IOP as examples.
#[derive(Clone, Debug)]
pub struct Secp256k1;

impl Secp256k1 {
    fn hash_message<D: AsRef<[u8]>>(data: D) -> secp::Message {
        let mut hasher = Sha256::default();
        hasher.update(data.as_ref());
        let mut hash = [0u8; secp::util::MESSAGE_SIZE];
        hash.copy_from_slice(hasher.finalize_fixed().as_slice());
        secp::Message::parse(&hash)
    }
}

pub use bip32::*;
pub use bip44::*;
pub use cc::{ChainCode, CHAIN_CODE_SIZE};
pub use ext_pk::SecpExtPublicKey;
pub use ext_sk::SecpExtPrivateKey;
pub use id::{SecpKeyId, KEY_ID_SIZE, KEY_ID_VERSION1};
pub use networks::{ark, btc, hyd, iop};
pub use pk::{SecpPublicKey, PUBLIC_KEY_SIZE, PUBLIC_KEY_UNCOMPRESSED_SIZE};
pub use sig::{SecpSignature, SIGNATURE_SIZE, SIGNATURE_VERSION1};
pub use sk::{SecpPrivateKey, PRIVATE_KEY_SIZE};

impl AsymmetricCrypto for Secp256k1 {
    type KeyId = SecpKeyId;
    type PublicKey = SecpPublicKey;
    type PrivateKey = SecpPrivateKey;
    type Signature = SecpSignature;
}

impl KeyDerivationCrypto for Secp256k1 {
    type ExtendedPrivateKey = SecpExtPrivateKey;
    type ExtendedPublicKey = SecpExtPublicKey;

    fn master(seed: &Seed) -> SecpExtPrivateKey {
        SecpExtPrivateKey::from_seed(seed.as_bytes())
    }
}

/// Since Wigy could not find any constant expression for the length of `u8` in bytes
/// (`std::u8::LEN` could be a good place), this is some manual trickery to define our
/// "standard version byte length in bytes"
pub const VERSION_SIZE: usize = 1;

/// SLIP-0010 defines keyed hashing for master key derivation. This does domain separation
/// for different cryptographic algorithms. This is the standard key for BIP-0032
pub const SLIP10_SEED_HASH_SALT: &[u8] = b"Bitcoin seed";

/// [BIP-0178](https://github.com/bitcoin/bips/blob/master/bip-0178.mediawiki) is an extension
/// to the de-facto WIF to encode how the private key was used to generate receiving addresses.
/// If in doubt, just use Compressed, which is compatible with most wallets.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Bip178 {
    /// Addresses generated with uncompressed public keys were mostly used before bitcoin core 0.6
    /// and are not economical since then. Still, if you have a WIF that does not start with L or K
    /// on the BTC mainnet, [you have to use uncompressed keys](https://www.reddit.com/r/Electrum/comments/bec22p/potential_loss_of_funds_if_import_uncompressed/).
    Uncompressed,
    /// The most common format as of 2019Q2 which only promises that the wallet did not generate the receiving
    /// addresses using an uncompressed public key
    Compressed,
    /// Not that popular format as of 2019Q2, but it promises that the wallet generated only P2PKH
    /// receiving addresses with this private key
    P2PKH_Only,
    /// Not that popular format as of 2019Q2, but it promises that the wallet generated only P2WPKH
    /// (bech32 segwit native) receiving addresses with this private key
    P2WPKH,
    /// Not that popular format as of 2019Q2, but it promises that the wallet generated only P2WPKH_P2SH
    /// (segwit wrapped in a legacy script hash address) receiving addresses with this private key
    P2WPKH_P2SH,
}

impl Bip178 {
    /// Provides WIF suffix bytes for different usages of a private key
    pub fn to_wif_suffix(self) -> &'static [u8] {
        use Bip178::*;
        match self {
            Uncompressed => b"",
            Compressed => b"\x01",
            P2PKH_Only => b"\x10",
            P2WPKH => b"\x11",
            P2WPKH_P2SH => b"\x12",
        }
    }

    /// Parses usage type from WIF suffix bytes
    pub fn from_wif_suffix(data: &[u8]) -> Result<Self> {
        use Bip178::*;
        match data {
            b"" => Ok(Uncompressed),
            b"\x01" => Ok(Compressed),
            b"\x10" => Ok(P2PKH_Only),
            b"\x11" => Ok(P2WPKH),
            b"\x12" => Ok(P2WPKH_P2SH),
            _ => Err(anyhow!("Unknown wif suffix {}", hex::encode(data))),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn invalid_private_key() {
        use super::SecpPrivateKey;
        let sk_bytes =
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap();
        let err = SecpPrivateKey::from_bytes(sk_bytes).err().unwrap();
        assert!(err.to_string().contains("Invalid secret key"))
    }

    mod sign_verify {
        use crate::secp256k1::{SecpPrivateKey, SecpSignature};
        use crate::{PrivateKey, PublicKey};

        fn test(sk_hex: &str, msg: &[u8], sig_hex: &str) {
            let sk_bytes = hex::decode(sk_hex).unwrap();
            let sk = SecpPrivateKey::from_bytes(sk_bytes).unwrap();

            let sig = sk.sign(msg);
            let sig_bytes = sig.to_bytes();
            assert_eq!(hex::encode(&sig_bytes), sig_hex);

            let sig2 = SecpSignature::from_bytes(&sig_bytes).unwrap();
            let pk = sk.public_key();
            assert!(pk.verify(msg, &sig2));
        }

        #[test]
        fn test_1() {
            test(
                "0000000000000000000000000000000000000000000000000000000000000001",
                b"Satoshi Nakamoto",
                "01934b1ea10a4b3c1757e2b0c017d0b6143ce3c9a7e6a4a49860d7a6ab210ee3d82442ce9d2b916064108014783e923ec36b49743e2ffa1c4496f01a512aafd9e5",
            );
        }
    }

    // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#test-vector-2
    // https://raw.githubusercontent.com/satoshilabs/slips/master/slip-0010/testvectors.py
    // Also, you can use
    mod slip10_test_vectors {
        use crate::{
            secp256k1::{btc, Network, Secp256k1, SecpExtPrivateKey},
            ExtendedPrivateKey, KeyDerivationCrypto, Seed,
        };
        struct TestDerivation {
            xprv: SecpExtPrivateKey,
        }

        impl TestDerivation {
            fn new(seed_hex: &str) -> Self {
                let seed_bytes = hex::decode(seed_hex).unwrap();
                let seed = Seed::from_bytes(&seed_bytes).unwrap();
                let master = Secp256k1::master(&seed);
                Self { xprv: master }
            }

            fn assert_state(&self, xpub_str: &str, xprv_str: &str) {
                let xpub = self.xprv.neuter();

                assert_eq!(xpub.to_xpub(&btc::Mainnet.bip32_xpub()), xpub_str);
                assert_eq!(self.xprv.to_xprv(&btc::Mainnet.bip32_xprv()), xprv_str);
            }

            fn derive_hardened(&mut self, idx: i32) {
                let xprv = self.xprv.derive_hardened_child(idx).unwrap();
                self.xprv = xprv;
            }

            fn derive_normal(&mut self, idx: i32) {
                let xprv = self.xprv.derive_normal_child(idx).unwrap();
                self.xprv = xprv;
            }
        }

        #[test]
        fn test_vector_2() {
            let mut t = TestDerivation::new("fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542");
            t.assert_state("xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB", "xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U");
            t.derive_normal(0);
            t.assert_state("xpub69H7F5d8KSRgmmdJg2KhpAK8SR3DjMwAdkxj3ZuxV27CprR9LgpeyGmXUbC6wb7ERfvrnKZjXoUmmDznezpbZb7ap6r1D3tgFxHmwMkQTPH", "xprv9vHkqa6EV4sPZHYqZznhT2NPtPCjKuDKGY38FBWLvgaDx45zo9WQRUT3dKYnjwih2yJD9mkrocEZXo1ex8G81dwSM1fwqWpWkeS3v86pgKt");
            t.derive_hardened(2_147_483_647);
            t.assert_state("xpub6ASAVgeehLbnwdqV6UKMHVzgqAG8Gr6riv3Fxxpj8ksbH9ebxaEyBLZ85ySDhKiLDBrQSARLq1uNRts8RuJiHjaDMBU4Zn9h8LZNnBC5y4a", "xprv9wSp6B7kry3Vj9m1zSnLvN3xH8RdsPP1Mh7fAaR7aRLcQMKTR2vidYEeEg2mUCTAwCd6vnxVrcjfy2kRgVsFawNzmjuHc2YmYRmagcEPdU9");
            t.derive_normal(1);
            t.assert_state("xpub6DF8uhdarytz3FWdA8TvFSvvAh8dP3283MY7p2V4SeE2wyWmG5mg5EwVvmdMVCQcoNJxGoWaU9DCWh89LojfZ537wTfunKau47EL2dhHKon", "xprv9zFnWC6h2cLgpmSA46vutJzBcfJ8yaJGg8cX1e5StJh45BBciYTRXSd25UEPVuesF9yog62tGAQtHjXajPPdbRCHuWS6T8XA2ECKADdw4Ef");
            t.derive_hardened(2_147_483_646);
            t.assert_state("xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL", "xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc");
            t.derive_normal(2);
            t.assert_state("xpub6FnCn6nSzZAw5Tw7cgR9bi15UV96gLZhjDstkXXxvCLsUXBGXPdSnLFbdpq8p9HmGsApME5hQTZ3emM2rnY5agb9rXpVGyy3bdW6EEgAtqt", "xprvA2nrNbFZABcdryreWet9Ea4LvTJcGsqrMzxHx98MMrotbir7yrKCEXw7nadnHM8Dq38EGfSh6dqA9QWTyefMLEcBYJUuekgW4BYPJcr9E7j");
        }

        #[test]
        fn test_vector_3() {
            let mut t = TestDerivation::new("4b381541583be4423346c643850da4b320e46a87ae3d2a4e6da11eba819cd4acba45d239319ac14f863b8d5ab5a0d0c64d2e8a1e7d1457df2e5a3c51c73235be");
            t.assert_state("xpub661MyMwAqRbcEZVB4dScxMAdx6d4nFc9nvyvH3v4gJL378CSRZiYmhRoP7mBy6gSPSCYk6SzXPTf3ND1cZAceL7SfJ1Z3GC8vBgp2epUt13", "xprv9s21ZrQH143K25QhxbucbDDuQ4naNntJRi4KUfWT7xo4EKsHt2QJDu7KXp1A3u7Bi1j8ph3EGsZ9Xvz9dGuVrtHHs7pXeTzjuxBrCmmhgC6");
            t.derive_hardened(0);
            t.assert_state("xpub68NZiKmJWnxxS6aaHmn81bvJeTESw724CRDs6HbuccFQN9Ku14VQrADWgqbhhTHBaohPX4CjNLf9fq9MYo6oDaPPLPxSb7gwQN3ih19Zm4Y", "xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L");
        }
    }

    // https://gobittest.appspot.com/Address is pre bitcoin-core 0.6 and uses the uncompressed public key, so
    // avoid using it now.
    // In https://gobittest.appspot.com/PrivateKey just ignore the 01 suffix added to WIFs only used
    // for post-0.6 compressed public keys.
    // You can check that these tests are valid using the following
    // ./bitcointool -c pubfrompriv -p <WIF>
    // ./bitcointool -c addrfrompub -k <pub>
    mod btc_key_conversions {
        use crate::secp256k1::{btc, Bip178, Network, SecpPrivateKey};
        use crate::{PrivateKey, PublicKey};

        fn test(sk_hex: &str, wif: &str, pk_hex: &str, id_hex: &str, address: &str) {
            let sk_bytes = hex::decode(sk_hex).unwrap();
            let sk = SecpPrivateKey::from_bytes(sk_bytes).unwrap();

            let sk_wif = sk.to_wif(&btc::Mainnet.wif(), Bip178::Compressed);
            assert_eq!(sk_wif, wif);

            let pk = sk.public_key();
            let pk_bytes = pk.to_bytes();
            assert_eq!(hex::encode(&pk_bytes), pk_hex);

            let id = pk.key_id();
            let id_bytes = id.to_bytes();
            assert_eq!(hex::encode(&id_bytes), id_hex);

            let act_address = id.to_p2pkh_addr(&btc::Mainnet.p2pkh_addr());
            assert_eq!(act_address, address);
        }

        #[test]
        fn test_1() {
            test(
                "0000000000000000000000000000000000000000000000000000000000000001",
                "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73sVHnoWn",
                "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                "01751e76e8199196d454941c45d1b3a323f1433bd6",
                "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH",
            );
        }
        #[test]
        fn test_2() {
            test(
                "0000000000000000000000000000000000000000000000000000000000000002",
                "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU74NMTptX4",
                "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
                "0106afd46bcdfd22ef94ac122aa11f241244a37ecc",
                "1cMh228HTCiwS8ZsaakH8A8wze1JR5ZsP",
            );
        }
        #[test]
        fn test_3() {
            test(
                "0000000000000000000000000000000000000000000000000000000000000003",
                "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU74sHUHy8S",
                "02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9",
                "017dd65592d0ab2fe0d0257d571abf032cd9db93dc",
                "1CUNEBjYrCn2y1SdiUMohaKUi4wpP326Lb",
            );
        }
        #[test]
        fn test_4() {
            test(
                "aa5e28d6a97a2479a65527f7290311a3624d4cc0fa1578598ee3c2613bf99522",
                "L2vtCpubwLeqLNYywTUqLLmN6LiijyYWUArxvyw5DyFD8TaxqJyu",
                "0234f9460f0e4f08393d192b3c5133a6ba099aa0ad9fd54ebccfacdfa239ff49c6",
                "01db820065e5bd79e976f0dc09f2257e35243879cf",
                "1M1eigHFbhtWLnc37qXQt1ao2taLhE49yg",
            );
        }
        #[test]
        fn test_5() {
            test(
                "7e2b897b8cebc6361663ad410835639826d590f393d90a9538881735256dfae3",
                "L1Sy9ysFzZDXh5gXYrgJmbyhnhbJVyptuTypUnD9ofZoV3V2SpUi",
                "03d74bf844b0862475103d96a611cf2d898447e288d34b360bc885cb8ce7c00575",
                "015716c6c9146a548ce31092f72ab24b44d8580914",
                "18wV5EG3Hqocod1RLm9STvbUnSqb1NMo44",
            );
        }
        #[test]
        fn test_6() {
            test(
                "6461e6df0fe7dfd05329f41bf771b86578143d4dd1f7866fb4ca7e97c5fa945d",
                "Kzaqk53898thvqucDWi4MqC3ogC2s2QmtZL31qjS9MRMvgHFKpDZ",
                "03e8aecc370aedd953483719a116711963ce201ac3eb21d3f3257bb48668c6a72f",
                "01e3281990058f008a4b6c658cb735ae2b7327daa5",
                "1Mi6RjU7ASvudQMZkeobQ1WoiZWAtVhkd6",
            );
        }
    }

    mod blockchain_com_derivation {
        use crate::secp256k1::*;

        #[test]
        fn btc_wallet() {
            let phrase =
                "hint replace increase neglect egg wood ill alert beef rich install potato";
            let xpub = "xpub6DVu7eWDWJkczyrDQdo2i99MKdns8idTeVfzwuHCaA3rKfSWaSCnpKigU21vJ2TdCT5MiLgbKWzpxZUx4gFx6zpNWukDzX8yRGU3UKyE9fC";
            let xpub2 = "xpub6DVu7eWDWJkd4MbGacHiWGFKT2amHnJwQ7SmxWpD2YuacVtaQz7XPsRYxR5PqsGa2TiLYWPi3jsimnruvJ5LvMBxjc96jgZniP7peLHcEdM";
            let old_addr0 = "17kxMsME7f8CVVWqPadgQNsDEMaHDBpCbv";
            let bip44_addr0 = "17Y82siUGdY8KEWGpUTW7uF5kj6cEMRYfo";
            let net = &btc::Mainnet;

            let seed = Bip39::new().short_phrase(phrase).unwrap().password("");
            let coin = Bip44.network(&seed, net).unwrap();
            let account: Bip44Account<Secp256k1> = coin.account(0).unwrap();

            assert_eq!(account.neuter().to_xpub(), xpub);

            let old_pk0 = account.node().derive_normal(0).unwrap().neuter();

            assert_eq!(old_pk0.to_p2pkh_addr(net), old_addr0);

            let pk0 = account.key(0).unwrap().neuter();

            assert_eq!(pk0.to_p2pkh_addr(), bip44_addr0);

            let account2 = coin.account(1).unwrap();

            assert_eq!(account2.neuter().to_xpub(), xpub2);
        }
    }

    mod coinomi_derivation {
        use crate::{
            secp256k1::{btc, hyd},
            *,
        };

        #[test]
        fn hyd_derive() {
            let mnemonic = "blast cargo razor option vote shoe stock cruel mansion boy spot never album crop reflect kangaroo blouse slam empty shoot cable vital crane manual";
            let pk0_hex = "02f946d10106f55c755c1f836b63bef35fb0015603e1870c8dbdcacf62f178587e";
            let addr0 = "hWNN8ymcsLdJivbwbBaPS8X1vekxB2pdwV";

            let seed = Bip39::new().phrase(mnemonic).unwrap().password(Seed::PASSWORD);
            let account = Bip44.network(&seed, &hyd::Mainnet).unwrap().account(0).unwrap();
            let key0 = account.key(0).unwrap().neuter();

            assert_eq!(hex::encode(key0.to_public_key().to_bytes()), pk0_hex);
            assert_eq!(key0.to_p2pkh_addr(), addr0);
        }

        #[test]
        fn tbtc_derive() {
            let mnemonic = "blast cargo razor option vote shoe stock cruel mansion boy spot never album crop reflect kangaroo blouse slam empty shoot cable vital crane manual";
            let xpub = "tpubDDfA4LQYyG71KmPW65gktxjvzxFAFcdhDAzj4zc6y5hpeX3rZu3nPh1GuvgWCyj4VWKfuFbnnCvFXyTuDLD6mmFA5yVTe2UUcSoNy7kgcYm";
            let xpub_coinomi_bug = "xpub6DHXY6asFz9Z3yCedthfh3QZfq9WGE8dfYQRiSYxdAwvuEP9VgChFFLSftbigUmphbnmHg5vF76CqnyHpUMrtKns2Nk9xVpF2VCyD7Uej6C";
            let addr0 = "mgH9VjC6uGTt1cWDDZEXisASAtCE8D6Xar";
            let net = &btc::Testnet;

            let seed = Bip39::new().phrase(mnemonic).unwrap().password(Seed::PASSWORD);
            let account = Bip44.network(&seed, net).unwrap().account(0).unwrap().neuter();

            assert_eq!(account.to_xpub(), xpub);
            assert_eq!(account.node().to_xpub(&btc::Mainnet), xpub_coinomi_bug);

            let key0 = account.key(0).unwrap();

            assert_eq!(key0.to_p2pkh_addr(), addr0);
        }
    }

    mod ark_desktop_hyd_address {
        use super::super::*;

        #[test]
        fn test() {
            let phrase = "boss slice draft close detail mix nation casino judge cigar melody catch";
            let pk_hex = "03fdd041ed3e51d8909c44ef6b9d1268a412161d8e6544c5cd4d87ef78bb49e2f7";
            let addr = "hFxvDfqQfXHzhvEebTJGkds8DBBGBCKpY9";
            let addr_dev = "dEatNarXZifEXaqnMFy5uP9Fv8bq7aB3Vn";
            let addr_test = "tXRoniBUYaGXc495HCdCL9V9qJPgBMywaH";

            let sk = SecpPrivateKey::from_ark_passphrase(phrase).unwrap();
            let pk = sk.public_key();

            assert_eq!(hex::encode(pk.to_bytes()), pk_hex);

            let key_id = pk.ark_key_id();

            assert_eq!(key_id.to_p2pkh_addr(&hyd::Mainnet.p2pkh_addr()), addr);
            assert_eq!(key_id.to_p2pkh_addr(&hyd::Devnet.p2pkh_addr()), addr_dev);
            assert_eq!(key_id.to_p2pkh_addr(&hyd::Testnet.p2pkh_addr()), addr_test);
        }
    }

    // ARK is special 😉 These tests are cross-checked with their JavaScript crpyto implementation and use private keys
    // of their testnet genesis delegates.
    mod ark_key_conversions {
        use super::super::*;

        fn test(passphrase: &str, pk_hex: &str, main_addr: &str, dev_addr: &str) {
            let sk = SecpPrivateKey::from_ark_passphrase(passphrase).unwrap();
            let pk = sk.public_key();
            assert_eq!(hex::encode(pk.to_bytes()), pk_hex);

            let key_id = pk.ark_key_id();

            assert_eq!(key_id.to_p2pkh_addr(&ark::Mainnet.p2pkh_addr()), main_addr);
            assert_eq!(key_id.to_p2pkh_addr(&ark::Devnet.p2pkh_addr()), dev_addr);
        }

        #[test]
        fn test_delegate_1() {
            test(
                "clay harbor enemy utility margin pretty hub comic piece aerobic umbrella acquire",
                "03287bfebba4c7881a0509717e71b34b63f31e40021c321f89ae04f84be6d6ac37",
                "ANBkoGqWeTSiaEVgVzSKZd3jS7UWzv9PSo",
                "DBYyh2vXcigrJGUHfvmYxVxEqeH7vomw6x",
            );
        }

        #[test]
        fn test_delegate_2() {
            test(
                "venue below waste gather spin cruise title still boost mother flash tuna",
                "02def27da9336e7fbf63131b8d7e5c9f45b296235db035f1f4242c507398f0f21d",
                "AbfQq8iRSf9TFQRzQWo33dHYU7HFMS17Zd",
                "DR2ditoSQvPaySQbaT8GSWC3se5rLyfq4T",
            );
        }
    }
}
