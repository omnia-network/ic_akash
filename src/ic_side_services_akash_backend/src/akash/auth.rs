use cosmrs::{
    crypto::PublicKey,
    proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest, QueryAccountResponse},
};
use ic_cdk::print;
use prost::Message;

use super::address::get_account_id_from_public_key;

pub fn account_request(sender_public_key: &PublicKey) -> Result<String, String> {
    let query = QueryAccountRequest {
        address: get_account_id_from_public_key(sender_public_key)
            .unwrap()
            .to_string(),
    };

    Ok(hex::encode(&query.encode_to_vec()))
}

pub fn account_response(hex_data: String) {
    let res = QueryAccountResponse::decode(hex::decode(hex_data).unwrap().as_slice()).unwrap();
    print(format!("account_response: {:?}", res));

    let value = BaseAccount::decode(res.account.unwrap().value.as_slice()).unwrap();
    print(format!("account_response.value: {:?}", value));
}
