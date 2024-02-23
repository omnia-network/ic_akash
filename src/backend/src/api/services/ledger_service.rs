use candid::Principal;
use ic_cdk::api::call::{call, CallResult};
use ic_ledger_types::{GetBlocksArgs, QueryBlocksResponse};

pub struct LedgerService {
    ledger_canister_id: Principal,
}

impl LedgerService {
    pub fn default() -> Self {
        Self {
            ledger_canister_id: Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
        }
    }

    pub async fn query_blocks(&self, args: GetBlocksArgs) -> CallResult<QueryBlocksResponse> {
        let (res,) = call(self.ledger_canister_id, "query_blocks", (args,)).await?;
        Ok(res)
    }
}
