/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*
* We call GNU 'make' from within the 'build.rs'. This allows us to build in the normal Cargo way,
* yet benefit from file system dependency trees.
*/
use anyhow::*;

use std::{
    env,
};

/*
*/
fn main() -> Result<()> {

    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    {
        if env::var("__CFBundleIdentifier").is_ok() {
            return Ok(());  // skip the rest
        }
    }

    // Do NOT allow build if a system-wide ESP-IDF is active.
    {
        if env::var("IDF_PATH").is_ok() {
            panic!("❗️Please build with a shell that doesn't know of system-wide esp-idf. 'IDF_PATH' env.var. detected.");
        }
    }

    // DEBUG: Show what we know about the compilation.
    #[cfg(false)]   // see -> .env.dump
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        panic!();
    }

    // 'esp-idf-svc' 0.51 falls down to 5.2.3 if it doesn't find the version we wish. This is unwelcome.
    //  <<
    //    DEP_ESP_IDF_SVC_EMBUILD_ESP_IDF_PATH=/home/ubuntu/.embuild/espressif/esp-idf/v5.2.3
    //  <<
    //
    // 'esp-idf-svc' 'master' (8-Mar-26) has this, instead:
    //  <<
    //    esp_idf_version=v5.5
    //  <<
    #[cfg(false)]
    {
        const NAME: &str = "DEP_ESP_IDF_SVC_EMBUILD_ESP_IDF_PATH";
        match env::var(NAME)
            .expect(format!("env.var '{NAME}' not detected").as_str()) {
            path if path.ends_with("v5.2.3") => {
                panic!("ESP-IDF version fallback to: {}", path)
            }
            _ => { /*proceed*/ }
        }
    }

    embuild::espidf::sysenv::output();

    Ok(())
}
