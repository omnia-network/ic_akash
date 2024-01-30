use candid::Deserialize;
use serde::Serialize;
use uuid::{Builder, Uuid as UuidImpl};

use crate::rand::with_random_bytes;

const UUID_SIZE: usize = 16;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default, Ord, PartialOrd, PartialEq, Eq)]
pub struct Uuid(UuidImpl);

impl Uuid {
    pub async fn new() -> Result<Self, String> {
        with_random_bytes(|bytes: [u8; UUID_SIZE]| Self::from_random_bytes(bytes)).await
    }

    pub fn from_random_bytes(bytes: [u8; UUID_SIZE]) -> Self {
        Self(Builder::from_random_bytes(bytes).into_uuid())
    }
}

impl TryFrom<&str> for Uuid {
    type Error = String;

    fn try_from(uuid: &str) -> Result<Uuid, Self::Error> {
        let uuid = UuidImpl::parse_str(uuid)
            .map_err(|_| format!("Failed to parse UUID from string: {}", uuid).to_string())?;

        Ok(Self(uuid))
    }
}

impl ToString for Uuid {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
