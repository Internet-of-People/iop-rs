// TODO provide a C API that allows
// 1. selecting a DID from a vault
// 2. selecting a key for a DID
// 3. sign content with the selected key
// +1 maybe later: create a witness request

use std::cell::RefCell;
use std::ffi;
use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use failure::{format_err, Fallible};

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

fn result_to_c<R>(
    id: *mut RequestId, res: Fallible<R>, success: Callback<R>, error: Callback<*const raw::c_char>,
) {
    match res {
        Ok(val) => success(id, val),
        Err(err) => error(id, string_out(err.to_string())),
    }
}

fn result_transformed_to_c<R, S, F: Fn(R) -> S>(
    id: *mut RequestId, res: Fallible<R>, success: Callback<S>,
    error: Callback<*const raw::c_char>, transform: F,
) {
    match res {
        Ok(v) => success(id, transform(v)),
        Err(e) => error(id, string_out(e.to_string())),
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
) {
    let result = imp::SdkContext::new();
    result_transformed_to_c(id, result, success, error, |ctx| Box::into_raw(Box::new(ctx)))
}

#[no_mangle]
pub extern "C" fn create_vault(
    ctx: *mut imp::SdkContext, seed: *const raw::c_char, path: *const raw::c_char,
    id: *mut RequestId, success: Callback<()>, error: Callback<*const raw::c_char>,
) {
    let ctx = unsafe { &mut *ctx };
    let result = block_on(async { ctx.create_vault(str_in(seed)?, str_in(path)?).await });
    result_to_c(id, result, success, error)
}

#[no_mangle]
pub extern "C" fn load_vault(
    ctx: *mut imp::SdkContext, path: *const raw::c_char, id: *mut RequestId, success: Callback<()>,
    error: Callback<*const raw::c_char>,
) {
    let ctx = unsafe { &mut *ctx };
    let result = block_on(async { ctx.load_vault(str_in(path)?).await });
    result_to_c(id, result, success, error)
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
        io::dist::did::hydra::HydraDidLedger,
        io::local::didvault::{InMemoryDidVault, PersistentDidVault},
        sdk::Client,
    };

    pub type StandardClient = Client<PersistentDidVault, HydraDidLedger>;

    pub struct SdkContext {
        pub client: Option<StandardClient>,
    }

    impl SdkContext {
        pub fn new() -> Fallible<Self> {
            Ok(Self { client: Default::default() })
        }

        pub async fn create_vault(&mut self, seed: &str, path: &str) -> Fallible<()> {
            let seed = keyvault::Seed::from_bip39(seed)?;
            let mem_vault = InMemoryDidVault::new(seed);
            let mut persistent_vault = PersistentDidVault::new(mem_vault, path);
            persistent_vault.save().await?;
            self.set_client(persistent_vault);
            Ok(())
        }

        pub async fn load_vault(&mut self, path: &str) -> Fallible<()> {
            let persistent_vault = PersistentDidVault::load(path).await?;
            self.set_client(persistent_vault);
            Ok(())
        }

        fn set_client(&mut self, vault: PersistentDidVault) -> () {
            let ledger = HydraDidLedger::new();
            self.client.replace(StandardClient::new(vault, ledger));
        }
    }
}
