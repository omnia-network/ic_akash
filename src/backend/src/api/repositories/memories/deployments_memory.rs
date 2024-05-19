use ic_stable_structures::{BTreeMap, Cell};

use crate::api::repositories::memories::memory_manager::DEPLOYMENTS_COUNTER_MEMORY_ID;
use crate::api::{Deployment, DeploymentId};

use super::{Memory, DEPLOYMENTS_MEMORY_ID, MEMORY_MANAGER};

pub type DeploymentsMemory = BTreeMap<DeploymentId, Deployment, Memory>;

pub type DeploymentsCounterMemory = Cell<u64, Memory>;

pub fn init_deployments() -> DeploymentsMemory {
    DeploymentsMemory::init(get_deployments_memory())
}

fn get_deployments_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(DEPLOYMENTS_MEMORY_ID))
}

pub fn init_deployments_counter() -> DeploymentsCounterMemory {
    DeploymentsCounterMemory::init(get_deployments_counter_memory(), 0).unwrap()
}

fn get_deployments_counter_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(DEPLOYMENTS_COUNTER_MEMORY_ID))
}
