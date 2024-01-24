use cosmrs::crypto::PublicKey;
use prost::Message;

use crate::{
    address::get_account_id_from_public_key,
    proto::market::{bid::BidFilters, query::QueryBidsRequest},
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
