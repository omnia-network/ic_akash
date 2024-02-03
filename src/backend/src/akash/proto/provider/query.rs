/// QueryProviderRequest is request type for the Query/Provider RPC method.
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/provider/v1beta3/query.proto#L40
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProviderRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}

/// QueryProviderResponse is response type for the Query/Provider RPC method.
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/provider/v1beta3/query.proto#L45
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProviderResponse {
    #[prost(message, tag = "1")]
    pub provider: ::core::option::Option<super::provider::Provider>,
}
