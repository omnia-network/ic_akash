use cosmrs::{
    auth::BaseAccount,
    crypto::PublicKey,
    proto::cosmos::auth::v1beta1::{
        BaseAccount as ProtoBaseAccount, QueryAccountRequest, QueryAccountResponse,
    },
};
use prost::Message;

use super::address::get_account_id_from_public_key;

pub async fn get_account(
    rpc_url: String,
    sender_public_key: &PublicKey,
) -> Result<BaseAccount, String> {
    let query = QueryAccountRequest {
        address: get_account_id_from_public_key(sender_public_key)?.to_string(),
    };

    let abci_res = ic_tendermint_rpc::abci_query(
        rpc_url,
        Some(String::from("/cosmos.auth.v1beta1.Query/Account")),
        query.encode_to_vec(),
        None,
        false,
    )
    .await?;

    let res = QueryAccountResponse::decode(abci_res.response.value.as_slice())
        .map_err(|e| e.to_string())?;

    let proto_account = ProtoBaseAccount::decode(res.account.unwrap().value.as_slice())
        .map_err(|e| e.to_string())?;

    BaseAccount::try_from(proto_account).map_err(|e| e.to_string())
}
