use crate::api::{ApiError, ApiResult, LedgerService};
use ic_cdk::{
    api::management_canister::http_request::{HttpResponse, TransformArgs},
    query, update,
};
use ic_ledger_types::{GetBlocksArgs, QueryBlocksResponse};

#[query(composite = true)]
async fn query_blocks(args: GetBlocksArgs) -> ApiResult<QueryBlocksResponse> {
    LedgerEndpoints::default().query_blocks(args).await.into()
}

#[update]
async fn get_icp_price() -> ApiResult<f64> {
    LedgerEndpoints::default()
        .get_usd_exchange("ICP")
        .await
        .into()
}

#[update]
async fn get_akt_price() -> ApiResult<f64> {
    LedgerEndpoints::default()
        .get_usd_exchange("AKT")
        .await
        .into()
}

#[query]
fn price_transform(raw: TransformArgs) -> HttpResponse {
    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        ..Default::default()
    };

    if res.status == 200 {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error from coinbase: err = {:?}", raw));
    }
    res
}

#[derive(Default)]
struct LedgerEndpoints {
    ledger_service: LedgerService,
}

impl LedgerEndpoints {
    async fn query_blocks(&self, args: GetBlocksArgs) -> Result<QueryBlocksResponse, ApiError> {
        self.ledger_service.query_blocks(args).await
    }

    async fn get_usd_exchange(&self, ticker: &str) -> Result<f64, ApiError> {
        self.ledger_service.get_usd_exchange(ticker).await
    }
}
