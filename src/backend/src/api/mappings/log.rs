use crate::api::{LogsFilterRequest, LogsFilter, LogEntry, ListLogsResponse, DateTime};

const MICROS_PER_MS: u64 = 1_000;

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

pub fn map_logs_filter_request(request: LogsFilterRequest) -> LogsFilter {
    request.into()
}

pub fn map_list_logs_response(logs: Vec<LogEntry>) -> ListLogsResponse {
    ListLogsResponse {
        logs: logs.into_iter().map(LogEntry::from).collect(),
    }
}
