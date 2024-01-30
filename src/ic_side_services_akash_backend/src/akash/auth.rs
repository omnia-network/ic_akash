use cosmrs::{
    crypto::PublicKey,
    proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest, QueryAccountResponse},
};
use prost::Message;

use super::address::get_account_id_from_public_key;

pub async fn get_account(sender_public_key: &PublicKey) -> Result<BaseAccount, String> {
    let query = QueryAccountRequest {
        address: get_account_id_from_public_key(sender_public_key)
            .unwrap()
            .to_string(),
    };

    // abci_query

    let res = QueryAccountResponse::decode(vec![].as_slice()).map_err(|e| e.to_string())?;

    BaseAccount::decode(res.account.unwrap().value.as_slice()).map_err(|e| e.to_string())
}
