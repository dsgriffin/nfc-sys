use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBNFC_LIB_DIR");
    println!("cargo:rerun-if-env-changed=LIBNFC_NO_HOMEBREW");

    if let Ok(lib_dir) = env::var("LIBNFC_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
    } else if env::var_os("LIBNFC_NO_HOMEBREW").is_none() {
        add_homebrew_libnfc();
    }

    println!("cargo:rustc-link-lib=nfc");
}

fn add_homebrew_libnfc() {
    let prefix = Command::new("brew")
        .args(["--prefix", "libnfc"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|stdout| stdout.trim().to_owned());

    let candidates = [
        prefix.as_deref(),
        Some("/opt/homebrew/opt/libnfc"),
        Some("/usr/local/opt/libnfc"),
    ];

    for candidate in candidates.iter().flatten() {
        let lib_dir = Path::new(candidate).join("lib");
        if lib_dir.join("libnfc.dylib").exists()
            || lib_dir.join("libnfc.so").exists()
            || lib_dir.join("libnfc.a").exists()
        {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            return;
        }
    }
}
