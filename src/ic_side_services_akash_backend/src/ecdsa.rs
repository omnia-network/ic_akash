use cosmrs::{crypto::PublicKey, proto::cosmos::crypto::secp256k1::PubKey};
use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
    SignWithEcdsaArgument,
};

pub enum EcdsaKeyIds {
    #[allow(unused)]
    TestKeyLocalDevelopment,
    #[allow(unused)]
    TestKey1,
    #[allow(unused)]
    ProductionKey1,
}

impl EcdsaKeyIds {
    pub fn to_key_id(&self) -> EcdsaKeyId {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: match self {
                Self::TestKeyLocalDevelopment => "dfx_test_key",
                Self::TestKey1 => "test_key_1",
                Self::ProductionKey1 => "key_1",
            }
            .to_string(),
        }
    }
}

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

pub async fn sign(message_hash: Vec<u8>) -> Result<Vec<u8>, String> {
    let request = SignWithEcdsaArgument {
        message_hash,
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    sign_with_ecdsa(request)
        .await
        .map_err(|e| format!("sign_with_ecdsa failed {}", e.1))
        .map(|(r,)| r.signature)
}
