/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/deployment/v1beta3/resourceunit.proto#L11
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceUnit {
    #[prost(message, tag = "1")]
    pub resource: ::core::option::Option<super::super::base::resources::Resources>,
    #[prost(uint32, tag = "2")]
    pub count: u32,
    #[prost(message, tag = "3")]
    pub price: ::core::option::Option<cosmrs::proto::cosmos::base::v1beta1::DecCoin>,
}
