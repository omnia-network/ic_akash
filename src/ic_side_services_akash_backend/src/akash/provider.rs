use ic_cdk::print;
use prost::Message;

use super::proto::provider::query::{QueryProviderRequest, QueryProviderResponse};

pub fn provider_request() -> Result<String, String> {
    let query = QueryProviderRequest {
        owner: String::from("provider"),
    };

    Ok(hex::encode(&query.encode_to_vec()))
}

pub fn provider_response(hex_data: String) {
    let res = QueryProviderResponse::decode(hex::decode(hex_data).unwrap().as_slice()).unwrap();
    print(format!("provider_response: {:?}", res));
}
