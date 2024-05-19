use std::str::FromStr;

use cosmrs::{auth::BaseAccount, crypto::PublicKey, tx::Fee, Coin, Denom};
use prost_types::Any;

use crate::helpers::EcdsaKeyIds;

use super::{
    proto::market::{bid::BidId, lease::MsgCreateLease},
    tx::create_tx,
};

pub async fn create_lease_tx(
    sender_public_key: &PublicKey,
    bid_id: BidId,
    account: &BaseAccount,
    ecdsa_key: &EcdsaKeyIds,
    chain_id: &str,
) -> Result<Vec<u8>, String> {
    let msg = MsgCreateLease {
        bid_id: Some(bid_id),
    };

    let amount = Coin {
        amount: 65_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    let gas = 2_500_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    create_tx(
        sender_public_key,
        Any::from_msg(&msg).unwrap(),
        fee,
        account.sequence,
        account.account_number,
        ecdsa_key,
        chain_id,
    )
    .await
}
