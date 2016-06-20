# nfc-sys

[![Crates.io](https://img.shields.io/crates/v/nfc-sys.svg?maxAge=2592000)](https://crates.io/crates/nfc-sys)

`nfc-sys` provides FFI bindings to [libnfc](https://github.com/nfc-tools/libnfc).

Following the `*-sys` package conventions, the `nfc-sys` package does not define higher-level abstractions over the native library; for a safe implementation, see [nfc](https://github.com/dsgriffin/nfc).

## Installation

Install `libnfc` (e.g. [Debian/Ubuntu](http://nfc-tools.org/index.php?title=Libnfc#Debian_.2F_Ubuntu), `brew install libnfc` using Homebrew on Mac OSx, or on [other systems](http://nfc-tools.org/index.php?title=Libnfc#Installation)).

### Cargo.toml

    [dependencies]
    libc = "0.2.0"
    nfc-sys = "0.1.4"
    
## Example Usage

#### // main.rs
```rust
extern crate nfc_sys;

use ::std::ffi::CStr;

fn main() {
    unsafe {
         // Create new Context and initialize libnfc
         let mut context = nfc_sys::nfc_context_new();
         nfc_sys::nfc_init(&mut context);
    
         if context.is_null() {
             println!("Unable to initialize new nfc context");
         }
    
         let version = CStr::from_ptr(nfc_sys::nfc_version()).to_str().unwrap();
    
         println!("libnfc version: {:?}", version);
    }
}
```    
## Contributing
    
I'm brand new to Rust so any help or constructive information would be really appreciated. Thanks in advance!    
    
## License
    
MIT
