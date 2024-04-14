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

#[update]
async fn get_5_akt_in_icp() -> ApiResult<f64> {
    LedgerEndpoints::default().get_5_akt_in_icp().await.into()
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
    pub async fn query_blocks(&self, args: GetBlocksArgs) -> Result<QueryBlocksResponse, ApiError> {
        self.ledger_service.query_blocks(args).await
    }

    pub async fn get_usd_exchange(&self, ticker: &str) -> Result<f64, ApiError> {
        self.ledger_service.get_usd_exchange(ticker).await
    }

    pub async fn get_5_akt_in_icp(&self) -> Result<f64, ApiError> {
        let icp_2_akt_conversion_rate = self.ledger_service.get_icp_2_akt_conversion_rate().await?;
        Ok(5.0 / icp_2_akt_conversion_rate)
    }
}
