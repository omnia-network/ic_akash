/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/base/v1beta3/storage.proto#L11
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Storage {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, tag = "2")]
    pub quantity: ::core::option::Option<super::resourcevalue::ResourceValue>,
    #[prost(message, repeated, tag = "3")]
    pub Attributes: ::prost::alloc::vec::Vec<super::attribute::Attribute>,
}
