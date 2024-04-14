use std::str::FromStr;

use cosmrs::{auth::BaseAccount, crypto::PublicKey, tx::Fee, Any, Coin, Denom};

use crate::helpers::EcdsaKeyIds;

use super::{
    address::get_account_id_from_public_key,
    proto::deployment::{
        deploymentmsg::{
            MsgCloseDeployment, MsgCreateDeployment, MsgDepositDeployment, MsgUpdateDeployment,
        },
        DeploymentID,
    },
    sdl::SdlV3,
    tx::create_tx,
};

pub async fn create_deployment_tx(
    sender_public_key: &PublicKey,
    sdl: &SdlV3,
    dseq: u64,
    deposit_uakt_amount: u64,
    account: &BaseAccount,
    ecdsa_key: &EcdsaKeyIds,
) -> Result<Vec<u8>, String> {
    let account_id = get_account_id_from_public_key(sender_public_key)?.to_string();
    // see https://github.com/akash-network/cloudmos/blob/8a8098b7e371e801dad3aad81ef92b8dfe387e4c/deploy-web/src/utils/deploymentData/v1beta3.ts#L230
    let msg = MsgCreateDeployment {
        id: Some(DeploymentID {
            owner: account_id.clone(),
            // see https://github.com/akash-network/cloudmos/blob/8a8098b7e371e801dad3aad81ef92b8dfe387e4c/deploy-web/src/utils/deploymentData/v1beta3.ts#L248C27-L248C27
            dseq, // obtained from /blocks/latest RPC call
        }),
        groups: sdl.groups(),
        version: sdl.manifest_version(),
        deposit: Some(
            Coin {
                amount: deposit_uakt_amount.into(),
                denom: Denom::from_str("uakt").unwrap(),
            }
            .into(),
        ),
        depositor: account_id,
    };

    let amount = Coin {
        amount: 20_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    let gas = 800_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    let tx_raw = create_tx(
        sender_public_key,
        Any::from_msg(&msg).unwrap(),
        fee,
        account.sequence,
        account.account_number,
        ecdsa_key,
    )
    .await?;

    Ok(tx_raw)
}

pub async fn deposit_deployment_tx(
    sender_public_key: &PublicKey,
    dseq: u64,
    uakt_amount: u64,
    account: &BaseAccount,
    ecdsa_key: &EcdsaKeyIds,
) -> Result<Vec<u8>, String> {
    let account_id = get_account_id_from_public_key(sender_public_key)?.to_string();
    let msg = MsgDepositDeployment {
        id: Some(DeploymentID {
            owner: account_id.clone(),
            // see https://github.com/akash-network/cloudmos/blob/8a8098b7e371e801dad3aad81ef92b8dfe387e4c/deploy-web/src/utils/deploymentData/v1beta3.ts#L248C27-L248C27
            dseq, // obtained from /blocks/latest RPC call
        }),
        amount: Some(
            Coin {
                amount: uakt_amount as u128,
                denom: Denom::from_str("uakt").unwrap(),
            }
            .into(),
        ),
        depositor: account_id,
    };

    let amount = Coin {
        amount: 20_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    let gas = 800_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    let tx_raw = create_tx(
        sender_public_key,
        Any::from_msg(&msg).unwrap(),
        fee,
        account.sequence,
        account.account_number,
        ecdsa_key,
    )
    .await?;

    Ok(tx_raw)
}

pub async fn update_deployment_sdl_tx(
    sender_public_key: &PublicKey,
    sdl: &SdlV3,
    dseq: u64,
    account: &BaseAccount,
    ecdsa_key: &EcdsaKeyIds,
) -> Result<Vec<u8>, String> {
    let account_id = get_account_id_from_public_key(sender_public_key)?.to_string();
    let msg = MsgUpdateDeployment {
        id: Some(DeploymentID {
            owner: account_id.clone(),
            // see https://github.com/akash-network/cloudmos/blob/8a8098b7e371e801dad3aad81ef92b8dfe387e4c/deploy-web/src/utils/deploymentData/v1beta3.ts#L248C27-L248C27
            dseq, // obtained from /blocks/latest RPC call
        }),

        version: sdl.manifest_version(),
    };

    let amount = Coin {
        amount: 20_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    let gas = 800_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    let tx_raw = create_tx(
        sender_public_key,
        Any::from_msg(&msg).unwrap(),
        fee,
        account.sequence,
        account.account_number,
        ecdsa_key,
    )
    .await?;

    Ok(tx_raw)
}

pub async fn close_deployment_tx(
    sender_public_key: &PublicKey,
    dseq: u64,
    account: &BaseAccount,
    ecdsa_key: &EcdsaKeyIds,
) -> Result<Vec<u8>, String> {
    let msg = MsgCloseDeployment {
        id: Some(DeploymentID {
            owner: get_account_id_from_public_key(sender_public_key)?.to_string(),
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
        sender_public_key,
        Any::from_msg(&msg).unwrap(),
        fee,
        account.sequence,
        account.account_number,
        ecdsa_key,
    )
    .await
}
