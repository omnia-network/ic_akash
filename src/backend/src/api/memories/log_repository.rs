use super::{init_logs, LogMemory};
use crate::api::{ApiError, LogEntry, LogId};
use std::cell::RefCell;

pub struct LogRepository {}

impl Default for LogRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl LogRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_logs(&self) -> Vec<LogEntry> {
        STATE.with_borrow(|s| s.logs.iter().collect::<Vec<_>>())
    }

    pub fn append_log(&self, log_entry: LogEntry) -> Result<LogId, ApiError> {
        STATE
            .with_borrow_mut(|s| s.logs.append(&log_entry))
            .map_err(|e| ApiError::internal(&format!("Cannot write log: {:?}", e)))
    }
}

struct LogState {
    logs: LogMemory,
}

impl Default for LogState {
    fn default() -> Self {
        Self { logs: init_logs() }
    }
}

thread_local! {
    static STATE: RefCell<LogState> = RefCell::new(LogState::default());
}
