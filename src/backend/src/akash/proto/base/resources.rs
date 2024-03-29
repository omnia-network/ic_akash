/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/base/v1beta3/resources.proto#L15
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resources {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(message, tag = "2")]
    pub cpu: ::core::option::Option<super::cpu::CPU>,
    #[prost(message, tag = "3")]
    pub memory: ::core::option::Option<super::memory::Memory>,
    #[prost(message, repeated, tag = "4")]
    pub storage: ::prost::alloc::vec::Vec<super::storage::Storage>,
    #[prost(message, tag = "5")]
    pub gpu: ::core::option::Option<super::gpu::GPU>,
    #[prost(message, repeated, tag = "6")]
    pub endpoints: ::prost::alloc::vec::Vec<super::endpoint::Endpoint>,
}
