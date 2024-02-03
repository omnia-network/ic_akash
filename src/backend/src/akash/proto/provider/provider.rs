/// Provider stores owner and host details.
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/provider/v1beta3/provider.proto#L98
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Provider {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub hostURI: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub attributes: ::prost::alloc::vec::Vec<super::super::base::attribute::Attribute>,
    #[prost(message, tag = "4")]
    pub info: ::core::option::Option<ProviderInfo>,
}

/// ProviderInfo.
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/provider/v1beta3/provider.proto#L22
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderInfo {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub website: ::prost::alloc::string::String,
}
