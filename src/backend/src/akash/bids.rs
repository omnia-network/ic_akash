use cosmrs::crypto::PublicKey;
use prost::Message;

use crate::config::get_config;

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
    let config = get_config();

    let query = QueryBidsRequest {
        filters: Some(BidFilters {
            owner: get_account_id_from_public_key(sender_public_key)
                .unwrap()
                .to_string(),
            DSeq: dseq, // same as in the CreateDeployment transaction
            GSeq: 0,
            OSeq: 0,
            provider: "".to_string(),
            state: "".to_string(),
        }),
        pagination: None,
    };

    let abci_res = ic_tendermint_rpc::abci_query(
        config.tendermint_rpc_url(),
        Some(String::from("/akash.market.v1beta4.Query/Bids")),
        query.encode_to_vec(),
        None,
        false,
    )
    .await?;

    let res =
        QueryBidsResponse::decode(abci_res.response.value.as_slice()).map_err(|e| e.to_string())?;

    Ok(res.bids)
}
