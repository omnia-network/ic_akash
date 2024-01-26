/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/deployment/v1beta3/groupspec.proto#L11
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSpec {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, tag = "2")]
    pub requirements: ::core::option::Option<super::super::base::attribute::PlacementRequirements>,
    #[prost(message, repeated, tag = "3")]
    pub resources: ::prost::alloc::vec::Vec<super::resourceunit::ResourceUnit>,
}
