use std::str::FromStr;

use cosmrs::{
    bank::MsgSend,
    crypto::PublicKey,
    tx::{Fee, Msg},
    AccountId, Coin, Denom,
};

use super::{address::get_account_id_from_public_key, tx::create_tx};

pub async fn create_send_tx(
    sender_public_key: &PublicKey,
    to_address: String,
    amount: u64,
    sequence_number: u64,
    account_number: u64,
) -> Result<Vec<u8>, String> {
    let recipient_account_id = AccountId::from_str(to_address.as_str()).unwrap();

    let amount = Coin {
        amount: amount.into(),
        denom: Denom::from_str("uakt").unwrap(),
    };

    let msg_send = MsgSend {
        from_address: get_account_id_from_public_key(&sender_public_key).unwrap(),
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
        sequence_number,
        account_number,
    )
    .await
}
