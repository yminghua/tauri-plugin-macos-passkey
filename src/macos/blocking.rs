#![cfg(target_os = "macos")]

use std::ffi::c_void;
use futures::executor::block_on;
use crate::macos::{PasskeyRegistrationResult, PasskeyLoginResult, begin_registration_from_rust, begin_login_from_rust};

/// This function runs on a blocking thread — raw pointer allowed
pub fn run_registration(
    window_ptr: *mut c_void,
    domain: &str,
    challenge: &[u8],
    username: &str,
    user_id: &[u8],
    salt: &[u8],
) -> Result<PasskeyRegistrationResult, String> {
    let fut = begin_registration_from_rust(
        window_ptr,
        domain,
        challenge,
        username,
        user_id,
        salt,
    );
    // Block here (safe, because we're in a spawned blocking thread)
    let result = block_on(fut);
    result.ok_or_else(|| "Registration failed".to_string())
}

/// This function runs on a blocking thread — raw pointer allowed
pub fn run_login(
    window_ptr: *mut c_void,
    domain: &str,
    challenge: &[u8],
    salt: &[u8],
) -> Result<PasskeyLoginResult, String> {
    let fut = begin_login_from_rust(
        window_ptr,
        domain,
        challenge,
        salt,
    );
    // Block here (safe, because we're in a spawned blocking thread)
    let result = block_on(fut);
    result.ok_or_else(|| "Login failed".to_string())
}
