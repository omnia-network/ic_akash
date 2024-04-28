use super::{Memory, LOGS_INDEX_MEMORY_ID, LOGS_MEMORY_ID, MEMORY_MANAGER};
use crate::api::LogEntry;
use ic_stable_structures::Log;

pub type LogMemory = Log<LogEntry, Memory, Memory>;

pub fn init_logs() -> LogMemory {
    // TODO: handle the error
    LogMemory::init(get_logs_index_memory(), get_logs_memory()).unwrap()
}

fn get_logs_index_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(LOGS_INDEX_MEMORY_ID))
}

fn get_logs_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(LOGS_MEMORY_ID))
}
