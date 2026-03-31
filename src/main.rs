#[cfg(not(esp_idf_version_major = "5"))]
compile_error!("Meant for ESP_IDF 5.3 .. 5.5");

mod logging;
use logging::esp_log_init;   // ties esp-idf-sys logging to 'log'

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by 'esp-idf-sys' might not link properly.
    // See https://github.com/esp-rs/esp-idf-template/issues/71
    //
    esp_idf_sys::link_patches();

    esp_log_init();

    log::info!("Hello, world!");
}
