mod sizes;

use std::{cmp::Ordering, collections::HashMap};

use cosmrs::proto::cosmos::base::v1beta1::DecCoin;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::hash::sha256;

use super::proto::{
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
    deployment::{groupspec::GroupSpec, resourceunit::ResourceUnit as ProtobufResourceUnit},
};

use sizes::{convert_cpu_resource_string, convert_resource_string};

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
                        let kind = match expose.should_be_ingress(to.clone()) {
                            true => EndpointKind::SharedHttp,
                            false => EndpointKind::RandomPort,
                        };

                        let default_ep = DeploymentGroupResourceEndpointV3 {
                            kind: match kind {
                                EndpointKind::SharedHttp => None,
                                _ => Some(kind),
                            },
                            sequence_number: 0,
                        };

                        match to.ip.is_some() {
                            true => {
                                vec![
                                    default_ep,
                                    DeploymentGroupResourceEndpointV3 {
                                        kind: Some(EndpointKind::LeasedIp),
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

    pub fn should_be_ingress(&self, to: ExposeToV2) -> bool {
        to.global() && self.proto() == "TCP" && self.external_port() == 80
    }

    pub fn http_options(&self) -> HttpOptionsV2 {
        self.http_options.clone().unwrap_or_default()
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

    pub fn service(&self) -> String {
        self.service.clone().unwrap_or("".to_string())
    }

    pub fn ip(&self) -> String {
        self.ip.clone().unwrap_or("".to_string())
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

impl Default for HttpOptionsV2 {
    fn default() -> Self {
        Self {
            max_body_size: 1048576,
            read_timeout: 60_000,
            send_timeout: 60_000,
            next_tries: 3,
            next_timeout: 0,
            next_cases: vec!["error".to_string(), "timeout".to_string()],
        }
    }
}

impl Into<ServiceExposeHttpOptionsV3> for HttpOptionsV2 {
    fn into(self) -> ServiceExposeHttpOptionsV3 {
        ServiceExposeHttpOptionsV3 {
            max_body_size: self.max_body_size,
            read_timeout: self.read_timeout,
            send_timeout: self.send_timeout,
            next_tries: self.next_tries,
            next_timeout: self.next_timeout,
            next_cases: self.next_cases,
        }
    }
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

impl Into<ManifestServiceParamsV3> for ServiceParamsV2 {
    fn into(self) -> ManifestServiceParamsV3 {
        ManifestServiceParamsV3 {
            storage: self
                .storage
                .clone()
                .unwrap_or_default()
                .keys()
                .map(|name| ServiceStorageParamsV2 {
                    name: name.clone(),
                    mount: self
                        .storage
                        .as_ref()
                        .expect("Storage must be defined")
                        .get(name)
                        .unwrap()
                        .mount
                        .clone(),
                    read_only: self
                        .storage
                        .as_ref()
                        .expect("Storage must be defined")
                        .get(name)
                        .unwrap()
                        .read_only,
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ServiceStorageParamsV2 {
    pub mount: String,
    pub name: String,
    #[serde(rename = "readOnly", default)]
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
                val: convert_cpu_resource_string(&self.units)
                    .unwrap()
                    .to_string()
                    .into_bytes(),
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

impl ResourceStorageV2 {
    pub fn name(&self) -> String {
        self.name.clone().unwrap_or("default".to_string())
    }
}

impl Into<Storage> for ResourceStorageV2 {
    fn into(self) -> Storage {
        Storage {
            name: self.name(),
            quantity: Some(ResourceValue {
                val: convert_resource_string(&self.size)
                    .unwrap()
                    .to_string()
                    .into_bytes(),
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
                val: convert_resource_string(&self.size)
                    .unwrap()
                    .to_string()
                    .into_bytes(),
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

impl ResourceGpuV3 {
    pub fn units(&self) -> String {
        if self.units.is_empty() {
            "0".to_string()
        } else {
            self.units.clone()
        }
    }
}

impl Into<GPU> for ResourceGpuV3 {
    fn into(self) -> GPU {
        GPU {
            units: Some(ResourceValue {
                val: self.units().into_bytes(),
            }),
            Attributes: match self.attributes {
                Some(attributes) => Into::<Attributes>::into(attributes)
                    .into_iter()
                    .map(|attr| attr.into())
                    .collect(),
                None => vec![],
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GpuAttributesV3 {
    pub vendor: HashMap<String, Option<Vec<GpuModelV3>>>,
}

impl Default for GpuAttributesV3 {
    fn default() -> Self {
        Self {
            vendor: HashMap::new(),
        }
    }
}

impl Into<Attributes> for GpuAttributesV3 {
    fn into(self) -> Attributes {
        self.vendor
            .into_iter()
            .flat_map(|(vendor, models)| match models {
                Some(models) => models
                    .iter()
                    .map(|model| Attribute {
                        key: format!("vendor/{}/model/{}", vendor, model.model),
                        value: "true".to_string(),
                    })
                    .collect(),
                None => vec![Attribute {
                    key: format!("vendor/{}/model/*", vendor),
                    value: "true".to_string(),
                }],
            })
            .collect()
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
        let mut services = self
            .services
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();
        services.sort_by_key(|(k, _)| k.clone());

        for (service_name, service) in services.iter() {
            for (placement_name, svc_depl) in self.deployment.get(service_name).unwrap() {
                let compute = self.profiles.compute.get(&svc_depl.profile).unwrap();
                let infra = self.profiles.placement.get(placement_name).unwrap();
                let pricing = infra.pricing.get(&svc_depl.profile).unwrap();

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

                match group
                    .bound_computes
                    .entry(placement_name.clone())
                    .or_insert(HashMap::new())
                    .get(&svc_depl.profile)
                {
                    None => {
                        let mut res = compute.resources.clone();

                        res.id = Some(group.dgroup.resources.len() as u32 + 1);

                        group.dgroup.resources.push(DeploymentGroupResourceV3 {
                            resource: res,
                            price: pricing.amount.try_into().unwrap(),
                            count: svc_depl.count,
                            endpoints: vec![],
                        });

                        group
                            .bound_computes
                            .get_mut(placement_name)
                            .unwrap()
                            .insert(
                                svc_depl.profile.clone(),
                                group.dgroup.resources.len() as u32 - 1,
                            );
                    }
                    Some(&location) => {
                        let location = location as usize;
                        group.dgroup.resources[location].count += svc_depl.count;
                        group.dgroup.resources[location].endpoints.append(
                            service
                                .service_resource_endpoints_v3(
                                    self.compute_endpoint_sequence_numbers(),
                                )
                                .as_mut(),
                        );
                        group.dgroup.resources[location].endpoints.sort();
                    }
                };
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
                    let mut expose_entries = expose
                        .to
                        .as_ref()
                        .unwrap_or(&vec![])
                        .iter()
                        .filter(|to| to.global() && to.ip.is_some())
                        .map(|to| to.ip.clone())
                        .collect::<Vec<_>>();

                    expose_entries.sort();

                    expose_entries
                        .iter()
                        .enumerate()
                        .map(|(index, ip)| (ip.clone().unwrap(), index as u32 + 1))
                        .collect::<Vec<_>>()
                })
            })
            .for_each(|(ip, index)| {
                map.insert(ip, index);
            });

        map
    }

    fn placements(&self) -> HashMap<String, ProfilePlacementV2> {
        self.profiles.placement.clone()
    }

    fn deployments_by_placement(&self, placement: String) -> Vec<(String, DeploymentV2)> {
        self.deployment
            .iter()
            .filter(|(_, deployment)| deployment.contains_key(&placement))
            .map(|(name, deployment)| (name.clone(), deployment.clone()))
            .collect()
    }

    fn service_resources_beta3(
        &self,
        id: u32,
        profile: &ProfileComputeV3,
        service: &ServiceV2,
    ) -> ResourceUnits {
        ResourceUnits {
            id,
            cpu: service_resource_cpu(&profile.resources.cpu),
            memory: service_resource_memory(&profile.resources.memory),
            storage: service_resource_storage(&profile.resources.storage),
            endpoints: service
                .service_resource_endpoints_v3(self.compute_endpoint_sequence_numbers()),
            gpu: service_resource_gpu(&profile.resources.gpu),
        }
    }

    fn manifest_expose_v3(&self, service: &ServiceV2) -> Vec<ServiceExposeV3> {
        let mut expose_vec = service
            .expose
            .iter()
            .flat_map(|expose| {
                expose
                    .to
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|to| ServiceExposeV3 {
                        port: expose.port,
                        external_port: expose.r#as.unwrap_or(0),
                        proto: expose.proto(),
                        service: to.service(),
                        global: to.global(),
                        hosts: expose.accept.clone(),
                        http_options: expose.http_options().into(),
                        ip: to.ip(),
                        endpoint_sequence_number: self
                            .compute_endpoint_sequence_numbers()
                            .get(&to.ip())
                            .unwrap_or(&0)
                            .to_owned(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        expose_vec.sort_by(|a, b| {
            if a.service != b.service {
                return a.service.cmp(&b.service);
            }
            if a.port != b.port {
                return a.port.cmp(&b.port);
            }
            if a.proto != b.proto {
                return a.proto.cmp(&b.proto);
            }
            if a.global != b.global {
                return if a.global {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }

            Ordering::Equal
        });

        expose_vec
    }

    fn manifest_service_v3(&self, id: u32, placement: String, name: String) -> ManifestServiceV3 {
        let service = self.services.get(&name).unwrap();
        let deployment = self.deployment.get(&name).unwrap();
        let svc_deployment = deployment.get(&placement).unwrap();
        let profile = self.profiles.compute.get(&svc_deployment.profile).unwrap();

        ManifestServiceV3 {
            name,
            image: service.image.to_owned(),
            command: service.command.to_owned(),
            args: service.args.to_owned(),
            env: service.env.to_owned(),
            resources: self.service_resources_beta3(id, profile, service),
            count: svc_deployment.count,
            expose: self.manifest_expose_v3(service),
            params: service.params.to_owned().map(|params| params.into()),
        }
    }

    pub fn manifest_sorted_json(&self) -> String {
        let manifest = self.manifest();
        serde_json::to_string(&manifest)
            .unwrap()
            // TODO: can we use the `size` field instead?
            .replace("\"quantity\":{\"val\"", "\"size\":{\"val\"")
    }

    pub fn manifest(&self) -> ManifestV3 {
        let groups = self.groups();

        self.placements()
            .keys()
            .enumerate()
            .map(|(p_idx, name)| {
                let mut services = self.deployments_by_placement(name.clone());
                services.sort_by_key(|(svc_name, _)| svc_name.clone());

                GroupV3 {
                    name: name.clone(),
                    services: services
                        .iter()
                        .enumerate()
                        .map(|(s_idx, (service, _))| {
                            self.manifest_service_v3(
                                groups[p_idx].resources[s_idx].resource.as_ref().unwrap().ID,
                                name.clone(),
                                service.clone(),
                            )
                        })
                        .collect(),
                }
            })
            .collect()
    }

    pub fn manifest_version(&self) -> Vec<u8> {
        sha256(self.manifest_sorted_json().as_bytes()).to_vec()
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

impl Into<ProtobufResourceUnit> for DeploymentGroupResourceV3 {
    fn into(self) -> ProtobufResourceUnit {
        ProtobufResourceUnit {
            resource: Some(Resources {
                Endpoints: self.endpoints.into_iter().map(|e| e.into()).collect(),
                ..self.resource.into()
            }),
            price: Some(DecCoin {
                denom: "uakt".to_string(),
                amount: format!("{:0<23}", self.price).to_string(),
            }),
            count: self.count,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct DeploymentGroupResourceEndpointV3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<EndpointKind>,
    pub sequence_number: u32,
}

impl Into<Endpoint> for DeploymentGroupResourceEndpointV3 {
    fn into(self) -> Endpoint {
        Endpoint {
            kind: self.kind.unwrap_or(EndpointKind::SharedHttp) as i32, // TODO: is this the right default value?
            SequenceNumber: self.sequence_number,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[repr(u32)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ManifestServiceV3 {
    pub args: Option<Vec<String>>,
    pub command: Option<Vec<String>>,
    pub count: u32,
    pub env: Option<Vec<String>>,
    pub expose: Vec<ServiceExposeV3>,
    pub image: String,
    pub name: String,
    pub params: Option<ManifestServiceParamsV3>,
    pub resources: ResourceUnits,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ResourceUnits {
    pub cpu: GenericResource,
    pub endpoints: Vec<DeploymentGroupResourceEndpointV3>,
    pub gpu: GenericResource,
    pub id: u32,
    pub memory: GenericResource,
    pub storage: Vec<GenericResource>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ServiceExposeV3 {
    #[serde(rename = "endpointSequenceNumber")]
    pub endpoint_sequence_number: u32,
    #[serde(rename = "externalPort")]
    pub external_port: u32,
    pub global: bool,
    pub hosts: Option<AcceptV2>,
    #[serde(rename = "httpOptions")]
    pub http_options: ServiceExposeHttpOptionsV3,
    pub ip: String,
    pub port: u32,
    pub proto: String,
    pub service: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ServiceExposeHttpOptionsV3 {
    #[serde(rename = "maxBodySize")]
    pub max_body_size: u32,
    #[serde(rename = "nextCases")]
    pub next_cases: Vec<String>,
    #[serde(rename = "nextTimeout")]
    pub next_timeout: u32,
    #[serde(rename = "nextTries")]
    pub next_tries: u32,
    #[serde(rename = "readTimeout")]
    pub read_timeout: u32,
    #[serde(rename = "sendTimeout")]
    pub send_timeout: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ManifestServiceParamsV3 {
    pub storage: Vec<ServiceStorageParamsV2>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GroupV3 {
    pub name: String,
    pub services: Vec<ManifestServiceV3>,
}

pub type ManifestV3 = Vec<GroupV3>;

fn service_resource_attributes(attributes: &Option<HashMap<String, String>>) -> Option<Attributes> {
    attributes.as_ref().map(|attrs| {
        let mut attrs = attrs
            .into_iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();
        attrs.sort_by_key(|(k, _)| k.clone());
        attrs
            .iter()
            .cloned()
            .map(|(k, v)| Attribute {
                key: k.clone(),
                value: v.clone(),
            })
            .collect()
    })
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GenericResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<GenericResourceUnits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<GenericResourceUnits>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GenericResourceUnits {
    pub val: String,
}

fn resource_unit(val: &str) -> GenericResourceUnits {
    GenericResourceUnits {
        val: convert_resource_string(val).unwrap().to_string(),
    }
}

fn service_resource_cpu(resource: &ResourceCpuV2) -> GenericResource {
    GenericResource {
        units: Some(GenericResourceUnits {
            val: convert_cpu_resource_string(&resource.units)
                .unwrap()
                .to_string(),
        }),
        quantity: None,
        name: None,
        attributes: service_resource_attributes(&resource.attributes),
    }
}

fn service_resource_memory(resource: &ResourceMemoryV2) -> GenericResource {
    GenericResource {
        units: None,
        quantity: Some(resource_unit(&resource.size)),
        name: None,
        attributes: service_resource_attributes(&resource.attributes),
    }
}

fn service_resource_storage(resource: &Vec<ResourceStorageV2>) -> Vec<GenericResource> {
    resource
        .iter()
        .map(|resource| GenericResource {
            units: None,
            quantity: Some(resource_unit(&resource.size)),
            name: Some(resource.name()),
            attributes: service_resource_attributes(&resource.attributes),
        })
        .collect()
}

fn service_resource_gpu(resource: &Option<ResourceGpuV3>) -> GenericResource {
    GenericResource {
        units: Some(GenericResourceUnits {
            val: resource
                .clone()
                .map(|r| r.units())
                .unwrap_or("0".to_string()),
        }),
        quantity: None,
        name: None,
        attributes: resource
            .clone()
            .map(|r| r.attributes.map(|a| a.into()))
            .flatten(),
    }
}
