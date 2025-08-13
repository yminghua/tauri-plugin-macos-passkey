#[cfg(target_os = "macos")]
fn main() {
    use swift_rs::SwiftLinker;

    const COMMANDS: &[&str] = &[
        "register_passkey",
        "login_passkey"
    ];
    
    SwiftLinker::new("15.0")
        .with_package("PasskeyBridge", "swift-lib")
        .link();
    tauri_plugin::Builder::new(COMMANDS).build();
}

// On non‑macOS: no-op
#[cfg(not(target_os = "macos"))]
fn main() {
    println!("cargo:warning=tauri-plugin-macos-passkey: build.rs skipped (non-macOS target)");
}
