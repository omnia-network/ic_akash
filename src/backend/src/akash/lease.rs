use std::str::FromStr;

use cosmrs::{crypto::PublicKey, tx::Fee, Coin, Denom};
use prost_types::Any;

use super::{
    proto::market::{bid::BidId, lease::MsgCreateLease},
    tx::create_tx,
};

pub async fn create_lease_tx(
    sender_public_key: &PublicKey,
    sequence_number: u64,
    bid_id: BidId,
    account_number: u64,
) -> Result<Vec<u8>, String> {
    let msg = MsgCreateLease {
        bid_id: Some(bid_id),
    };

    let amount = Coin {
        amount: 50_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    let gas = 2_000_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    create_tx(
        &sender_public_key,
        Any::from_msg(&msg).unwrap(),
        fee,
        sequence_number,
        account_number,
    )
    .await
}
