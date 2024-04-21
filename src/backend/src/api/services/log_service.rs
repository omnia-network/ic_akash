use crate::api::{
    backend_api::{ListLogsResponse, LogsFilterRequest},
    map_list_logs_response, map_logs_filter_request, ApiError, DateTime, LogEntry, LogLevel,
    LogRepository,
};

use utils::get_date_time;

pub struct LogService {
    log_repository: LogRepository,
}

impl Default for LogService {
    fn default() -> Self {
        Self::new(LogRepository::default())
    }
}

impl LogService {
    pub fn list_logs(&self, request: LogsFilterRequest) -> ListLogsResponse {
        let filter = map_logs_filter_request(request);

        let logs = self
            .log_repository
            .get_logs()
            .iter()
            .filter(|l| filter.matches(l))
            .cloned()
            .collect::<Vec<_>>();

        map_list_logs_response(logs)
    }

    pub fn append_log(
        &self,
        level: LogLevel,
        message: String,
        context: Option<String>,
    ) -> Result<(), ApiError> {
        let date_time = get_date_time().map_err(|e| ApiError::internal(&e))?;

        let log_entry = LogEntry {
            date_time: DateTime::new(date_time)?,
            level,
            context,
            message,
        };

        self.log_repository.append_log(log_entry).map(|_| ())
    }

    pub fn log_info(&self, message: String, context: Option<String>) -> Result<(), ApiError> {
        self.append_log(LogLevel::Info, message, context)
    }

    pub fn log_warn(&self, message: String, context: Option<String>) -> Result<(), ApiError> {
        self.append_log(LogLevel::Warn, message, context)
    }

    pub fn log_error(&self, message: String, context: Option<String>) -> Result<(), ApiError> {
        self.append_log(LogLevel::Error, message, context)
    }
}

impl LogService {
    fn new(log_repository: LogRepository) -> Self {
        Self { log_repository }
    }
}
