use cosmrs::crypto::PublicKey;
use prost::Message;

use super::{
    address::get_account_id_from_public_key,
    proto::market::{
        bid::BidFilters,
        query::{QueryBidResponse, QueryBidsRequest, QueryBidsResponse},
    },
};

pub async fn fetch_bids(
    sender_public_key: &PublicKey,
    dseq: u64,
) -> Result<Vec<QueryBidResponse>, String> {
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

    // abci_query

    let res = QueryBidsResponse::decode(vec![].as_slice()).map_err(|e| e.to_string())?;

    Ok(res.bids)
}
