#![cfg(target_os = "macos")]

use std::ffi::c_void;
use tauri::{Window, async_runtime::spawn_blocking};
use crate::macos::{PasskeyRegistrationResult, PasskeyLoginResult, blocking::{run_registration, run_login}};

#[tauri::command]
pub async fn register_passkey(
    domain: String,
    challenge: Vec<u8>,
    username: String,
    user_id: Vec<u8>,
    salt: Vec<u8>,
    window: Window
) -> Result<PasskeyRegistrationResult, String> {
    let raw_ptr = window.ns_window().map_err(|_| "Missing ns_window")? as usize;

    spawn_blocking(move || {
        let window_ptr = raw_ptr as *mut c_void;
        run_registration(window_ptr, &domain, &challenge, &username, &user_id, &salt)
    })
    .await
    .map_err(|e| format!("Join error: {}", e))?
}

#[tauri::command]
pub async fn login_passkey(
    domain: String,
    challenge: Vec<u8>,
    salt: Vec<u8>,
    window: Window
) -> Result<PasskeyLoginResult, String> {
    let raw_ptr = window.ns_window().map_err(|_| "Missing ns_window")? as usize;

    spawn_blocking(move || {
        let window_ptr = raw_ptr as *mut c_void;
        run_login(window_ptr, &domain, &challenge, &salt)
    })
    .await
    .map_err(|e| format!("Join error: {}", e))?
}
