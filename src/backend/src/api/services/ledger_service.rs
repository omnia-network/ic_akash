use crate::api::{
    init_config, ApiError, Asset, AssetClass, Config, ConfigMemory, GetExchangeRateRequest,
    GetExchangeRateResult,
};
use candid::Principal;
use ic_cdk::api::call::call_with_payment;
use ic_cdk::api::management_canister::http_request::{HttpMethod, TransformContext};
use ic_cdk::{api::call::call, print};
use ic_ledger_types::{
    AccountIdentifier, GetBlocksArgs, Operation, QueryBlocksResponse, Subaccount,
};
use utils::{get_time_nanos, make_http_request};

/// assume requests are at most 1kb
const REQUEST_SIZE: u128 = 1_000;
/// refuse responses that return more than 10kb
const MAX_RESPONSE_SIZE: u64 = 10_000;

pub struct LedgerService {
    config_memory: ConfigMemory,
    ledger_canister_id: Principal,
    xrc_id: Principal,
}

impl LedgerService {
    pub fn default() -> Self {
        Self {
            config_memory: init_config(),
            ledger_canister_id: Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
            xrc_id: Principal::from_text("uf6dk-hyaaa-aaaaq-qaaaq-cai").unwrap(),
        }
    }

    pub fn get_config(&self) -> Config {
        self.config_memory.get().clone()
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
    ) -> Result<Option<f64>, ApiError> {
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
            let paid_akt =
                (amount.e8s() / 100_000_000) as f64 * self.get_icp_2_akt_conversion_rate().await?;

            // the payment might still be a double spend,
            // therefore it is important to check that this 'payment_block_heihgt'
            // has not been used for a previous deployment
            // this is taken care of by the `users_service`
            return Ok(Some(paid_akt));
        }

        print(&format!("no transfer found"));
        Ok(None)
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

    pub async fn get_icp_2_akt_conversion_rate(&self) -> Result<f64, ApiError> {
        if self.get_config().is_mainnet() {
            let args = GetExchangeRateRequest {
                base_asset: Asset {
                    symbol: "AKT".to_string(),
                    class: AssetClass::Cryptocurrency,
                },
                quote_asset: Asset {
                    symbol: "ICP".to_string(),
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

            return Ok(
                exchange_rate.rate as f64 * 10_f64.powi(exchange_rate.metadata.decimals as i32)
            );
        }

        // used only when testing locally
        let icp_price = self.get_usd_exchange("ICP").await?;
        // as the AKT price is not available on the coinbase API, use a hardcoded value
        // let akt_price = self.get_usd_exchange("AKT").await;
        let akt_price = 5.0;

        Ok(icp_price / akt_price)
    }
}
