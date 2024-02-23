use crate::api::{ApiError, ApiResult, LedgerService};
use ic_cdk::query;
use ic_ledger_types::{GetBlocksArgs, QueryBlocksResponse};

#[query(composite = true)]
async fn query_blocks(args: GetBlocksArgs) -> ApiResult<QueryBlocksResponse> {
    LedgerEndpoints::default().query_blocks(args).await.into()
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
        self.ledger_service
            .query_blocks(args)
            .await
            .map_err(|(code, e)| {
                ApiError::internal(&format!(
                    "failed to query blocks. Rejection code: {:?}, error: {}",
                    code, e
                ))
            })
    }
}
