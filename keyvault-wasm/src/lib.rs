#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! # Keyvault
//!
//! Keyvault is a general purpose hierarchical deterministic (HD) generator for asymmetric keys.
//! It is based on the same concepts as a Bitcoin HD-wallet and is built on the same specifications like
//! [HD wallets of Bip32](https://en.bitcoin.it/wiki/BIP_0032),
//! [Mnemonic word lists of Bip39](https://en.bitcoin.it/wiki/BIP_0039) and
//! [Purpose fields of Bip43](https://en.bitcoin.it/wiki/BIP_0043).
//!
//! Though keyvault is capable of generating wallet addresses as defined in
//! [Multi-Account cryptocurrency wallets of Bip44](https://en.bitcoin.it/wiki/BIP_0044),
//! it is not only an address generator for multiple cryptocurrencies.
//! Keyvault can also derive all the keys you might need in other software stacks
//! and aims to be your all-in-one Swiss Army knife identity manager.
//!
//! Keyvault can
//!
//! - use the same seed to derive keys with multiple cipher suites, currently `ed25519` and `secp256k1`
//! - use any purpose field and account hierarchy, not only Bip43 and Bip44
//! - handle several purposes (i.e. attached subhierarchies) at the same time
//! - be used from other platforms via its C and WebAssembly bindings
//!
//! Keyvault was originally created as part of the
//! [Mercury communication protocol](https://github.com/Internet-of-People/mercury-rust)
//! but being a general-purpose tool it was reused in other components as well,
//! hence was separated into [its own repository](https://github.com/Internet-of-People/keyvault-rust) then finally merged into this monorepository.
//!
//! **This documentation is optimized for reading after it is copied into JSDoc**
//!
//! For more information on this crate and potential usage, see the [IOP developer site].
//!
//! [IOP developer site]: https://developer.iop.technology/glossary?id=multicipher

// NOTE Always receive function arguments as references (as long as bindgen allows)
//      and return results by value. Otherwise the generated code may destroy
//      JS variables by moving out underlying pointers.

// sub-modules

mod bip32;
mod bip39;
mod bip44;
mod encrypt;
mod id;
mod morpheus;
mod pk;
mod seed;
mod sig;
mod sk;

// exports

pub use bip32::*;
pub use bip39::*;
pub use bip44::*;
pub use encrypt::*;
pub use id::*;
pub use morpheus::*;
pub use pk::*;
pub use seed::*;
pub use sig::*;
pub use sk::*;

// imports from standard library

// imports from 3rd party crates

use serde::Serialize;
use wasm_bindgen::prelude::*;

// imports from own crates

use iop_keyvault::{
    ed25519::*, encrypt as keyvault_encrypt, multicipher::*, secp256k1::*, Bip32, Bip32Node,
    Bip32PublicNode, Bip39, Bip39Phrase, Bip44, Bip44Account, Bip44Coin, Bip44Key,
    Bip44PublicAccount, Bip44PublicKey, Bip44PublicSubAccount, Bip44SubAccount, Chain, Networks,
    PrivateKey as _, PublicKey as _, Seed,
};

// code

/// Converts any error that can be converted into a string into a JavaScript error string
/// usable by wasm_bindgen
pub fn err_to_js<E: ToString>(e: E) -> JsValue {
    JsValue::from(e.to_string())
}

/// An extension trait on [`Result`] that helps easy conversion of Rust errors to JavaScript
/// error strings usable by wasm_bindgen
pub trait MapJsError<T> {
    /// An extension method on [`Result`] to easily convert Rust errors to JavaScript ones.
    ///
    /// ```ignore
    /// #[wasm_bindgen]
    /// pub fn method(&self) -> Result<JsSomething, JsValue> {
    ///     let result: JsSomething = self.fallible().map_err_to_js()?;
    ///     Ok(result)
    /// }
    /// ```
    fn map_err_to_js(self) -> Result<T, JsValue>;
}

impl<T, E: ToString> MapJsError<T> for Result<T, E> {
    fn map_err_to_js(self) -> Result<T, JsValue> {
        self.map_err(err_to_js)
    }
}

/// Most WASM types are wrapping a Rust type one-on-one. This trait helps to enforce a convention
/// so that WASM types can easily peek under the hood of other such wrapped WASM types.
///
/// See also [`WrapsMut<T>`]
pub trait Wraps<T>: From<T> {
    /// Converts a reference to a WASM type to a reference to the underlying Rust type.
    fn inner(&self) -> &T;
}

/// Most WASM types are wrapping a Rust type one-on-one. This trait helps to enforce a convention
/// so that WASM types can easily peek under the hood of other such wrapped WASM types.
///
/// See also [`Wraps<T>`]
pub trait WrapsMut<T>: Wraps<T> {
    /// Converts an exclusive reference to a WASM type to an exclusive reference to the underlying Rust type.
    fn inner_mut(&mut self) -> &mut T;
}

/// Free function that checks if a string is a valid network name usable as a parameter in some other calls.
///
/// @see allNetworkNames
#[wasm_bindgen(js_name = validateNetworkName)]
pub fn validate_network_name(name: &str) -> bool {
    Networks::by_name(name).is_ok()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type IStringArray;
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
struct NetworkNames(Vec<&'static str>);

/// The list of all network names accepted by {@link validateNetworkName}
#[wasm_bindgen(js_name = allNetworkNames)]
pub fn all_network_names() -> IStringArray {
    let names: Vec<&'static str> = Networks::ALL.iter().map(|n| n.name()).collect();
    let array = JsValue::from_serde(&NetworkNames(names))
        .expect("No object keyed maps in the object graph; qed");
    array.into()
}
