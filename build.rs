/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*/
use std::{
    env,
};

/*
*/
fn main() {
    // Needed. E.g. "emits the necessary cfg flags for conditional compilation" (and likely way more..)
    embuild::espidf::sysenv::output();

    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    {
        if env::var("__CFBundleIdentifier").is_ok() {
            return;  // skip the rest
        }
    }

    // Do NOT allow build if a system-wide ESP-IDF is active.
    {
        if env::var("IDF_PATH").is_ok() {
            panic!("❗️Please build with a shell that doesn't know of system-wide esp-idf. 'IDF_PATH' env.var. detected.");
        }
    }

    // DEBUG: Show what we know about the compilation.
    #[cfg(false)]
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        panic!();
    }

    println!(r#"cargo::rustc-check-cfg=cfg(esp_idf_version_major, values("5"))"#);
    println!(r#"cargo::rustc-check-cfg=cfg(esp_idf_version, values("5.3", "5.4", "5.5"))"#);
}
