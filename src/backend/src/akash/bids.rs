use cosmrs::AccountId;
use prost::Message;

use super::proto::market::{
    bid::BidFilters,
    query::{QueryBidResponse, QueryBidsRequest, QueryBidsResponse},
};

pub async fn fetch_bids(
    rpc_url: String,
    account_id: &AccountId,
    dseq: u64,
) -> Result<Vec<QueryBidResponse>, String> {
    let query = QueryBidsRequest {
        filters: Some(BidFilters {
            owner: account_id.to_string(),
            dseq, // same as in the CreateDeployment transaction
            gseq: 0,
            oseq: 0,
            provider: "".to_string(),
            state: "".to_string(),
        }),
        pagination: None,
    };

    let abci_res = ic_tendermint_rpc::abci_query(
        rpc_url,
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
