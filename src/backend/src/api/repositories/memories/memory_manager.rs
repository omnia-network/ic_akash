use std::cell::RefCell;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;

pub(super) type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub(super) const USERS_MEMORY_ID: MemoryId = MemoryId::new(0);
pub(super) const DEPLOYMENTS_MEMORY_ID: MemoryId = MemoryId::new(1);
pub(super) const LOGS_INDEX_MEMORY_ID: MemoryId = MemoryId::new(2);
pub(super) const LOGS_MEMORY_ID: MemoryId = MemoryId::new(3);
pub(super) const DEPLOYMENTS_COUNTER_MEMORY_ID: MemoryId = MemoryId::new(4);
