// TODO provide a C API that allows
// 1. selecting a DID from a vault
// 2. selecting a key for a DID
// 3. sign content with the selected key
// +1 maybe later: create a witness request

use std::cell::RefCell;
use std::ffi;
use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use failure::{err_msg, Fallible};
use serde_json;

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
pub extern "C" fn ping(
    message: *const raw::c_char, delay_secs: u32, id: *mut RequestId,
    success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
) -> () {
    let fut = async {
        let message = str_in(message)?;
        tokio::time::delay_for(std::time::Duration::from_secs(delay_secs.into())).await;
        if message.starts_with("fail") {
            return Err(err_msg(message));
        }
        let out = format!(
            "From Rust: You sent '{}'. It works even with async operations involved.",
            message
        );
        Ok(string_out(out))
    };
    CallContext::new(id, success, error).run_async(fut)
}

#[no_mangle]
pub extern "C" fn init_sdk(
    id: *mut RequestId, success: Callback<*mut imp::SdkContext>,
    error: Callback<*const raw::c_char>,
) -> () {
    let fun = || {
        let sdk = imp::SdkContext::default();
        Ok(Box::into_raw(Box::new(sdk)))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn create_vault(
    sdk: *mut imp::SdkContext, seed: *const raw::c_char, path: *const raw::c_char,
    id: *mut RequestId, success: Callback<*const raw::c_void>, error: Callback<*const raw::c_char>,
) -> () {
    let sdk = unsafe { &mut *sdk };
    let fut = async {
        sdk.create_vault(str_in(seed)?, str_in(path)?).await?;
        Ok(std::ptr::null())
    };
    CallContext::new(id, success, error).run_async(fut)
}

#[no_mangle]
pub extern "C" fn load_vault(
    sdk: *mut imp::SdkContext, path: *const raw::c_char, id: *mut RequestId,
    success: Callback<*const raw::c_void>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fut = async {
        sdk.load_vault(str_in(path)?).await?;
        Ok(std::ptr::null())
    };
    CallContext::new(id, success, error).run_async(fut)
}

#[no_mangle]
pub extern "C" fn fake_ledger(
    sdk: *mut imp::SdkContext, id: *mut RequestId, success: Callback<*const raw::c_void>,
    error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fut = async {
        sdk.fake_ledger().await?;
        Ok(std::ptr::null())
    };
    CallContext::new(id, success, error).run_async(fut)
}

#[no_mangle]
pub extern "C" fn real_ledger(
    sdk: *mut imp::SdkContext, url: *const raw::c_char, id: *mut RequestId,
    success: Callback<*const raw::c_void>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fut = async {
        sdk.real_ledger(str_in(url)?).await?;
        Ok(std::ptr::null())
    };
    CallContext::new(id, success, error).run_async(fut)
}

#[repr(C)]
pub struct RawSlice<T> {
    first: *mut T,
    length: usize,
}

impl<T> From<&mut [T]> for RawSlice<T> {
    fn from(slice: &mut [T]) -> Self {
        let first = slice.as_mut_ptr();
        let length = slice.len();
        Self { first, length }
    }
}

#[no_mangle]
pub extern "C" fn list_dids(
    sdk: *mut imp::SdkContext, id: *mut RequestId,
    success: Callback<*mut RawSlice<*mut raw::c_char>>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fut = async {
        let did_vec = sdk.list_dids().await?;
        let cptr_box_slice =
            did_vec.iter().map(|did| string_out(did.to_string())).collect::<Box<[_]>>();
        let raw_box_slice = Box::into_raw(cptr_box_slice);
        let raw_slice: RawSlice<*mut raw::c_char> = unsafe { &mut *raw_box_slice }.into();
        //unsafe { Box::from_raw(raw_box_slice) };
        Ok(Box::into_raw(Box::new(raw_slice)))
    };
    CallContext::new(id, success, error).run_async(fut)
}

#[no_mangle]
pub extern "C" fn create_did(
    sdk: *mut imp::SdkContext, id: *mut RequestId, success: Callback<*mut raw::c_char>,
    error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fut = async {
        let did = sdk.create_did().await?;
        Ok(string_out(did.to_string()))
    };
    CallContext::new(id, success, error).run_async(fut)
}

#[no_mangle]
pub extern "C" fn get_document(
    sdk: *mut imp::SdkContext, did: *const raw::c_char, id: *mut RequestId,
    success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fut = async {
        let did_str = str_in(did)?;
        let did = did_str.parse()?;
        let document = sdk.get_document(&did).await?;
        let json = serde_json::to_string(&document)?;
        Ok(string_out(json))
    };
    CallContext::new(id, success, error).run_async(fut)
}

#[no_mangle]
pub extern "C" fn sign_witness_request(
    sdk: *mut imp::SdkContext, req: *const raw::c_char, auth: *const raw::c_char,
    id: *mut RequestId, success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fut = async {
        let req_str = str_in(req)?;
        //let req = req_str.parse()?;
        let auth_str = format!("{:?}", str_in(auth)?);
        let auth = serde_json::from_str(&auth_str)?;
        let signed_request = sdk.sign_witness_request(req_str.to_owned(), &auth).await?;
        let json = serde_json::to_string(&signed_request)?;
        Ok(string_out(json))
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
        crypto::sign::{Signable, Signed},
        data::{auth::Authentication, did::Did, diddoc::DidDocument},
        io::dist::did::{HydraDidLedger, /*FakeDidLedger, */ LedgerOperations, LedgerQueries},
        io::local::didvault::{DidVault, FilePersister, InMemoryDidVault, PersistentDidVault},
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
            let file_persister = Box::new(FilePersister::new(&path));
            let mut persistent_vault = PersistentDidVault::new(mem_vault, file_persister);
            persistent_vault.save().await?;
            self.client.set_vault(persistent_vault)
        }

        pub async fn load_vault(&mut self, path: &str) -> Fallible<()> {
            let file_persister = Box::new(FilePersister::new(&path));
            let persistent_vault = PersistentDidVault::load(file_persister).await?;
            self.client.set_vault(persistent_vault)
        }

        pub async fn fake_ledger(&mut self) -> Fallible<()> {
            todo!();
            // self.client.set_ledger(FakeDidLedger::new())?;
            // Ok(())
        }

        pub async fn real_ledger(&mut self, url: &str) -> Fallible<()> {
            self.client.set_ledger(HydraDidLedger::new(url))?;
            Ok(())
            // Err(err_msg("Not implemented yet"))
        }

        pub async fn list_dids(&self) -> Fallible<Vec<Did>> {
            self.client.vault()?.dids()
        }

        pub async fn create_did(&mut self) -> Fallible<Did> {
            let vault = self.client.mut_vault()?;
            let rec = vault.create(None).await?;
            Ok(rec.did())
        }

        pub async fn get_document(&self, did: &Did) -> Fallible<DidDocument> {
            let doc = self.client.ledger()?.document(did).await?;
            Ok(doc)
        }

        // TODO REQUEST MUST BE TYPED
        pub async fn sign_witness_request(
            &self, req: String, auth: &Authentication,
        ) -> Fallible<Signed<String>> {
            let vault = self.client.vault()?;
            let signer = vault.signer_by_auth(auth)?;
            req.sign(signer.as_ref()).await
        }
    }
}
