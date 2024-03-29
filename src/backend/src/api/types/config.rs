use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode};
use cosmrs::crypto::PublicKey;
use ic_stable_structures::{storable::Bound, Storable};

use crate::helpers::{get_public_key, EcdsaKeyIds};

#[derive(CandidType, Clone, Deserialize)]
pub struct Config {
    is_mainnet: bool,
    ecdsa_key: EcdsaKeyIds,
    tendermint_rpc_url: String,
}

impl Config {
    pub fn new_mainnet(ecdsa_key: EcdsaKeyIds, tendermint_rpc_url: &str) -> Self {
        Self {
            is_mainnet: true,
            ecdsa_key,
            tendermint_rpc_url: tendermint_rpc_url.to_string(),
        }
    }

    pub fn is_mainnet(&self) -> bool {
        self.is_mainnet
    }

    pub fn ecdsa_key(&self) -> EcdsaKeyIds {
        self.ecdsa_key.clone()
    }

    pub fn tendermint_rpc_url(&self) -> String {
        self.tendermint_rpc_url.clone()
    }

    pub async fn public_key(&self) -> Result<PublicKey, String> {
        get_public_key(self.ecdsa_key()).await
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            is_mainnet: false,
            ecdsa_key: EcdsaKeyIds::TestKeyLocalDevelopment,
            tendermint_rpc_url: "https://rpc.sandbox-01.aksh.pw".to_string(),
        }
    }
}

impl Storable for Config {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
