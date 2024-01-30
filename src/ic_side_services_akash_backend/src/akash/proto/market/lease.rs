use cosmrs::proto::traits::Name;

use super::bid::BidId;

/// MsgCreateLease is sent to create a lease.
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/market/v1beta4/lease.proto#L121
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateLease {
    #[prost(message, tag = "1")]
    pub bid_id: ::core::option::Option<BidId>,
}

impl Name for MsgCreateLease {
    const NAME: &'static str = "MsgCreateLease";
    const PACKAGE: &'static str = "akash.market.v1beta4";
}
