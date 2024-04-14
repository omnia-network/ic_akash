use cosmrs::proto::traits::Name;

use super::{groupspec::GroupSpec, DeploymentID};

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

/// MsgDepositDeployment deposits more funds into the deposit account
///
/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/deployment/v1beta3/deploymentmsg.proto#L48
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositDeployment {
    #[prost(message, tag = "1")]
    pub id: ::core::option::Option<DeploymentID>,
    #[prost(message, tag = "2")]
    pub amount: ::core::option::Option<cosmrs::proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub depositor: ::prost::alloc::string::String,
}

/// from https://github.com/akash-network/cloudmos/blob/d644be8430a57ebbf195afd2c460946b38e5a56f/deploy-web/src/utils/TransactionMessageData.ts#L10
impl Name for MsgDepositDeployment {
    const NAME: &'static str = "MsgDepositDeployment";
    const PACKAGE: &'static str = "akash.deployment.v1beta3";
}

/// MsgUpdateDeployment defines an SDK message for updating deployment
///
/// from https://github.com/akash-network/akash-api/blob/8b3ecebafedd45c27653f34cfe8917cbdcc7c970/proto/node/akash/deployment/v1beta3/deploymentmsg.proto#L75
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateDeployment {
    #[prost(message, tag = "1")]
    pub id: ::core::option::Option<DeploymentID>,
    #[prost(bytes, tag = "3")]
    pub version: ::prost::alloc::vec::Vec<u8>,
}

// from https://github.com/akash-network/cloudmos/blob/d644be8430a57ebbf195afd2c460946b38e5a56f/deploy-web/src/utils/TransactionMessageData.ts#L12
impl Name for MsgUpdateDeployment {
    const NAME: &'static str = "MsgUpdateDeployment";
    const PACKAGE: &'static str = "akash.deployment.v1beta3";
}

/// MsgCloseDeployment defines an SDK message for closing deployment.
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/deployment/v1beta3/deploymentmsg.proto#L94
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCloseDeployment {
    #[prost(message, tag = "1")]
    pub id: ::core::option::Option<DeploymentID>,
}

// from https://github.com/akash-network/cloudmos/blob/d644be8430a57ebbf195afd2c460946b38e5a56f/deploy-web/src/utils/TransactionMessageData.ts#L8
impl Name for MsgCloseDeployment {
    const NAME: &'static str = "MsgCloseDeployment";
    const PACKAGE: &'static str = "akash.deployment.v1beta3";
}
