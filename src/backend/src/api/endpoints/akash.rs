use crate::api::AkashService;
use ic_cdk::update;

#[update]
async fn address() -> Result<String, String> {
    AkashEndpoints::default().address().await
}

#[update]
async fn balance() -> Result<String, String> {
    AkashEndpoints::default().balance().await
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

    pub async fn address(&self) -> Result<String, String> {
        self.akash_service.address().await
    }

    pub async fn balance(&self) -> Result<String, String> {
        self.akash_service.balance().await
    }
}
