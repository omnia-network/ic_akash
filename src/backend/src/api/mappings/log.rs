use crate::api::{DateTime, LogEntry, LogLevel, LogsFilter};

use self::backend_api::ListLogsResponse;

const MICROS_PER_MS: u64 = 1_000;

impl From<backend_api::LogsFilterRequest> for LogsFilter {
    fn from(value: backend_api::LogsFilterRequest) -> Self {
        // unwrapping here should be ok, since this struct should be only used in query calls
        Self {
            before: value
                .before_timestamp_ms
                .map(|t| DateTime::from_timestamp_micros(t * MICROS_PER_MS).unwrap()),
            after: value
                .after_timestamp_ms
                .map(|t| DateTime::from_timestamp_micros(t * MICROS_PER_MS).unwrap()),
            level: value.level.map(|l| l.into()),
            context_contains_any: value.context_contains_any,
            message_contains_any: value.message_contains_any,
        }
    }
}

impl From<LogsFilter> for backend_api::LogsFilterRequest {
    fn from(value: LogsFilter) -> Self {
        Self {
            before_timestamp_ms: value.before.map(|t| t.timestamp_micros() / MICROS_PER_MS),
            after_timestamp_ms: value.after.map(|t| t.timestamp_micros() / MICROS_PER_MS),
            level: value.level.map(|l| l.into()),
            context_contains_any: value.context_contains_any,
            message_contains_any: value.message_contains_any,
        }
    }
}

impl From<LogEntry> for backend_api::LogEntry {
    fn from(value: LogEntry) -> Self {
        Self {
            date_time: value.date_time.to_string(),
            level: value.level.into(),
            context: value.context,
            message: value.message,
        }
    }
}

impl From<backend_api::LogLevel> for LogLevel {
    fn from(value: backend_api::LogLevel) -> Self {
        match value {
            backend_api::LogLevel::Info => Self::Info,
            backend_api::LogLevel::Warn => Self::Warn,
            backend_api::LogLevel::Error => Self::Error,
        }
    }
}

impl From<LogLevel> for backend_api::LogLevel {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Info => Self::Info,
            LogLevel::Warn => Self::Warn,
            LogLevel::Error => Self::Error,
        }
    }
}

pub fn map_logs_filter_request(request: backend_api::LogsFilterRequest) -> LogsFilter {
    request.into()
}

pub fn map_list_logs_response(logs: Vec<LogEntry>) -> ListLogsResponse {
    ListLogsResponse {
        logs: logs.into_iter().map(backend_api::LogEntry::from).collect(),
    }
}

pub mod backend_api {
    use candid::{CandidType, Deserialize};

    #[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
    pub struct LogsFilterRequest {
        pub before_timestamp_ms: Option<u64>,
        pub after_timestamp_ms: Option<u64>,
        pub level: Option<LogLevel>,
        pub context_contains_any: Option<Vec<String>>,
        pub message_contains_any: Option<Vec<String>>,
    }

    #[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
    pub enum LogLevel {
        #[serde(rename = "info")]
        Info,
        #[serde(rename = "warn")]
        Warn,
        #[serde(rename = "error")]
        Error,
    }

    #[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
    pub struct LogEntry {
        pub date_time: String,
        pub level: LogLevel,
        pub context: Option<String>,
        pub message: String,
    }

    #[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
    pub struct ListLogsResponse {
        pub logs: Vec<LogEntry>,
    }
}
