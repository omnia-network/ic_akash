use cosmrs::{
    auth::BaseAccount,
    bank::MsgSend,
    crypto::PublicKey,
    proto::cosmos::{
        bank::v1beta1::{QueryBalanceRequest, QueryBalanceResponse},
        base::v1beta1::Coin as CoinProto,
    },
    tx::{Fee, Msg},
    AccountId, Coin, Denom,
};
use prost::Message;
use std::str::FromStr;

use crate::helpers::EcdsaKeyIds;

use super::{address::get_account_id_from_public_key, tx::create_tx};

pub async fn create_send_tx(
    sender_public_key: &PublicKey,
    recipient_account_id: AccountId,
    amount: u64,
    account: &BaseAccount,
    ecdsa_key: &EcdsaKeyIds,
) -> Result<Vec<u8>, String> {
    let amount = Coin {
        amount: amount.into(),
        denom: Denom::from_str("uakt").unwrap(),
    };

    let msg_send = MsgSend {
        from_address: get_account_id_from_public_key(&sender_public_key)?,
        to_address: recipient_account_id,
        amount: vec![amount.clone()],
    };

    let gas = 100_000u64;
    let fee = Fee::from_amount_and_gas(
        Coin {
            amount: 3_000u128,
            denom: Denom::from_str("uakt").unwrap(),
        },
        gas,
    );

    create_tx(
        &sender_public_key,
        msg_send.to_any().unwrap(),
        fee,
        account.sequence,
        account.account_number,
        ecdsa_key,
    )
    .await
}

pub async fn get_balance(rpc_url: String, public_key: &PublicKey) -> Result<CoinProto, String> {
    let query = QueryBalanceRequest {
        address: get_account_id_from_public_key(public_key)?.to_string(),

        denom: String::from("uakt"),
    };

    let abci_res = ic_tendermint_rpc::abci_query(
        rpc_url,
        Some(String::from("/cosmos.bank.v1beta1.Query/Balance")),
        query.encode_to_vec(),
        None,
        false,
    )
    .await?;

    let res = QueryBalanceResponse::decode(abci_res.response.value.as_slice())
        .map_err(|e| e.to_string())?;

    let balance = res.balance.ok_or(String::from("Address not found"))?;
    Ok(balance)
}
