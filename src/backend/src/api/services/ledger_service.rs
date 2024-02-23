use candid::Principal;
use ic_cdk::{api::call::call, print};
use ic_ledger_types::{
    AccountIdentifier, GetBlocksArgs, Operation, QueryBlocksResponse, Subaccount,
};

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
            print(&format!("no blocks found"));
            return Ok(None);
        }
        if let Some(Operation::Transfer {
            from,
            to,
            amount,
            fee: _fee,
        }) = query_blocks_response.blocks[0].transaction.operation
        {
            let caller_account_id =
                AccountIdentifier::new(&calling_principal, &Subaccount([0; 32]));
            let orchestrator_account_id =
                AccountIdentifier::new(&ic_cdk::api::id(), &Subaccount([0; 32]));

            if from != caller_account_id {
                return Err(ApiError::not_found(
                    "caller is not the sender of the payment",
                ));
            }
            if to != orchestrator_account_id {
                return Err(ApiError::not_found(
                    "orchestrator is not the recipient of the payment",
                ));
            }
            if amount.e8s() < 500_000_000 {
                return Err(ApiError::not_found("payment amount is less than 5 ICPs"));
            }

            // TODO: store payment_block_height so that it cannot be reused for another deployment
            print(&format!("payment found"));
            return Ok(Some(()));
        }
        print(&format!("no transfer found"));
        Ok(None)
    }
}
