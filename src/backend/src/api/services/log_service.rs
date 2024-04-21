use crate::api::{ApiError, ListLogsResponse, LogsFilterRequest, DateTime, LogEntry, LogLevel, map_list_logs_response, map_logs_filter_request, LogRepository, LogRepositoryImpl};

use utils::get_date_time;

pub trait LogService {
    fn list_logs(&self, filter: LogsFilterRequest) -> ListLogsResponse;

    fn append_log(
        &self,
        level: LogLevel,
        message: String,
        context: Option<String>,
    ) -> Result<(), ApiError>;

    fn log_info(&self, message: String, context: Option<String>) -> Result<(), ApiError>;

    fn log_warn(&self, message: String, context: Option<String>) -> Result<(), ApiError>;

    fn log_error(&self, message: String, context: Option<String>) -> Result<(), ApiError>;
}

pub struct LogServiceImpl<T: LogRepository> {
    log_repository: T,
}

impl Default for LogServiceImpl<LogRepositoryImpl> {
    fn default() -> Self {
        Self::new(LogRepositoryImpl::default())
    }
}

impl<T: LogRepository> LogService for LogServiceImpl<T> {
    fn list_logs(&self, request: LogsFilterRequest) -> ListLogsResponse {
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

    fn append_log(
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

    fn log_info(&self, message: String, context: Option<String>) -> Result<(), ApiError> {
        self.append_log(LogLevel::Info, message, context)
    }

    fn log_warn(&self, message: String, context: Option<String>) -> Result<(), ApiError> {
        self.append_log(LogLevel::Warn, message, context)
    }

    fn log_error(&self, message: String, context: Option<String>) -> Result<(), ApiError> {
        self.append_log(LogLevel::Error, message, context)
    }
}

impl<T: LogRepository> LogServiceImpl<T> {
    fn new(log_repository: T) -> Self {
        Self { log_repository }
    }
}