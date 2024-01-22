use std::collections::HashMap;

use cosmrs::proto::cosmos::base::v1beta1::DecCoin;
use serde::{Deserialize, Serialize};

use crate::proto::{
    base::{
        attribute::{
            Attribute as ProtobufAttribute, PlacementRequirements, SignedBy as ProtobufSignedBy,
        },
        cpu::CPU,
        endpoint::Endpoint,
        gpu::GPU,
        memory::Memory,
        resources::Resources,
        resourcevalue::ResourceValue,
        storage::Storage,
    },
    deployment::{groupspec::GroupSpec, resourceunit::ResourceUnit},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ServiceV2 {
    pub image: String,
    pub command: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub expose: Vec<ExposeV2>,
    pub dependencies: Option<Vec<DependencyV2>>,
    pub params: Option<ServiceParamsV2>,
}

impl ServiceV2 {
    pub fn service_resource_endpoints_v3(
        &self,
        endpoint_sequence_numbers: EndpointSequenceNumbers,
    ) -> Vec<DeploymentGroupResourceEndpointV3> {
        self.expose
            .iter()
            .flat_map(|expose| {
                expose
                    .to
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter(|to| to.global())
                    .flat_map(|to| {
                        let kind = match expose_should_be_ingress(
                            expose.external_port(),
                            expose.proto(),
                            to.clone(),
                        ) {
                            true => EndpointKind::SharedHttp,
                            false => EndpointKind::RandomPort,
                        };

                        let default_ep = DeploymentGroupResourceEndpointV3 {
                            kind,
                            sequence_number: 0,
                        };

                        match to.ip.is_some() {
                            true => {
                                vec![
                                    default_ep,
                                    DeploymentGroupResourceEndpointV3 {
                                        kind: EndpointKind::LeasedIp,
                                        sequence_number: *endpoint_sequence_numbers
                                            .get(to.ip.as_ref().unwrap())
                                            .unwrap_or(&0),
                                    },
                                ]
                            }
                            false => {
                                vec![default_ep]
                            }
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn expose_should_be_ingress(external_port: u32, proto: String, to: ExposeToV2) -> bool {
    to.global() && proto == "TCP" && external_port == 80
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ExposeV2 {
    pub port: u32,
    pub r#as: Option<u32>,
    pub proto: Option<String>,
    pub to: Option<Vec<ExposeToV2>>,
    pub accept: Option<AcceptV2>,
    pub http_options: Option<HttpOptionsV2>,
}

impl ExposeV2 {
    pub fn external_port(&self) -> u32 {
        self.r#as.unwrap_or(self.port)
    }

    pub fn proto(&self) -> String {
        self.proto.clone().unwrap_or("TCP".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ExposeToV2 {
    pub service: Option<String>,
    pub global: Option<bool>,
    pub http_options: Option<HttpOptionsV2>,
    pub ip: Option<String>,
}

impl ExposeToV2 {
    pub fn global(&self) -> bool {
        self.global.unwrap_or(false)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct HttpOptionsV2 {
    pub max_body_size: u32,
    pub read_timeout: u32,
    pub send_timeout: u32,
    pub next_tries: u32,
    pub next_timeout: u32,
    pub next_cases: Vec<String>,
}

// doesn't work
// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// pub struct AcceptV2 {
//     pub items: Option<Vec<String>>,
// }
pub type AcceptV2 = Vec<String>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DependencyV2 {
    pub service: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ServiceParamsV2 {
    pub storage: Option<HashMap<String, ServiceStorageParamsV2>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ServiceStorageParamsV2 {
    pub name: String,
    pub mount: String,
    #[serde(rename = "readOnly")]
    pub read_only: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProfilesV3 {
    pub compute: HashMap<String, ProfileComputeV3>,
    pub placement: HashMap<String, ProfilePlacementV2>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProfileComputeV3 {
    pub resources: ComputeResourcesV3,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ComputeResourcesV3 {
    pub cpu: ResourceCpuV2,
    pub memory: ResourceMemoryV2,
    pub storage: Vec<ResourceStorageV2>,
    pub gpu: Option<ResourceGpuV3>,
    pub id: Option<u32>,
}

impl Into<Resources> for ComputeResourcesV3 {
    fn into(self) -> Resources {
        Resources {
            ID: self.id.unwrap(),
            CPU: Some(self.cpu.into()),
            memory: Some(self.memory.into()),
            storage: self.storage.into_iter().map(|s| s.into()).collect(),
            GPU: self.gpu.map(|g| g.into()),
            Endpoints: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ResourceCpuV2 {
    pub units: String,
    pub attributes: Option<CpuAttributesV2>,
}

impl Into<CPU> for ResourceCpuV2 {
    fn into(self) -> CPU {
        CPU {
            units: Some(ResourceValue {
                val: self.units.as_bytes().to_vec(),
            }),
            Attributes: self
                .attributes
                .unwrap_or_default()
                .into_iter()
                .map(|(k, v)| ProtobufAttribute { key: k, value: v })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ResourceStorageV2 {
    pub name: Option<String>,
    pub size: String,
    pub attributes: Option<StorageAttributesV2>,
}

impl Into<Storage> for ResourceStorageV2 {
    fn into(self) -> Storage {
        Storage {
            name: self.name.unwrap(),
            quantity: Some(ResourceValue {
                val: self.size.as_bytes().to_vec(),
            }),
            Attributes: self
                .attributes
                .unwrap_or_default()
                .into_iter()
                .map(|(k, v)| ProtobufAttribute { key: k, value: v })
                .collect(),
        }
    }
}

pub type StorageAttributesV2 = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ResourceMemoryV2 {
    pub size: String,
    pub attributes: Option<HashMap<String, String>>,
}

impl Into<Memory> for ResourceMemoryV2 {
    fn into(self) -> Memory {
        Memory {
            quantity: Some(ResourceValue {
                val: self.size.as_bytes().to_vec(),
            }),
            Attributes: self
                .attributes
                .unwrap_or_default()
                .into_iter()
                .map(|(k, v)| ProtobufAttribute { key: k, value: v })
                .collect(),
        }
    }
}

pub type CpuAttributesV2 = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ResourceGpuV3 {
    pub units: String,
    pub attributes: Option<GpuAttributesV3>,
}

impl Into<GPU> for ResourceGpuV3 {
    fn into(self) -> GPU {
        GPU {
            units: Some(ResourceValue {
                val: self.units.as_bytes().to_vec(),
            }),
            // TODO: figure out how to map to proto
            // Attributes: self.attributes.unwrap_or_default().vendor.into_iter().map(|(k, v)| ProtobufAttribute { key: k, value: v }).collect(),
            Attributes: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GpuAttributesV3 {
    pub vendor: HashMap<String, Vec<GpuModelV3>>,
}

impl Default for GpuAttributesV3 {
    fn default() -> Self {
        Self {
            vendor: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GpuModelV3 {
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProfilePlacementV2 {
    pub attributes: Option<Attributes>,
    #[serde(rename = "signedBy")]
    pub signed_by: Option<SignedBy>,
    pub pricing: PlacementPricingV2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

impl Into<ProtobufAttribute> for Attribute {
    fn into(self) -> ProtobufAttribute {
        ProtobufAttribute {
            key: self.key,
            value: self.value,
        }
    }
}

pub type Attributes = Vec<Attribute>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SignedBy {
    #[serde(rename = "allOf")]
    pub all_of: Vec<String>,
    #[serde(rename = "anyOf")]
    pub any_of: Vec<String>,
}

impl Default for SignedBy {
    fn default() -> Self {
        Self {
            all_of: vec![],
            any_of: vec![],
        }
    }
}

impl Into<ProtobufSignedBy> for SignedBy {
    fn into(self) -> ProtobufSignedBy {
        ProtobufSignedBy {
            all_of: self.all_of,
            any_of: self.any_of,
        }
    }
}

pub type PlacementPricingV2 = HashMap<String, CoinV2>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CoinV2 {
    pub denom: String,
    pub value: Option<u32>,
    pub amount: u32,
}

pub type DeploymentV2 = HashMap<String, ServiceDeploymentV2>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ServiceDeploymentV2 {
    pub profile: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EndpointV2 {
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SdlV3 {
    pub version: String,
    pub services: HashMap<String, ServiceV2>,
    pub profiles: ProfilesV3,
    pub deployment: HashMap<String, DeploymentV2>,
    pub endpoints: Option<HashMap<String, EndpointV2>>,
}

impl SdlV3 {
    pub fn try_from_str(sdl: &str) -> Result<SdlV3, String> {
        serde_yaml::from_str(sdl).map_err(|e| e.to_string())
    }

    pub fn groups(&self) -> Vec<GroupSpec> {
        let mut groups: HashMap<String, Group> = HashMap::new();

        for (service_name, service) in self.services.iter() {
            for (placement_name, svc_depl) in self.deployment.get(service_name).unwrap() {
                let compute = self.profiles.compute.get(&svc_depl.profile).unwrap();
                let infra = self.profiles.placement.get(placement_name).unwrap();
                let pricing = infra.pricing.get(&svc_depl.profile).unwrap();
                let price = DecCoin {
                    denom: pricing.denom.clone(),
                    amount: pricing.amount.to_string(),
                };

                let group = groups.entry(placement_name.to_string()).or_insert_with(|| {
                    let mut attributes = infra.attributes.clone().unwrap_or(vec![]);

                    attributes.sort_by(|a, b| a.key.cmp(&b.key));

                    Group {
                        dgroup: DeploymentGroupV3 {
                            name: placement_name.clone(),
                            resources: vec![],
                            requirements: DeploymentGroupRequirementsV3 {
                                attributes,
                                signed_by: infra.signed_by.clone().unwrap_or_default(),
                            },
                        },
                        bound_computes: HashMap::new(),
                    }
                });

                if group.bound_computes.get(placement_name).is_none() {
                    group
                        .bound_computes
                        .insert(service_name.clone(), HashMap::new());
                }

                let location = *(group
                    .bound_computes
                    .get_mut(placement_name)
                    .unwrap()
                    .entry(svc_depl.profile.clone())
                    .or_insert_with(|| {
                        let mut res = compute.resources.clone();

                        res.id = Some(group.dgroup.resources.len() as u32 + 1);

                        group.dgroup.resources.push(DeploymentGroupResourceV3 {
                            resource: res,
                            price: price.amount.parse().unwrap(),
                            count: svc_depl.count,
                            endpoints: vec![],
                        });

                        (group.dgroup.resources.len() - 1) as u32
                    })) as usize;

                group.dgroup.resources[location].count += svc_depl.count;
                group.dgroup.resources[location].endpoints.append(
                    service
                        .service_resource_endpoints_v3(self.compute_endpoint_sequence_numbers())
                        .as_mut(),
                );
                // TODO: sort?
            }
        }

        let mut names = groups.keys().collect::<Vec<_>>();
        names.sort();

        names
            .iter()
            .map(|&name| {
                let dgroup = groups.get(name).unwrap().dgroup.clone();

                dgroup.into()
            })
            .collect()
    }

    fn compute_endpoint_sequence_numbers(&self) -> EndpointSequenceNumbers {
        let mut map = EndpointSequenceNumbers::new();
        self.services
            .values()
            .flat_map(|service| {
                service.expose.iter().flat_map(|expose| {
                    expose
                        .to
                        .as_ref()
                        .unwrap_or(&vec![])
                        .iter()
                        .filter(|to| to.global() && to.ip.is_some())
                        .enumerate()
                        // TODO: sort?
                        .map(|(index, to)| (to.ip.clone().unwrap(), index as u32 + 1))
                        .collect::<Vec<_>>()
                })
            })
            .for_each(|(ip, index)| {
                map.insert(ip, index);
            });

        map
    }
}

struct Group {
    pub dgroup: DeploymentGroupV3,
    pub bound_computes: HashMap<String, HashMap<String, u32>>,
}

type EndpointSequenceNumbers = HashMap<String, u32>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DeploymentGroupV3 {
    pub name: String,
    pub resources: Vec<DeploymentGroupResourceV3>,
    pub requirements: DeploymentGroupRequirementsV3,
}

impl Into<GroupSpec> for DeploymentGroupV3 {
    fn into(self) -> GroupSpec {
        GroupSpec {
            name: self.name,
            resources: self.resources.iter().map(|r| r.clone().into()).collect(),
            requirements: Some(self.requirements.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DeploymentGroupResourceV3 {
    pub resource: ComputeResourcesV3,
    pub price: u64,
    pub count: u32,
    pub endpoints: Vec<DeploymentGroupResourceEndpointV3>,
}

impl Into<ResourceUnit> for DeploymentGroupResourceV3 {
    fn into(self) -> ResourceUnit {
        ResourceUnit {
            resource: Some(Resources {
                Endpoints: self.endpoints.into_iter().map(|e| e.into()).collect(),
                ..self.resource.into()
            }),
            price: Some(DecCoin {
                denom: "uakt".to_string(),
                amount: self.price.to_string(),
            }),
            count: self.count,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DeploymentGroupResourceEndpointV3 {
    pub kind: EndpointKind,
    pub sequence_number: u32,
}

impl Into<Endpoint> for DeploymentGroupResourceEndpointV3 {
    fn into(self) -> Endpoint {
        Endpoint {
            kind: self.kind as i32,
            SequenceNumber: self.sequence_number,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum EndpointKind {
    SharedHttp = 0,
    RandomPort = 1,
    LeasedIp = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DeploymentGroupRequirementsV3 {
    pub attributes: Attributes,
    #[serde(rename = "signedBy")]
    pub signed_by: SignedBy,
}

impl Into<PlacementRequirements> for DeploymentGroupRequirementsV3 {
    fn into(self) -> PlacementRequirements {
        PlacementRequirements {
            attributes: self
                .attributes
                .iter()
                .map(|attr| attr.clone().into())
                .collect(),
            signed_by: Some(self.signed_by.into()),
        }
    }
}
