#[cfg(not(esp_idf_version_major = "5"))]
compile_error!("Meant for ESP_IDF 5.3 .. 5.5");

// esp_idf_version: "{major}.{minor}" // always just these two

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by 'esp-idf-sys' might not link properly.
    // See https://github.com/esp-rs/esp-idf-template/issues/71
    //
    esp_idf_svc::sys::link_patches();

    // For logging options, see -> https://github.com/esp-rs/esp-idf-svc/blob/master/examples/logging.rs
    //
    // Bind the log crate to the ESP Logging facilities
    #[cfg(any(esp_idf_version = "5.4", esp_idf_version = "5.5"))]
    esp_idf_svc::log::init_from_esp_idf();  // > 5.3

    #[cfg(esp_idf_version = "5.3")]
    esp_idf_svc::log::EspLogger::initialize_default();  // 5.3

    log::info!("Hello, world!");
}
