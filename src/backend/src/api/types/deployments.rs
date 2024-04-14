use super::{TimestampNs, UserId};
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use utils::{get_time_nanos, Uuid};

pub type DeploymentId = Uuid;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Deployment {
    sdl: String,
    user_id: UserId,
    state_history: Vec<(TimestampNs, DeploymentState)>,
    akt_price: f64,
    icp_price: f64,
}

impl Deployment {
    pub fn new(sdl: String, user_id: UserId, akt_price: f64, icp_price: f64) -> Self {
        Self {
            sdl,
            user_id,
            state_history: vec![(get_time_nanos(), DeploymentState::Initialized)],
            akt_price,
            icp_price,
        }
    }

    pub fn sdl(&self) -> String {
        self.sdl.clone()
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn state(&self) -> DeploymentState {
        self.state_history
            .last()
            .expect("must have at least one state")
            .1
            .clone()
    }

    pub fn icp_price(&self) -> f64 {
        self.icp_price
    }

    pub fn user_owns_deployment(&self, user_id: &UserId) -> bool {
        self.user_id == *user_id
    }

    pub fn update_state(&mut self, update: DeploymentState) {
        self.state_history.push((get_time_nanos(), update));
    }

    pub fn get_history(&self) -> Vec<(u64, DeploymentState)> {
        self.state_history.clone()
    }

    pub fn get_akash_info(&self) -> Option<u64> {
        self.state_history
            .iter()
            .filter_map(|(_, update)| update.get_akash_info())
            .collect::<Vec<u64>>()
            .first()
            .cloned()
    }
}

impl Storable for Deployment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// Deployment update sent to the client via IC WebSocket
#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum DeploymentState {
    Initialized,
    DeploymentCreated {
        tx_hash: String,
        dseq: u64,
        manifest_sorted_json: String,
    },
    LeaseCreated {
        tx_hash: String,
        provider_url: String,
    },
    Active,
    Closed,
    FailedOnCanister {
        reason: String,
    },
    FailedOnClient {
        reason: String,
    },
}

impl DeploymentState {
    pub fn get_akash_info(&self) -> Option<u64> {
        match self {
            DeploymentState::DeploymentCreated { dseq, .. } => Some(*dseq),
            _ => None,
        }
    }
}

impl Storable for DeploymentState {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct DeploymentParams {
    pub name: Option<String>,
    pub image: Option<String>,
    pub env_vars: Option<Vec<(String, String)>>,
    pub ports: Vec<(u32, Option<u32>)>,
    pub cpu: ResourceSize,
    pub memory: ResourceSize,
    pub storage: ResourceSize,
    pub volume_mount: Option<String>,
    pub command: Option<Vec<String>>,
}

impl DeploymentParams {
    /// Create a new deployment based on a Docker image
    pub fn builder() -> DeploymentParamsBuilder {
        DeploymentParamsBuilder {
            inner: DeploymentParams {
                name: None,
                image: None,
                env_vars: None,
                ports: Vec::new(),
                cpu: ResourceSize::Small,
                memory: ResourceSize::Small,
                storage: ResourceSize::Small,
                volume_mount: None,
                command: None,
            },
        }
    }
}

#[derive(Debug)]
pub struct DeploymentParamsBuilder {
    inner: DeploymentParams,
}

impl DeploymentParamsBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.inner.name = Some(name);
        self
    }

    pub fn image(mut self, image: String) -> Self {
        self.inner.image = Some(image);
        self
    }

    pub fn env_var(mut self, env_var: (String, String)) -> Self {
        if let Some(ref mut env_vars) = self.inner.env_vars {
            env_vars.push(env_var);
        } else {
            self.inner.env_vars = Some(vec![env_var]);
        }
        self
    }

    pub fn port(mut self, port: (u32, Option<u32>)) -> Self {
        self.inner.ports.push(port);
        self
    }

    pub fn cpu(mut self, cpu: ResourceSize) -> Self {
        self.inner.cpu = cpu;
        self
    }

    pub fn memory(mut self, memory: ResourceSize) -> Self {
        self.inner.memory = memory;
        self
    }

    pub fn storage(mut self, storage: ResourceSize) -> Self {
        self.inner.storage = storage;
        self
    }

    pub fn volume_mount(mut self, volume_mount: String) -> Self {
        self.inner.volume_mount = Some(volume_mount);
        self
    }

    pub fn command(mut self, command: Vec<String>) -> Self {
        self.inner.command = Some(command);
        self
    }

    pub fn build(self) -> DeploymentParams {
        self.inner
    }
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum ResourceSize {
    Small,
    Medium,
    Large,
}
