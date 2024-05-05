use crate::api::{ApiError, ApiResult, LedgerService};
use candid::Nat;
use ic_cdk::{
    api::management_canister::http_request::{HttpResponse, TransformArgs},
    query,
};
use ic_ledger_types::{GetBlocksArgs, QueryBlocksResponse};

#[query(composite = true)]
async fn query_blocks(args: GetBlocksArgs) -> ApiResult<QueryBlocksResponse> {
    LedgerEndpoints::default().query_blocks(args).await.into()
}

#[query]
fn price_transform(raw: TransformArgs) -> HttpResponse {
    let status_ok = Nat::from(200u16);

    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        ..Default::default()
    };

    if res.status == status_ok {
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
}
