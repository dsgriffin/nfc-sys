# nfc-sys

[![Crates.io](https://img.shields.io/crates/v/nfc-sys.svg?maxAge=2592000)](https://crates.io/crates/nfc-sys)

`nfc-sys` provides FFI bindings to [libnfc](https://github.com/nfc-tools/libnfc).

Following the `*-sys` package conventions, the `nfc-sys` package does not define higher-level abstractions over the native library.

## Installation

Install `libnfc` (e.g. [Debian/Ubuntu](http://nfc-tools.org/index.php?title=Libnfc#Debian_.2F_Ubuntu), `brew install libnfc` using Homebrew on Mac OSx, or on [other systems](http://nfc-tools.org/index.php?title=Libnfc#Installation)).

### Cargo.toml

    [dependencies]
    libc = "0.2.0"
    nfc-sys = "0.1.2"
    
## Example Usage

#### // main.rs    
    extern crate libc;
    extern crate nfc_sys;
    
    use nfc_sys::nfc_version;
    use std::ffi::CStr;
    use std::str;
    
    fn main() {
        unsafe {
            let slice = CStr::from_ptr(nfc_version());
            println!("libnfc version: {}", slice.to_str().unwrap()); // o: libnfc version: libnfc-1.7.1-79-ge23f8a9
        }
    }
    
## Contributing
    
I'm brand new to Rust so any help or constructive information would be really appreciated. Thanks in advance!    
    
## License
    
MIT