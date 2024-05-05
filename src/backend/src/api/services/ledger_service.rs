use crate::api::{log_info, ApiError};
use candid::Principal;
use ic_cdk::api::call::{call, call_with_payment};
use ic_ledger_types::{
    AccountIdentifier, GetBlocksArgs, Operation, QueryBlocksResponse, Subaccount, Tokens,
    MAINNET_LEDGER_CANISTER_ID,
};
use ic_xrc_types::{Asset, AssetClass, GetExchangeRateRequest, GetExchangeRateResult};

pub struct LedgerService {
    ledger_canister_id: Principal,
    xrc_id: Principal,
}

impl Default for LedgerService {
    fn default() -> Self {
        Self {
            ledger_canister_id: MAINNET_LEDGER_CANISTER_ID,
            xrc_id: Principal::from_text("uf6dk-hyaaa-aaaaq-qaaaq-cai").unwrap(),
        }
    }
}

impl LedgerService {
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
        payment_block_height: u64,
    ) -> Result<f64, ApiError> {
        let args = GetBlocksArgs {
            start: payment_block_height,
            length: 1,
        };

        let query_blocks_response = self.query_blocks(args).await?;

        if query_blocks_response.blocks.is_empty() {
            return Err(ApiError::internal(&format!(
                "No blocks found at height: {}",
                payment_block_height,
            )));
        }

        let operation = &query_blocks_response.blocks[0].transaction.operation;

        log_info!(
            format!(
                "[check_payment]: Height: {}, Operation: {:?}",
                payment_block_height, operation,
            ),
            "ledger_service"
        );

        let Some(Operation::Transfer {
            from, to, amount, ..
        }) = operation
        else {
            return Err(ApiError::internal(&format!(
                "No Transfer operation found in block at height: {}",
                payment_block_height,
            )));
        };

        let caller_account_id = AccountIdentifier::new(&calling_principal, &Subaccount([0; 32]));
        let orchestrator_account_id =
            AccountIdentifier::new(&ic_cdk::api::id(), &Subaccount([0; 32]));

        if from != &caller_account_id {
            return Err(ApiError::not_found(
                "caller is not the sender of the payment",
            ));
        }
        if to != &orchestrator_account_id {
            return Err(ApiError::not_found(
                "orchestrator is not the recipient of the payment",
            ));
        }
        let paid_akt = (amount.e8s() / Tokens::SUBDIVIDABLE_BY) as f64
            * self.get_icp_2_akt_conversion_rate().await?;

        // the payment might still be a double spend,
        // therefore it is important to check that this 'payment_block_height'
        // has not been used for a previous deployment
        // this is taken care of by the `users_service`
        Ok(paid_akt)
    }

    pub async fn get_icp_2_akt_conversion_rate(&self) -> Result<f64, ApiError> {
        let args = GetExchangeRateRequest {
            base_asset: Asset {
                symbol: "ICP".to_string(),
                class: AssetClass::Cryptocurrency,
            },
            quote_asset: Asset {
                symbol: "AKT".to_string(),
                class: AssetClass::Cryptocurrency,
            },
            timestamp: None,
        };

        let (res,): (GetExchangeRateResult,) =
            call_with_payment(self.xrc_id, "get_exchange_rate", (args,), 10_000_000_000)
                .await
                .map_err(|(code, e)| {
                    ApiError::internal(&format!(
                        "failed to get exchange rate. Rejection code: {:?}, error: {}",
                        code, e
                    ))
                })?;

        let exchange_rate =
            res.map_err(|e| ApiError::internal(&format!("exchange rate error: {:?}", e)))?;

        log_info!(
            format!("exchange rate result: {:?}", exchange_rate),
            "ledger_service"
        );

        Ok(exchange_rate.rate as f64 / 10_f64.powi(exchange_rate.metadata.decimals as i32))
    }
}
