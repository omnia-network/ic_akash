use std::str::FromStr;

use address::get_account_id_from_public_key;
use candid::CandidType;
use cosmrs::{
    bank::MsgSend,
    tx::{Fee, Msg},
    AccountId, Coin, Denom,
};
use ecdsa::get_public_key;
use ic_cdk::{
    api::management_canister::ecdsa::{
        sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, SignWithEcdsaArgument,
    },
    update,
};
use serde::Serialize;
use sha2::Digest;
use tx::{create_certificate_tx, create_tx};

mod address;
mod ecdsa;
mod proto;
mod tx;
mod utils;

#[derive(CandidType, Serialize, Debug)]
struct PublicKeyReply {
    pub public_key_hex: String,
}

#[derive(CandidType, Serialize, Debug)]
struct SignatureReply {
    pub signature_hex: String,
}

#[derive(CandidType, Serialize, Debug)]
struct SignatureVerificationReply {
    pub is_signature_valid: bool,
}

#[update]
async fn address() -> Result<String, String> {
    let public_key = get_public_key().await.unwrap();

    let sender_account_id = get_account_id_from_public_key(&public_key).unwrap();
    Ok(sender_account_id.to_string())
}

#[update]
async fn create_transaction() -> String {
    let public_key = get_public_key().await.unwrap();

    let sender_account_id = get_account_id_from_public_key(&public_key).unwrap();
    let recipient_account_id =
        AccountId::from_str("akash13gtrvjrzx8tst260ucszcflt4wny68shwdmrxs").unwrap();

    // We'll be doing a simple send transaction.
    // First we'll create a "Coin" amount to be sent.
    let amount = Coin {
        amount: 250_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    // Next we'll create a send message (from the "bank" module) for the coin
    // amount we created above.
    let msg_send = MsgSend {
        from_address: sender_account_id.clone(),
        to_address: recipient_account_id,
        amount: vec![amount.clone()],
    };

    let gas = 100_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);
    let sequence_num = 0;

    create_tx(&public_key, msg_send.to_any().unwrap(), fee, sequence_num)
        .await
        .unwrap()
}

#[update]
async fn create_certificate_transaction(
    cert_pem_base64: String,
    pub_key_pem_base64: String,
) -> String {
    let public_key = get_public_key().await.unwrap();

    create_certificate_tx(&public_key, cert_pem_base64, pub_key_pem_base64)
        .await
        .unwrap()
}

#[update]
async fn sign(message: String) -> Result<SignatureReply, String> {
    let request = SignWithEcdsaArgument {
        message_hash: sha256(&message.into_bytes()).to_vec(),
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (response,) = sign_with_ecdsa(request)
        .await
        .map_err(|e| format!("sign_with_ecdsa failed {}", e.1))?;

    Ok(SignatureReply {
        signature_hex: hex::encode(&response.signature),
    })
}

// #[query]
// async fn verify(
//     signature_hex: String,
//     message: String,
//     public_key_hex: String,
// ) -> Result<SignatureVerificationReply, String> {
//     let signature_bytes = hex::decode(&signature_hex).expect("failed to hex-decode signature");
//     let pubkey_bytes = hex::decode(&public_key_hex).expect("failed to hex-decode public key");
//     let message_bytes = message.as_bytes();

//     use k256::ecdsa::signature::Verifier;
//     let signature = k256::ecdsa::Signature::try_from(signature_bytes.as_slice())
//         .expect("failed to deserialize signature");
//     let is_signature_valid = k256::ecdsa::VerifyingKey::from_sec1_bytes(&pubkey_bytes)
//         .expect("failed to deserialize sec1 encoding into public key")
//         .verify(message_bytes, &signature)
//         .is_ok();

//     Ok(SignatureVerificationReply { is_signature_valid })
// }

fn sha256(input: &[u8]) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}

enum EcdsaKeyIds {
    #[allow(unused)]
    TestKeyLocalDevelopment,
    #[allow(unused)]
    TestKey1,
    #[allow(unused)]
    ProductionKey1,
}

impl EcdsaKeyIds {
    fn to_key_id(&self) -> EcdsaKeyId {
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

// In the following, we register a custom getrandom implementation because
// otherwise getrandom (which is a dependency of k256) fails to compile.
// This is necessary because getrandom by default fails to compile for the
// wasm32-unknown-unknown target (which is required for deploying a canister).
// Our custom implementation always fails, which is sufficient here because
// we only use the k256 crate for verifying secp256k1 signatures, and such
// signature verification does not require any randomness.
getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}
