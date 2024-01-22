use cosmrs::proto::traits::Name;

/// MsgCreateCertificate defines an SDK message for creating certificate.
///
/// from https://github.com/akash-network/akash-api/blob/d60ce66786ce213bebd7d62aa4698f25de3bae12/proto/node/akash/cert/v1beta3/cert.proto#L83
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateCertificate {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(bytes, tag = "2")]
    pub cert: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
}

impl Name for MsgCreateCertificate {
    const NAME: &'static str = "MsgCreateCertificate";
    const PACKAGE: &'static str = "akash.cert.v1beta3";
}
