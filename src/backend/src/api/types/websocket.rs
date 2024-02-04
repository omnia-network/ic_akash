use candid::{encode_one, CandidType};
use serde::{Deserialize, Serialize};

// this is the application message, you can change it as you wish
#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct AppMessage {
    pub text: String,
}

impl AppMessage {
    pub fn candid_serialize(&self) -> Vec<u8> {
        encode_one(&self).unwrap()
    }
}
