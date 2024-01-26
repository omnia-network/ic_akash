use std::str::FromStr;

use candid::Principal;
use cosmrs::{
    bank::MsgSend,
    tx::{Fee, Msg},
    AccountId, Coin, Denom,
};
use ic_cdk::{init, post_upgrade, query, update};

use akash::{
    address::get_account_id_from_public_key,
    auth::{account_request, account_response},
    bids::{bids_request, bids_response},
    deployment::{close_deployment_tx, create_deployment_tx},
    lease::create_lease_tx,
    provider::{provider_request, provider_response},
    tx::{create_certificate_tx, create_tx},
};
use config::{set_config, Config};
use ecdsa::get_public_key;

mod akash;
mod config;
mod ecdsa;
mod hash;
mod utils;

#[init]
fn init(tendermint_rpc_canister_id: Principal) {
    let config = Config::new(tendermint_rpc_canister_id);

    set_config(config);
}

#[post_upgrade]
fn post_upgrade(tendermint_rpc_canister_id: Principal) {
    init(tendermint_rpc_canister_id);
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
async fn create_deployment_transaction(height: u64, sequence_number: u64) -> String {
    let public_key = get_public_key().await.unwrap();

    create_deployment_tx(&public_key, height, sequence_number)
        .await
        .unwrap()
}

#[update]
async fn close_deployment_transaction(height: u64, sequence_number: u64) -> String {
    let public_key = get_public_key().await.unwrap();

    close_deployment_tx(&public_key, height, sequence_number)
        .await
        .unwrap()
}

#[update]
async fn fetch_bids(dseq: u64) -> String {
    let public_key = get_public_key().await.unwrap();

    bids_request(&public_key, dseq).unwrap()
}

#[query]
fn decode_bids_response(hex_data: String) {
    bids_response(hex_data);
}

#[update]
async fn get_account() -> String {
    let public_key = get_public_key().await.unwrap();

    account_request(&public_key).unwrap()
}

#[query]
fn decode_account_response(hex_data: String) {
    account_response(hex_data);
}

#[update]
async fn create_lease(sequence_number: u64) -> String {
    let public_key = get_public_key().await.unwrap();

    create_lease_tx(&public_key, sequence_number).await.unwrap()
}

#[query]
fn get_provider() -> String {
    provider_request().unwrap()
}

#[query]
fn decode_provider_response(hex_data: String) {
    provider_response(hex_data);
}

// In the following, we register a custom getrandom implementation because
// otherwise getrandom (which is a dependency of k256) fails to compile.
// This is necessary because getrandom by default fails to compile for the
// wasm32-unknown-unknown target (which is required for deploying a canister).
getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}
