use super::{init_logs, LogMemory};
use crate::api::{ApiError, LogEntry, LogId};
use std::cell::RefCell;

pub trait LogRepository {
    fn get_logs(&self) -> Vec<LogEntry>;

    fn append_log(&self, log_entry: LogEntry) -> Result<LogId, ApiError>;
}

pub struct LogRepositoryImpl {}

impl Default for LogRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl LogRepository for LogRepositoryImpl {
    fn get_logs(&self) -> Vec<LogEntry> {
        STATE.with_borrow(|s| s.logs.iter().collect::<Vec<_>>())
    }

    fn append_log(&self, log_entry: LogEntry) -> Result<LogId, ApiError> {
        STATE
            .with_borrow_mut(|s| s.logs.append(&log_entry))
            .map_err(|e| ApiError::internal(&format!("Cannot write log: {:?}", e)))
    }
}

impl LogRepositoryImpl {
    pub fn new() -> Self {
        Self {}
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
