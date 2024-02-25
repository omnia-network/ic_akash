use crate::api::{ApiError, ApiResult, LedgerService};
use ic_cdk::{
    api::management_canister::http_request::{
        HttpMethod, HttpResponse, TransformArgs, TransformContext,
    },
    query, update,
};
use ic_ledger_types::{GetBlocksArgs, QueryBlocksResponse};
use utils::{get_time_nanos, make_http_request};

/// assume requests are at most 1kb
const REQUEST_SIZE: u128 = 1_000;
/// refuse responses that return more than 10kb
const MAX_RESPONSE_SIZE: u64 = 10_000;

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

struct LedgerEndpoints {
    ledger_service: LedgerService,
}

impl Default for LedgerEndpoints {
    fn default() -> Self {
        Self {
            ledger_service: LedgerService::default(),
        }
    }
}

impl LedgerEndpoints {
    pub async fn query_blocks(&self, args: GetBlocksArgs) -> Result<QueryBlocksResponse, ApiError> {
        self.ledger_service.query_blocks(args).await
    }

    pub async fn get_usd_exchange(&self, ticker: &str) -> Result<f64, ApiError> {
        let current_timestamp_s = get_time_nanos() / 1_000_000_000;
        let host = "api.pro.coinbase.com";
        let url = format!(
            "https://{}/products/{}-USD/candles?start={}&end={}",
            host,
            ticker,
            // price info is updated every 60 seconds
            // by requesting prices in the last 300 seconds, it is guaranteed that the response contains at least one price
            current_timestamp_s - 300,
            current_timestamp_s
        );

        let response = make_http_request(
            url.to_string(),
            HttpMethod::GET,
            None,
            vec![],
            Some(TransformContext::from_name(
                "price_transform".to_string(),
                vec![],
            )),
            REQUEST_SIZE,
            MAX_RESPONSE_SIZE,
        )
        .await
        .map_err(|e| ApiError::internal(&format!("failed to get {} price: {}", ticker, e)))?;

        // the response body will looks like this:
        // ("[[1682978460,5.714,5.718,5.714,5.714,243.5678], ...]")
        // which can be formatted as this
        //  [
        //     [
        //         1682978460, <-- start/timestamp
        //         5.714, <-- low
        //         5.718, <-- high
        //         5.714, <-- open
        //         5.714, <-- close
        //         243.5678 <-- volume
        //     ],
        //     ...
        //  ]
        let string_body =
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");
        let parsed_body: Vec<Vec<f64>> = serde_json::from_str(&string_body).map_err(|e| {
            ApiError::internal(&format!(
                "failed to parse {} price: {:?}",
                ticker,
                e.to_string()
            ))
        })?;

        if parsed_body.len() == 0 {
            return Err(ApiError::internal("API did not return any prices"));
        }
        // within the latest price range (index 0 of outer vec), return the lowest price (index 1 of inner vec)
        Ok(parsed_body[0][1])
    }
}
