#![no_std]
#![no_main]

#[cfg(not(esp_idf_version_major = "5"))]
compile_error!("Meant for ESP_IDF 5.3 .. 5.5");

extern crate alloc;

mod logging;
use logging::esp_log_init;   // ties esp-idf-sys logging to 'log'

#[unsafe(no_mangle)]
pub extern "C" fn app_main() {
    esp_log_init();

    log::info!("Hello, world!");
}
