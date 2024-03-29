/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/base/v1beta3/gpu.proto#L11
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GPU {
    #[prost(message, tag = "1")]
    pub units: ::core::option::Option<super::resourcevalue::ResourceValue>,
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<super::attribute::Attribute>,
}
