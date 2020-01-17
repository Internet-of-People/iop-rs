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

type Callback<T> = extern "C" fn(T) -> ();

#[no_mangle]
pub extern "C" fn init(success: Callback<*const raw::c_void>, error: Callback<*const raw::c_char>) -> () {
    match imp::SdkContext::new() {
        Ok(ctx) => success(&ctx),
        Err(e) => error(e.to_string()),
    }
}

#[no_mangle]
pub extern "C" fn create_vault(ctx: *mut imp::SdkContext, seed: *const raw::c_char, path: *const raw::c_char) -> bool {
    let ctx = unsafe { &mut *ctx };
    let may_fail = async { ctx.create_vault(str_in(seed)?, str_in(path)?).await };
    match ctx.run(may_fail) {
        Ok(()) => true,
        Err(e) => false,
    }
}

#[no_mangle]
pub extern "C" fn load_vault(*mut SdkContext, path: *const raw::c_char) -> bool {
    let may_fail = async { safe::load_vault(str_in(path)?).await };
    match may_fail {
        Ok(()) => true,
        Err(e) => false,
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
        client: Option<StandardClient>,
        runtime: tokio::runtime::Runtime,
    }

    impl SdkContext {
        pub fn new() -> Fallible<Self> {
            let client = Default::default();
            let runtime = tokio::runtime::Builder::new().basic_scheduler().enable_all().build()?;
            Ok(Self {
                client,
                runtime
            })
        }

        pub fn run<R>(&mut self, f: &mut (dyn std::future::Future<Output = R> + Unpin)) -> R {
            self.runtime.block_on(f)
        }

        pub async fn create_vault(&mut self, seed: &str, path: &str) -> Fallible<()> {
            let seed = keyvault::Seed::from_bip39(seed)?;
            let mem_vault =  InMemoryDidVault::new(seed);
            let persistent_vault = PersistentDidVault::new(mem_vault, path);
            persistent_vault.save().await;
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
            self.client.replace( StandardClient::new(vault, HydraDidLedger::new()) );
        }
    }
}
