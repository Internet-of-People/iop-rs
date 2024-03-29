# Morpheus

Morpheus is a framework for decentralized identifiers (DID), key and right management and
cryptographically verifiable claims and credentials (VC) about such identities
based on a web of trust. For an in-depth description, please check our [developer portal](https://developer.iop.technology/).

Morpheus DIDs were based on the [W3C DID specification](https://w3c.github.io/did-core/) and
VCs on the [W3C VC specification](https://www.w3.org/TR/vc-data-model/).
Morpheus key management is basically a decentralized version of a Public Key Infrastructure (PKI).
Adding Morpheus right management on top is close to a decentralized version of Active Directory.
Morpheus VCs provide a privacy-by-design system and enable self sovereignty over your user data.

## Development

Having a Rust environment installed (e.g. by using [rustup](https://rustup.rs/)),
you can simply use `cargo` to build and test everything, e.g. `cargo build`.

## Key concepts

Morpheus can generate safe identities for one-time use without requiring any network connectivity.
Any key or right management and further optional features requiring public verification like
timestamping of content ids and signatures need a public ledger.

The current implementation uses the [Hydra blockchain](https://github.com/Internet-of-People/hydra-core),
but is ledger-agnostic in general. Hydra was built as a bridge-chain of the Ark ecosystem and Morpheus adds a
[layer 2 plugin](https://github.com/Internet-of-People/iop-ts) on top to define custom transactions
managing DIDs and rights.

Anything else that does not need public verification is not stored on the ledger or
anywhere publicly, with special emphasis on any user data, e.g. verifiable claims.
Instead, user data is shared explicitly on demand with only a specific peer.
Shared data contains a license describing who might use it for what purpose and when the license expires.

Before sharing user data, the user can mask out any details not meant for sharing
without losing cryptographic verifiability of witness signatures.
E.g. you have a digital ID signed by some authority.
Ordering pizza, you can give your name and address without exposing any other details
that are irrelevant for delivering your meal like your birthday and mother's name. 
Despite those details were masked out, the restaurant can still verify that data was witnessed by the authority.
This is achieved by signing a Merkle-proof instead of the data directly.
We use [JSON digests](https://json-digest.rocks/) for our proofs for easy implementation and portability.

## History

Morpheus was built on the KeyVault and elaborates some concepts originally conceived under
[Mercury](https://github.com/Internet-of-People/mercury-rust)
that turned out to deserve their own project.

## Reporting Vulnerabilities

Please contact the package authors.

## Usage

See the [IOP developer page](https://developer.iop.technology).

## Contributing

Please contact the package authors.
