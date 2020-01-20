// TODO provide a C API that allows
// 1. selecting a DID from a vault
// 2. selecting a key for a DID
// 3. sign content with the selected key
// +1 maybe later: create a witness request

use std::ffi;
use std::os::raw;
use std::panic::catch_unwind;

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

type Callback<T> = extern "C" fn(T) -> ();

#[no_mangle]
pub extern "C" fn init_sdk(
    callback: Callback<*mut imp::SdkContext>, error: Callback<*const raw::c_char>,
) {
    match imp::SdkContext::new() {
        Ok(ctx) => callback(Box::into_raw(Box::new(ctx))),
        Err(e) => error(string_out(e.to_string())),
    }
}

#[no_mangle]
pub extern "C" fn create_vault(
    ctx: *mut imp::SdkContext, seed: *const raw::c_char, path: *const raw::c_char,
    callback: Callback<()>, error: Callback<*const raw::c_char>,
) {
    let ctx = unsafe { &mut *ctx };
    let (runtime, client) = (&mut ctx.runtime, &mut ctx.client);
    let fallible_async =
        async { imp::SdkContext::create_vault(client, str_in(seed)?, str_in(path)?).await };
    match runtime.block_on(fallible_async) {
        Ok(()) => callback(()),
        Err(e) => error(string_out(e.to_string())),
    }
}

#[no_mangle]
pub extern "C" fn load_vault(
    ctx: *mut imp::SdkContext, path: *const raw::c_char, callback: Callback<()>,
    error: Callback<*const raw::c_char>,
) {
    let mut ctx = unsafe { &mut *ctx };
    let (runtime, client) = (&mut ctx.runtime, &mut ctx.client);
    let fallible_async = async { imp::SdkContext::load_vault(client, str_in(path)?).await };
    match runtime.block_on(fallible_async) {
        Ok(()) => callback(()),
        Err(e) => error(string_out(e.to_string())),
    }
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
        pub runtime: tokio::runtime::Runtime,
    }

    impl SdkContext {
        pub fn new() -> Fallible<Self> {
            let runtime = tokio::runtime::Builder::new().basic_scheduler().enable_all().build()?;
            Ok(Self { client: Default::default(), runtime })
        }

        pub async fn create_vault(
            client: &mut Option<StandardClient>, seed: &str, path: &str,
        ) -> Fallible<()> {
            let seed = keyvault::Seed::from_bip39(seed)?;
            let mem_vault = InMemoryDidVault::new(seed);
            let mut persistent_vault = PersistentDidVault::new(mem_vault, path);
            persistent_vault.save().await?;
            Self::set_client(client, persistent_vault);
            Ok(())
        }

        pub async fn load_vault(client: &mut Option<StandardClient>, path: &str) -> Fallible<()> {
            let persistent_vault = PersistentDidVault::load(path).await?;
            Self::set_client(client, persistent_vault);
            Ok(())
        }

        fn set_client(client: &mut Option<StandardClient>, vault: PersistentDidVault) -> () {
            let ledger = HydraDidLedger::new();
            client.replace(StandardClient::new(vault, ledger));
        }
    }
}
