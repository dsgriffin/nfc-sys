# nfc-sys

[![Crates.io](https://img.shields.io/crates/v/nfc-sys.svg?maxAge=2592000)](https://crates.io/crates/nfc-sys)

Raw Rust FFI bindings for [libnfc](https://github.com/nfc-tools/libnfc), the
Free/Libre Near Field Communication library.

Following Rust `*-sys` crate conventions, this crate exposes libnfc's C API as
directly as possible. It does not provide ownership management, lifetime checks,
or high-level NFC abstractions. Almost every useful operation is therefore
`unsafe` and follows the same preconditions as the native libnfc function.

## Installation

Install libnfc before building this crate.

On macOS with Homebrew:

```sh
brew install libnfc
```

On Debian or Ubuntu:

```sh
sudo apt install libnfc-dev
```

### Cargo.toml

```toml
[dependencies]
nfc-sys = "1.0.0"
```

## Linking

The build script links against a system-provided `libnfc`.

On macOS, it automatically checks common Homebrew locations, including
`/opt/homebrew/opt/libnfc` and `/usr/local/opt/libnfc`.

If libnfc is installed in a non-standard location, set `LIBNFC_LIB_DIR` to the
directory containing the native library.

```sh
LIBNFC_LIB_DIR=/path/to/lib cargo build
```

Set `LIBNFC_NO_HOMEBREW=1` to skip Homebrew probing.

## Example

```rust
use std::ffi::CStr;
use std::ptr;

use nfc_sys::{nfc_exit, nfc_init, nfc_version};

fn main() {
    unsafe {
        let mut context = ptr::null_mut();
        nfc_init(&mut context);

        if context.is_null() {
            eprintln!("Unable to initialize libnfc context");
            return;
        }

        let version = CStr::from_ptr(nfc_version()).to_string_lossy();
        println!("libnfc version: {}", version);

        nfc_exit(context);
    }
}
```

## Safety

This crate is intentionally unsafe. Callers are responsible for passing valid
pointers, honoring libnfc's initialization and shutdown rules, checking return
codes, and freeing native allocations with the correct libnfc function.

For example, buffers returned by functions such as
`nfc_device_get_information_about` and `str_nfc_target` must be released with
`nfc_free`.

## Versioning

Version `1.0.0` aligns the bindings with libnfc's public 1.8.x headers and makes
libnfc-owned handle types opaque. This is a breaking change from older `0.x`
releases, which exposed several private libnfc internals.

## Contributing

Issues and pull requests are welcome. Changes to the FFI surface should be
checked against the public libnfc headers and verified with `cargo test`.

## License

MIT
