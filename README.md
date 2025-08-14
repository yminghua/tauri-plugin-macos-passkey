# tauri-plugin-macos-passkey

[![Crates.io Version](https://img.shields.io/crates/v/tauri-plugin-macos-passkey)](https://crates.io/crates/tauri-plugin-macos-passkey)
[![Crates.io Downloads](https://img.shields.io/crates/d/tauri-plugin-macos-passkey)](https://crates.io/crates/tauri-plugin-macos-passkey)
![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/tauri-plugin-macos-passkey)
[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blueviolet?logo=tauri)](https://v2.tauri.app/)
![macOS 15+](https://img.shields.io/badge/macOS-15%2B-success?logo=apple)

A Tauri plugin that lets your Tauri app call macOS native Passkey APIs for registration and login, with optional support for returning the Passkey PRF extension output.

## Setup

For detailed setup instructions, see the [tauri-passkey-demo](https://github.com/yminghua/tauri-passkey-demo) repository.

## APIs

### `register_passkey`

#### Signature (frontend via invoke)

```typescript
invoke("plugin:macos-passkey|register_passkey", {
  domain: string,
  challenge: number[],
  username: string,
  userId: number[],
  salt: number[]  // pass [] to skip PRF
})
```

#### Returns

```typescript
{
  id: string,
  raw_id: string,
  client_data_json: string,
  attestation_object: string,
  prf_output: number[]  // empty if PRF skipped
}
```

### `login_passkey`

#### Signature (frontend via invoke)

```typescript
invoke("plugin:macos-passkey|login_passkey", {
  domain: string,
  challenge: number[],
  salt: number[]  // pass [] to skip PRF
})
```

#### Returns

```typescript
{
  id: string,
  raw_id: string,
  client_data_json: string,
  authenticator_data: string,
  signature: string,
  user_handle: string,
  prf_output: number[]  // empty if PRF skipped
}
```

## Notes

- **macOS version**: Requires macOS 15 or later.
- **PRF Output**: If you don’t need PRF, pass an empty salt (`[]`) and the plugin will skip the PRF extension.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
