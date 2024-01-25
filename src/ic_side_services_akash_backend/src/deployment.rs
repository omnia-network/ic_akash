use std::str::FromStr;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use cosmrs::{crypto::PublicKey, tx::Fee, Any, Coin, Denom};
use ic_cdk::print;

use crate::{
    address::get_account_id_from_public_key,
    proto::deployment::{
        deployment::DeploymentID,
        deploymentmsg::{MsgCloseDeployment, MsgCreateDeployment},
    },
    sdl::SdlV3,
    tx::create_tx,
};

pub async fn create_deployment_tx(
    sender_public_key: &PublicKey,
    height: u64,
    sequence_number: u64,
) -> Result<String, String> {
    // hash of this deployment (base64): TGNKUw/ffyyB/d0EaY9FWMEIhsBzcjY3PLBRHYDqszs=
    // see https://deploy.cloudmos.io/transactions/268DEE51F9FAB84B1BABCD916092D380784A483EA088345CF7B86657BBC8A4DA?network=sandbox
    let sdl_str = r#"
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
    "#;

    let sdl = SdlV3::try_from_str(sdl_str).unwrap();
    print(format!("sdl: {:?}", sdl));
    print(format!("sdl groups: {:?}", sdl.groups()));
    print(format!(
        "sdl manifest sorted: {}",
        sdl.manifest_sorted_json()
    ));
    print(format!(
        "sdl version (base64): {}",
        STANDARD.encode(sdl.manifest_version())
    ));

    // see https://github.com/akash-network/cloudmos/blob/8a8098b7e371e801dad3aad81ef92b8dfe387e4c/deploy-web/src/utils/deploymentData/v1beta3.ts#L230
    let msg = MsgCreateDeployment {
        id: Some(DeploymentID {
            owner: get_account_id_from_public_key(sender_public_key)
                .unwrap()
                .to_string(),
            // see https://github.com/akash-network/cloudmos/blob/8a8098b7e371e801dad3aad81ef92b8dfe387e4c/deploy-web/src/utils/deploymentData/v1beta3.ts#L248C27-L248C27
            dseq: height, // obtained from /blocks/latest RPC call
        }),
        groups: sdl.groups(),
        version: sdl.manifest_version(),
        deposit: Some(
            Coin {
                amount: 5_000_000u128,
                denom: Denom::from_str("uakt").unwrap(),
            }
            .into(),
        ),
        depositor: get_account_id_from_public_key(sender_public_key)
            .unwrap()
            .to_string(),
    };

    let amount = Coin {
        amount: 5_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    let gas = 150_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    create_tx(
        &sender_public_key,
        Any::from_msg(&msg).unwrap(),
        fee,
        sequence_number,
    )
    .await
}

pub async fn close_deployment_tx(
    sender_public_key: &PublicKey,
    dseq: u64,
    sequence_number: u64,
) -> Result<String, String> {
    let msg = MsgCloseDeployment {
        ID: Some(DeploymentID {
            owner: get_account_id_from_public_key(sender_public_key)
                .unwrap()
                .to_string(),
            dseq,
        }),
    };

    let amount = Coin {
        amount: 20_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    let gas = 800_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    create_tx(
        &sender_public_key,
        Any::from_msg(&msg).unwrap(),
        fee,
        sequence_number,
    )
    .await
}