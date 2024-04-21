use crate::api::{ApiError, ApiResult, LogServiceImpl, ListLogsResponse, LogsFilterRequest, AccessControlService, LogService, LogRepositoryImpl};
use candid::Principal;
use ic_cdk::*;

#[query]
fn list_logs(request: LogsFilterRequest) -> ApiResult<ListLogsResponse> {
    let calling_principal = caller();
    
    LogController::default()
        .list_logs(calling_principal, request)
        .into()
}

struct LogController {
    access_control_service: AccessControlService,
    log_service: LogServiceImpl<LogRepositoryImpl>,
}

impl LogController {
    fn default() -> Self {
        Self::new(
            AccessControlService::default(),
            LogServiceImpl::default(),
        )
    }

    fn new(access_control_service: AccessControlService, log_service: LogServiceImpl<LogRepositoryImpl>) -> Self {
        Self {
            access_control_service,
            log_service,
        }
    }

    fn list_logs(
        &self,
        calling_principal: Principal,
        request: LogsFilterRequest,
    ) -> Result<ListLogsResponse, ApiError> {
        self.access_control_service
            .assert_principal_is_admin(&calling_principal)?;

        let logs = self.log_service.list_logs(request);

        Ok(logs)
    }
}