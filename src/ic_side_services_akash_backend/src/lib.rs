use ic_cdk::{init, post_upgrade, print, update};

use akash::{
    address::get_account_id_from_public_key,
    auth::get_account,
    bank::create_send_tx,
    bids::fetch_bids,
    deployment::{close_deployment_tx, create_deployment_tx},
    lease::create_lease_tx,
    provider::fetch_provider,
    tx::create_certificate_tx,
};
use config::{set_config, Config};
use ecdsa::get_public_key;
use utils::base64_decode;

mod akash;
mod config;
mod ecdsa;
mod hash;
mod utils;

#[init]
fn init() {
    let config = Config::new();

    set_config(config);
}

#[post_upgrade]
fn post_upgrade() {
    init();
}

#[update]
async fn address() -> Result<String, String> {
    let public_key = get_public_key().await?;

    let sender_account_id = get_account_id_from_public_key(&public_key)?;
    Ok(sender_account_id.to_string())
}

#[update]
async fn send(to_address: String, amount: u64) -> Result<(), String> {
    let public_key = get_public_key().await?;

    let account = get_account(&public_key).await?;
    let sequence_number = if account.sequence > 0 {
        account.sequence + 1
    } else {
        0
    };

    let tx_raw = create_send_tx(
        &public_key,
        to_address,
        amount,
        sequence_number,
        account.account_number,
    )
    .await?;
    // print(format!("tx_raw: {}", hex_encode(&tx_raw)));
    let tx_res = ic_tendermint_rpc::broadcast_tx_sync(tx_raw).await?;

    print(format!("[send] tx_res: {:?}", tx_res));

    Ok(())
}

#[update]
async fn create_certificate(
    cert_pem_base64: String,
    pub_key_pem_base64: String,
) -> Result<(), String> {
    let public_key = get_public_key().await?;

    let cert_pem = base64_decode(&cert_pem_base64)?;
    let pub_key_pem = base64_decode(&pub_key_pem_base64)?;

    let account = get_account(&public_key).await?;
    let sequence_number = if account.sequence > 0 {
        account.sequence + 1
    } else {
        0
    };

    let tx_raw = create_certificate_tx(
        &public_key,
        cert_pem,
        pub_key_pem,
        sequence_number,
        account.account_number,
    )
    .await?;
    let tx_res = ic_tendermint_rpc::broadcast_tx_sync(tx_raw).await?;

    print(format!("[create_certificate] tx_res: {:?}", tx_res));

    Ok(())
}

#[update]
async fn deploy() -> Result<(String, String), String> {
    let public_key = get_public_key().await?;

    let account = get_account(&public_key).await?;
    let mut sequence_number = if account.sequence > 0 {
        account.sequence + 1
    } else {
        0
    };

    let abci_info_res = ic_tendermint_rpc::abci_info().await?;
    let height = abci_info_res.response.last_block_height;

    let sdl_raw = example_sdl();

    let (sdl, tx_raw) = create_deployment_tx(
        &public_key,
        height.value(),
        sequence_number,
        sdl_raw,
        account.account_number,
    )
    .await?;
    let tx_res = ic_tendermint_rpc::broadcast_tx_sync(tx_raw).await?;
    print(format!("[create_deployment] tx_res: {:?}", tx_res));

    sequence_number += 1;

    let bid = fetch_bids(&public_key, height.value()).await?[0]
        .bid
        .clone()
        .unwrap();
    let bid_id = bid.bid_id.unwrap();

    let tx_raw = create_lease_tx(
        &public_key,
        sequence_number,
        bid_id.clone(),
        account.account_number,
    )
    .await?;
    let tx_res = ic_tendermint_rpc::broadcast_tx_sync(tx_raw).await?;
    print(format!("[create_lease] tx_res: {:?}", tx_res));

    let provider = fetch_provider(bid_id.owner).await?;

    let deployment_url = format!(
        "https://{}/deployment/{}/manifest",
        provider.hostURI, bid_id.dseq
    );

    Ok((deployment_url, sdl.manifest_sorted_json()))
}

#[update]
async fn close_deployment(dseq: u64) -> Result<(), String> {
    let public_key = get_public_key().await.unwrap();

    let account = get_account(&public_key).await?;
    let sequence_number = if account.sequence > 0 {
        account.sequence + 1
    } else {
        0
    };

    let tx_raw =
        close_deployment_tx(&public_key, dseq, sequence_number, account.account_number).await?;
    let tx_res = ic_tendermint_rpc::broadcast_tx_sync(tx_raw).await?;
    print(format!("[close_deployment] tx_res: {:?}", tx_res));

    Ok(())
}

fn example_sdl<'a>() -> &'a str {
    // hash of this deployment (base64): TGNKUw/ffyyB/d0EaY9FWMEIhsBzcjY3PLBRHYDqszs=
    // see https://deploy.cloudmos.io/transactions/268DEE51F9FAB84B1BABCD916092D380784A483EA088345CF7B86657BBC8A4DA?network=sandbox
    r#"
version: "3.0"
services:
  ic-websocket-gateway:
    image: omniadevs/ic-websocket-gateway
    expose:
      - port: 8080
        as: 80
        accept:
          - "akash-gateway.icws.io"
        to:
          - global: true
    command:
      - "/ic-ws-gateway/ic_websocket_gateway"
      - "--gateway-address"
      - "0.0.0.0:8080"
      - "--ic-network-url"
      - "https://icp-api.io"
      - "--polling-interval"
      - "400"
profiles:
  compute:
    ic-websocket-gateway:
      resources:
        cpu:
          units: 0.5
        memory:
          size: 512Mi
        storage:
          - size: 512Mi
        gpu:
          units: 0
  placement:
    dcloud:
      pricing:
        ic-websocket-gateway:
          denom: uakt
          amount: 1000
deployment:
  ic-websocket-gateway:
    dcloud:
      profile: ic-websocket-gateway
      count: 1
    "#
}

// In the following, we register a custom getrandom implementation because
// otherwise getrandom (which is a dependency of k256) fails to compile.
// This is necessary because getrandom by default fails to compile for the
// wasm32-unknown-unknown target (which is required for deploying a canister).
getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}
