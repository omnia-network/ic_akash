use crate::api::{AkashService, ApiError, ApiResult};
use ic_cdk::update;

#[update]
async fn address() -> ApiResult<String> {
    AkashEndpoints::default().address().await.into()
}

#[update]
async fn balance() -> ApiResult<String> {
    AkashEndpoints::default().balance().await.into()
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

    pub async fn balance(&self) -> Result<String, ApiError> {
        self.akash_service
            .balance()
            .await
            .map_err(|e| ApiError::internal(&format!("failed to get balance: {}", e)))
    }
}
