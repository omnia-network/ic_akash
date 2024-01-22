use cosmrs::proto::traits::Name;

use super::{deployment::DeploymentID, groupspec::GroupSpec};

/// MsgCreateDeployment defines an SDK message for creating deployment.
///
/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/deployment/v1beta3/deploymentmsg.proto#L14
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDeployment {
    #[prost(message, tag = "1")]
    pub id: ::core::option::Option<DeploymentID>,
    #[prost(message, repeated, tag = "2")]
    pub groups: ::prost::alloc::vec::Vec<GroupSpec>,
    #[prost(bytes, tag = "3")]
    pub version: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, tag = "4")]
    pub deposit: ::core::option::Option<cosmrs::proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "5")]
    pub depositor: ::prost::alloc::string::String,
}

// from https://github.com/akash-network/cloudmos/blob/d644be8430a57ebbf195afd2c460946b38e5a56f/deploy-web/src/utils/TransactionMessageData.ts#L9
impl Name for MsgCreateDeployment {
    const NAME: &'static str = "MsgCreateDeployment";
    const PACKAGE: &'static str = "akash.deployment.v1beta3";
}
