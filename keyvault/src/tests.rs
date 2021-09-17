use super::Seed;

const fn as_bytes(bits: usize) -> usize {
    bits / 8
}

#[test]
fn seed_from_bytes_accepts_512_bits() {
    let bytes = [0u8; as_bytes(Seed::BITS)]; // 512 bits
    let seed_res = Seed::from_bytes(&bytes);
    assert!(seed_res.is_ok());
}

#[test]
fn seed_from_bytes_rejects_not_512_bits() {
    let bytes = [0u8; 32]; // 256 bits
    let seed_res = Seed::from_bytes(&bytes);
    assert!(seed_res.unwrap_err().to_string().contains("-bit seed"));
}

// These 2 conversions were done with the multiformats@9.4.7 npm package, so these tests confirm compatibility with the Rust version

fn test_multibase(hex_binary: &str, expected: &str) {
    let binary = hex::decode(hex_binary).unwrap();
    let actual = multibase::encode(multibase::Base::Base64Url, &binary);
    assert_eq!(expected, actual);
    let round_trip = multibase::decode(expected).unwrap().1;
    assert_eq!(&binary, &round_trip);
}

#[test]
fn multibase_u_test1() {
    test_multibase(
        "d878785db3f5de155148d339c8dcca741f357f50b2558ab6323495132dcefa5e9cd77d1447e049c12e0cb589b9101b68466c3c9d82a1f1e51bb5192403d67235",
        "u2Hh4XbP13hVRSNM5yNzKdB81f1CyVYq2MjSVEy3O-l6c130UR-BJwS4MtYm5EBtoRmw8nYKh8eUbtRkkA9ZyNQ",
    );
}

#[test]
fn multibase_u_test2() {
    test_multibase(
        "01d878785db3f5de155148d339c8dcca741f357f50b2558ab6323495132dcefa5e9cd77d1447e049c12e0cb589b9101b68466c3c9d82a1f1e51bb5192403d67235",
        "uAdh4eF2z9d4VUUjTOcjcynQfNX9QslWKtjI0lRMtzvpenNd9FEfgScEuDLWJuRAbaEZsPJ2CofHlG7UZJAPWcjU",
    );
}
