//! from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/base/v1beta3/attribute.proto

/// Attribute represents key value pair
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}

/// SignedBy represents validation accounts that tenant expects signatures for provider attributes
/// AllOf has precedence i.e. if there is at least one entry AnyOf is ignored regardless to how many
/// entries there
/// this behaviour to be discussed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedBy {
    #[prost(string, repeated, tag = "1")]
    pub all_of: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub any_of: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

/// PlacementRequirements
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlacementRequirements {
    /// SignedBy list of keys that tenants expect to have signatures from
    #[prost(message, tag = "1")]
    pub signed_by: ::core::option::Option<SignedBy>,
    /// Attribute list of attributes tenant expects from the provider
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
}
