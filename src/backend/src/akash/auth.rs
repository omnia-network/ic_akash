use cosmrs::{
    crypto::PublicKey,
    proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest, QueryAccountResponse},
};
use prost::Message;

use crate::config::get_config;

use super::address::get_account_id_from_public_key;

pub async fn get_account(sender_public_key: &PublicKey) -> Result<BaseAccount, String> {
    let config = get_config();

    let query = QueryAccountRequest {
        address: get_account_id_from_public_key(sender_public_key)
            .unwrap()
            .to_string(),
    };

    let abci_res = ic_tendermint_rpc::abci_query(
        config.tendermint_rpc_url(),
        Some(String::from("/cosmos.auth.v1beta1.Query/Account")),
        query.encode_to_vec(),
        None,
        false,
    )
    .await?;

    let res = QueryAccountResponse::decode(abci_res.response.value.as_slice())
        .map_err(|e| e.to_string())?;

    BaseAccount::decode(res.account.unwrap().value.as_slice()).map_err(|e| e.to_string())
}
