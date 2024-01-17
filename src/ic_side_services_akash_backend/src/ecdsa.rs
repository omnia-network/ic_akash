use cosmrs::{crypto::PublicKey, proto::cosmos::crypto::secp256k1::PubKey};
use ic_cdk::api::management_canister::ecdsa::{ecdsa_public_key, EcdsaPublicKeyArgument};

use crate::EcdsaKeyIds;

pub async fn get_public_key() -> Result<PublicKey, String> {
    let request = EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (res,) = ecdsa_public_key(request)
        .await
        .map_err(|e| format!("ecdsa_public_key failed {}", e.1))?;

    PublicKey::try_from(PubKey {
        key: res.public_key,
    })
    .map_err(|e| e.to_string())
}
