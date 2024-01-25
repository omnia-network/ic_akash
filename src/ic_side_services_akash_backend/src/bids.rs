use cosmrs::crypto::PublicKey;
use ic_cdk::print;
use prost::Message;

use crate::{
    address::get_account_id_from_public_key,
    proto::market::{
        bid::BidFilters,
        query::{QueryBidsRequest, QueryBidsResponse},
    },
};

pub fn bids_request(sender_public_key: &PublicKey, dseq: u64) -> Result<String, String> {
    let query = QueryBidsRequest {
        filters: Some(BidFilters {
            owner: get_account_id_from_public_key(sender_public_key)
                .unwrap()
                .to_string(),
            dseq, // same as in the CreateDeployment transaction
            gseq: 0,
            oseq: 0,
            provider: "".to_string(),
            state: "".to_string(),
        }),
        pagination: None,
    };

    Ok(hex::encode(&query.encode_to_vec()))
}

pub fn bids_response(hex_data: String) {
    let res = QueryBidsResponse::decode(hex::decode(hex_data).unwrap().as_slice()).unwrap();
    print(format!("account_response: {:?}", res));
}
