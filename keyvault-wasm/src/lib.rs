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

use wasm_bindgen::prelude::*;

// imports from own crates

use iop_keyvault::{
    ed25519::*, encrypt as keyvault_encrypt, multicipher::*, secp256k1::*, Bip32, Bip32Node,
    Bip32PublicNode, Bip39, Bip39Phrase, Bip44, Bip44Account, Bip44Coin, Bip44Key,
    Bip44PublicAccount, Bip44PublicKey, Bip44PublicSubAccount, Bip44SubAccount, Chain, Networks,
    PrivateKey as _, PublicKey as _, Seed,
};

// code

pub fn err_to_js<E: ToString>(e: E) -> JsValue {
    JsValue::from(e.to_string())
}

pub trait MapJsError<T> {
    fn map_err_to_js(self) -> Result<T, JsValue>;
}

impl<T, E: ToString> MapJsError<T> for Result<T, E> {
    fn map_err_to_js(self) -> Result<T, JsValue> {
        self.map_err(err_to_js)
    }
}

pub trait Wraps<T>: From<T> {
    fn inner(&self) -> &T;
}

pub trait WrapsMut<T>: Wraps<T> {
    fn inner_mut(&mut self) -> &mut T;
}

#[wasm_bindgen(js_name = validateNetworkName)]
pub fn validate_network_name(name: &str) -> bool {
    Networks::by_name(name).is_ok()
}
