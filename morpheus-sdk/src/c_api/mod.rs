// TODO provide a C API that allows
// 1. selecting a DID from a vault
// 2. selecting a key for a DID
// 3. sign content with the selected key
// +1 maybe later: create a witness request

use std::cell::RefCell;
use std::ffi;
use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use failure::Fallible;

fn str_in<'a>(s: *const raw::c_char) -> Fallible<&'a str> {
    let c_str = unsafe { ffi::CStr::from_ptr(s) };
    let s = c_str.to_str()?;
    Ok(s)
}

fn string_out(s: String) -> *mut raw::c_char {
    let c_str = ffi::CString::new(s).unwrap();
    c_str.into_raw()
}

#[repr(C)]
pub struct RequestId {
    _private_internal_layout: [u8; 0],
}
type Callback<T> = extern "C" fn(*mut RequestId, T) -> ();

struct CallContext<T> {
    id: *mut RequestId,
    success: Callback<T>,
    error: Callback<*const raw::c_char>,
}

impl<T> CallContext<T> {
    pub fn new(
        id: *mut RequestId, success: Callback<T>, error: Callback<*const raw::c_char>,
    ) -> Self {
        Self { id, success, error }
    }

    pub fn run(self, f: impl FnOnce() -> Fallible<T>) {
        match f() {
            Ok(val) => (self.success)(self.id, val),
            Err(err) => (self.error)(self.id, string_out(err.to_string())),
        }
    }

    pub fn run_async(self, f: impl std::future::Future<Output = Fallible<T>>) {
        self.run(|| block_on(f))
    }
}

thread_local! {
    static REACTOR: RefCell<tokio::runtime::Runtime> = RefCell::new(
        tokio::runtime::Builder::new().enable_all().basic_scheduler().build()
            .expect("Failed to initialize Tokio runtime")
     );
}

fn block_on<R>(fut: impl std::future::Future<Output = R>) -> R {
    REACTOR.with(|reactor| reactor.borrow_mut().block_on(fut))
}

#[no_mangle]
pub extern "C" fn init_sdk(
    id: *mut RequestId, success: Callback<*mut imp::SdkContext>,
    error: Callback<*const raw::c_char>,
) -> () {
    let fun = || {
        let sdk = imp::SdkContext::default();
        Fallible::Ok(Box::into_raw(Box::new(sdk)))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn create_vault(
    sdk: *mut imp::SdkContext, seed: *const raw::c_char, path: *const raw::c_char,
    id: *mut RequestId, success: Callback<()>, error: Callback<*const raw::c_char>,
) -> () {
    let sdk = unsafe { &mut *sdk };
    let fut = async { sdk.create_vault(str_in(seed)?, str_in(path)?).await };
    CallContext::new(id, success, error).run_async(fut)
}

#[no_mangle]
pub extern "C" fn load_vault(
    sdk: *mut imp::SdkContext, path: *const raw::c_char, id: *mut RequestId, success: Callback<()>,
    error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fut = async { sdk.load_vault(str_in(path)?).await };
    CallContext::new(id, success, error).run_async(fut)
}

#[no_mangle]
pub extern "C" fn list_dids(
    sdk: *mut imp::SdkContext, id: *mut RequestId, success: Callback<()>,
    error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fut = async {
        let vec = sdk.list_dids().await;
        // TODO we should return an array of strings somehow
        vec.map(|dids| ())
    };
    CallContext::new(id, success, error).run_async(fut)
}

#[no_mangle]
pub extern "C" fn close_sdk(sdk: *mut imp::SdkContext) {
    if !sdk.is_null() {
        unsafe {
            Box::from_raw(sdk);
        }
    }
}

mod imp {
    use failure::Fallible;

    use crate::{
        data::did::Did,
        io::dist::did::{hydra::HydraDidLedger, LedgerOperations, LedgerQueries},
        io::local::didvault::{DidVault, InMemoryDidVault, PersistentDidVault},
        sdk::Client,
    };

    pub type SdkContext = Sdk<PersistentDidVault, HydraDidLedger>;

    pub struct Sdk<V: DidVault, L: LedgerQueries + LedgerOperations> {
        pub client: Client<V, L>,
    }

    impl<V: DidVault, L: LedgerQueries + LedgerOperations> Default for Sdk<V, L> {
        fn default() -> Self {
            Self { client: Default::default() }
        }
    }

    impl SdkContext {
        pub async fn create_vault(&mut self, seed: &str, path: &str) -> Fallible<()> {
            let seed = keyvault::Seed::from_bip39(seed)?;
            let mem_vault = InMemoryDidVault::new(seed);
            let mut persistent_vault = PersistentDidVault::new(mem_vault, path);
            persistent_vault.save().await?;
            self.client.set_vault(persistent_vault)
        }

        pub async fn load_vault(&mut self, path: &str) -> Fallible<()> {
            let persistent_vault = PersistentDidVault::load(path).await?;
            self.client.set_vault(persistent_vault)
        }

        pub async fn list_dids(&self) -> Fallible<Vec<Did>> {
            self.client.vault()?.dids()
        }
    }
}
