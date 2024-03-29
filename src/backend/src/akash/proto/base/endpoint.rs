/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/base/v1beta3/endpoint.proto#L9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    #[prost(enumeration = "Kind", tag = "1")]
    pub kind: i32,
    #[prost(uint32, tag = "2")]
    pub sequence_number: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Kind {
    SharedHttp = 0,
    RandomPort = 1,
    LeasedIp = 2,
}
