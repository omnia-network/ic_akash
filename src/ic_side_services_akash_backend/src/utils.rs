use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn get_time_nanos() -> u64 {
    ic_cdk::api::time()
}

pub fn get_time_micros() -> u64 {
    get_time_nanos() / 1000
}

pub fn get_time_millis() -> u64 {
    get_time_nanos() / 1_000_000
}

pub fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    STANDARD.decode(&data).map_err(|e| e.to_string())
}
