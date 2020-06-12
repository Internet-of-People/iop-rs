use wasm_bindgen::prelude::*;

// NOTE Always receive function arguments as references (as long as bindgen allows)
//      and return results by value. Otherwise the generated code may destroy
//      JS variables by moving out underlying pointers.

pub fn err_to_js<E: ToString>(e: E) -> JsValue {
    JsValue::from(e.to_string())
}

pub trait Wraps<T>: From<T> {
    fn inner(&self) -> &T;
}

#[wasm_bindgen(js_name = validateNetworkName)]
pub fn validate_network_name(name: &str) -> bool {
    Networks::by_name(name).is_ok()
}

mod bip32;
mod bip39;
mod bip44;
mod encrypt;
mod id;
mod morpheus;
mod networks;
mod pk;
mod seed;
mod sig;
mod sk;

pub use bip32::{JsBip32, JsBip32Node, JsBip32PublicNode};
pub use bip39::{JsBip39, JsBip39Phrase};
pub use bip44::{
    JsBip44, JsBip44Account, JsBip44Coin, JsBip44Key, JsBip44PublicAccount, JsBip44PublicKey,
    JsBip44PublicSubAccount, JsBip44SubAccount,
};
pub use encrypt::{decrypt, encrypt};
pub use id::{JsMKeyId, JsSecpKeyId};
pub use morpheus::{JsMorpheus, JsMorpheusRoot};
pub use networks::Networks;
pub use pk::{JsMPublicKey, JsSecpPublicKey};
pub use seed::JsSeed;
pub use sig::{JsMSignature, JsSecpSignature};
pub use sk::{JsMPrivateKey, JsSecpPrivateKey};
