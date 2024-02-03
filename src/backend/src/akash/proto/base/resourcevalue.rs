/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/base/v1beta3/resourcevalue.proto#L9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceValue {
    #[prost(bytes, tag = "1")]
    pub val: ::prost::alloc::vec::Vec<u8>,
}
