pub mod deploymentmsg;
pub mod groupspec;
pub mod resourceunit;

/// MsgCreateDeployment defines an SDK message for creating deployment.
///
/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/deployment/v1beta3/deployment.proto#L9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentID {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub dseq: u64,
}
