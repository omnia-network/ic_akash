use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    STANDARD.decode(data).map_err(|e| e.to_string())
}

pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}
