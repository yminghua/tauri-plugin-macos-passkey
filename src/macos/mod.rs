#![cfg(target_os = "macos")]

pub mod blocking;
pub mod commands;

use std::{os::raw::c_void};
use swift_rs::{SRData, SRString, SRObject};
use tokio::sync::oneshot;

// --- FFI bindings ---
extern "C" {
    pub fn begin_passkey_registration(
        window_ptr: *mut c_void,
        domain: SRString,
        challenge: SRData,
        username: SRString,
        user_id: SRData,
        salt: SRData,
        context: u64,
        callback: PasskeyResultCallback
    );

    pub fn begin_passkey_login(
        window_ptr: *mut c_void,
        domain: SRString,
        challenge: SRData,
        salt: SRData,
        context: u64,
        callback: PasskeyResultCallback
    );
}

// --- Result structs exposed to frontend ---
#[derive(Debug, serde::Serialize)]
pub struct PasskeyRegistrationResult {
    pub id: String,
    pub raw_id: String,
    pub client_data_json: String,
    pub attestation_object: String,
    pub prf_output: Vec<u8>
}

#[derive(Debug, serde::Serialize)]
pub struct PasskeyLoginResult {
    pub id: String,
    pub raw_id: String,
    pub client_data_json: String,
    pub authenticator_data: String,
    pub signature: String,
    pub user_handle: String,
    pub prf_output: Vec<u8>
}

// --- FFI structs used to match Swift NSObject layout ---
#[allow(non_snake_case)]
#[repr(C)]
pub struct RegistrationResultObject {
    pub id: SRString,
    pub rawId: SRString,
    pub clientDataJSON: SRString,
    pub attestationObject: SRString,
    pub prfOutput: SRData
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct LoginResultObject {
    pub id: SRString,
    pub rawId: SRString,
    pub clientDataJSON: SRString,
    pub authenticatorData: SRString,
    pub signature: SRString,
    pub userHandle: SRString,
    pub prfOutput: SRData
}

// --- FFI callback type ---
type PasskeyResultCallback = unsafe extern "C" fn(result: *mut c_void, context: u64);

// --- FFI callback implementation ---
extern "C" fn passkey_result_callback(result: *mut c_void, context: u64) {
    let sender: Box<oneshot::Sender<*mut c_void>> = unsafe { Box::from_raw(context as *mut _) };
    let _ = sender.send(result);
}

// --- Helper function to convert raw pointer to SRObject ---
pub unsafe fn sr_object_from_raw<T>(ptr: *mut c_void) -> SRObject<T> {
    std::mem::transmute(ptr)
}

/// Async wrapper for passkey registration
pub async fn begin_registration_from_rust(
    window_ptr: *mut c_void,
    domain: &str,
    challenge: &[u8],
    username: &str,
    user_id: &[u8],
    salt: &[u8]
) -> Option<PasskeyRegistrationResult> {
    let (sender, receiver) = oneshot::channel::<*mut c_void>();
    let context = Box::into_raw(Box::new(sender)) as u64;

    unsafe {
        begin_passkey_registration(
            window_ptr,
            SRString::from(domain),
            SRData::from(challenge),
            SRString::from(username),
            SRData::from(user_id),
            SRData::from(salt),
            context,
            passkey_result_callback
        );
    }

    if let Ok(ptr) = receiver.await {
        if ptr.is_null() {
            return None;
        }

        let obj: SRObject<RegistrationResultObject> = unsafe { sr_object_from_raw(ptr) };

        let result = PasskeyRegistrationResult {
            id: obj.id.to_string(),
            raw_id: obj.rawId.to_string(),
            client_data_json: obj.clientDataJSON.to_string(),
            attestation_object: obj.attestationObject.to_string(),
            prf_output: obj.prfOutput.to_vec()
        };

        Some(result)
    } else {
        None
    }
}

pub async fn begin_login_from_rust(
    window_ptr: *mut c_void,
    domain: &str,
    challenge: &[u8],
    salt: &[u8]
) -> Option<PasskeyLoginResult> {
    let (sender, receiver) = oneshot::channel::<*mut c_void>();
    let context = Box::into_raw(Box::new(sender)) as u64;

    unsafe {
        begin_passkey_login(
            window_ptr,
            SRString::from(domain),
            SRData::from(challenge),
            SRData::from(salt),
            context,
            passkey_result_callback
        );
    }

    if let Ok(ptr) = receiver.await {
        if ptr.is_null() {
            return None;
        }

        let obj: SRObject<LoginResultObject> = unsafe { sr_object_from_raw(ptr) };

        let result = PasskeyLoginResult {
            id: obj.id.to_string(),
            raw_id: obj.rawId.to_string(),
            client_data_json: obj.clientDataJSON.to_string(),
            authenticator_data: obj.authenticatorData.to_string(),
            signature: obj.signature.to_string(),
            user_handle: obj.userHandle.to_string(),
            prf_output: obj.prfOutput.to_vec()
        };

        Some(result)
    } else {
        None
    }
}
