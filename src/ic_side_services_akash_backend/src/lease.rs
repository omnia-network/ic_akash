use std::str::FromStr;

use cosmrs::{crypto::PublicKey, tx::Fee, Coin, Denom};
use prost_types::Any;

use crate::{
    proto::market::{bid::BidId, lease::MsgCreateLease},
    tx::create_tx,
};

pub async fn create_lease_tx(
    sender_public_key: &PublicKey,
    sequence_number: u64,
) -> Result<String, String> {
    let msg = MsgCreateLease {
        bid_id: Some(BidId {
            owner: String::from(""),
            dseq: 0,
            gseq: 0,
            oseq: 0,
            provider: String::from(""),
        }),
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
