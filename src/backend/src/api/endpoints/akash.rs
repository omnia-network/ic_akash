use crate::api::{AkashService, ApiError, ApiResult};
use ic_cdk::update;

#[update]
async fn address() -> ApiResult<String> {
    AkashEndpoints::default().address().await.into()
}

#[update]
async fn balance() -> ApiResult<u64> {
    AkashEndpoints::default().balance().await.into()
}

#[update]
async fn check_tx(tx_hash_hex: String) -> ApiResult<()> {
    AkashEndpoints::default().check_tx(tx_hash_hex).await.into()
}

struct AkashEndpoints {
    akash_service: AkashService,
}

impl AkashEndpoints {
    pub fn default() -> Self {
        Self {
            akash_service: AkashService::default(),
        }
    }

    pub async fn address(&self) -> Result<String, ApiError> {
        self.akash_service
            .address()
            .await
            .map_err(|e| ApiError::internal(&format!("failed to get address: {}", e)))
    }

    pub async fn balance(&self) -> Result<u64, ApiError> {
        self.akash_service
            .balance()
            .await
            .map_err(|e| ApiError::internal(&format!("could not get balance: {}", e)))
    }

    pub async fn check_tx(&self, tx_hash_hex: String) -> Result<(), ApiError> {
        self.akash_service
            .check_tx(tx_hash_hex)
            .await
            .map_err(|e| ApiError::internal(&format!("failed to check tx: {}", e)))
    }
}
