use candid::{CandidType, Deserialize};

use crate::api::{DateTime, LogEntry, LogLevel, LogsFilter};

const MICROS_PER_MS: u64 = 1_000;

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct LogsFilterRequest {
    pub before_timestamp_ms: Option<u64>,
    pub after_timestamp_ms: Option<u64>,
    pub level: Option<MappedLogLevel>,
    pub context_contains_any: Option<Vec<String>>,
    pub message_contains_any: Option<Vec<String>>,
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub enum MappedLogLevel {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct MappedLogEntry {
    pub date_time: String,
    pub level: MappedLogLevel,
    pub context: Option<String>,
    pub message: String,
}

impl From<LogsFilterRequest> for LogsFilter {
    fn from(value: LogsFilterRequest) -> Self {
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

impl From<LogsFilter> for LogsFilterRequest {
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

impl From<LogEntry> for MappedLogEntry {
    fn from(value: LogEntry) -> Self {
        Self {
            date_time: value.date_time.to_string(),
            level: value.level.into(),
            context: value.context,
            message: value.message,
        }
    }
}

impl From<MappedLogLevel> for LogLevel {
    fn from(value: MappedLogLevel) -> Self {
        match value {
            MappedLogLevel::Info => Self::Info,
            MappedLogLevel::Warn => Self::Warn,
            MappedLogLevel::Error => Self::Error,
        }
    }
}

impl From<LogLevel> for MappedLogLevel {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Info => Self::Info,
            LogLevel::Warn => Self::Warn,
            LogLevel::Error => Self::Error,
        }
    }
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct ListLogsResponse {
    pub logs: Vec<MappedLogEntry>,
}

pub fn map_logs_filter_request(request: LogsFilterRequest) -> LogsFilter {
    request.into()
}

pub fn map_list_logs_response(logs: Vec<LogEntry>) -> ListLogsResponse {
    ListLogsResponse {
        logs: logs.into_iter().map(MappedLogEntry::from).collect(),
    }
}
