use candid::Principal;
use ic_cdk::api::call::call;
use ic_ledger_types::{GetBlocksArgs, QueryBlocksResponse};

use crate::api::ApiError;

pub struct LedgerService {
    ledger_canister_id: Principal,
}

impl LedgerService {
    pub fn default() -> Self {
        Self {
            ledger_canister_id: Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
        }
    }

    pub async fn query_blocks(&self, args: GetBlocksArgs) -> Result<QueryBlocksResponse, ApiError> {
        let (res,) = call(self.ledger_canister_id, "query_blocks", (args,))
            .await
            .map_err(|(code, e)| {
                ApiError::internal(&format!(
                    "failed to query blocks. Rejection code: {:?}, error: {}",
                    code, e
                ))
            })?;
        Ok(res)
    }

    pub async fn check_payment(
        &self,
        calling_principal: Principal,
        payment_block_heihgt: u64,
    ) -> Result<Option<()>, ApiError> {
        let args = GetBlocksArgs {
            start: payment_block_heihgt,
            length: 1,
        };

        let query_blocks_response = self.query_blocks(args).await?;

        if query_blocks_response.blocks.is_empty() {
            return Ok(None);
        }
        // TODO: check that the sender is the caller
        Ok(Some(()))
    }
}
