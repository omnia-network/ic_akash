use ic_stable_structures::BTreeMap;

use crate::api::{Deployment, DeploymentId};

use super::{Memory, DEPLOYMENTS_MEMORY_ID, MEMORY_MANAGER};

pub type DeploymentsMemory = BTreeMap<DeploymentId, Deployment, Memory>;

pub fn init_deployments() -> DeploymentsMemory {
    DeploymentsMemory::init(get_deployments_memory())
}

fn get_deployments_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(DEPLOYMENTS_MEMORY_ID))
}
