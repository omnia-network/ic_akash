/// BidFilters defines flags for bid list filter.
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/market/v1beta4/bid.proto#L169
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidFilters {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub dseq: u64,
    #[prost(uint32, tag = "3")]
    pub gseq: u32,
    #[prost(uint32, tag = "4")]
    pub oseq: u32,
    #[prost(string, tag = "5")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub state: ::prost::alloc::string::String,
}

/// BidID stores owner and all other seq numbers
/// A successful bid becomes a Lease(ID).
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/market/v1beta4/bid.proto#L80
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidId {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub dseq: u64,
    #[prost(uint32, tag = "3")]
    pub gseq: u32,
    #[prost(uint32, tag = "4")]
    pub oseq: u32,
    #[prost(string, tag = "5")]
    pub provider: ::prost::alloc::string::String,
}

/// Bid stores BidID, state of bid and price
///
/// https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/market/v1beta4/bid.proto#L110
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bid {
    #[prost(message, optional, tag = "1")]
    pub bid_id: ::core::option::Option<BidId>,
    #[prost(enumeration = "State", tag = "2")]
    pub state: i32,
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<cosmrs::proto::cosmos::base::v1beta1::DecCoin>,
    #[prost(int64, tag = "4")]
    pub created_at: i64,
    #[prost(message, repeated, tag = "5")]
    pub resources_offered: ::prost::alloc::vec::Vec<ResourceOffer>,
}

/// ResourceOffer describes resources that provider is offering
/// for deployment
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/market/v1beta4/bid.proto#L14
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceOffer {
    #[prost(message, optional, tag = "1")]
    pub resources: ::core::option::Option<super::super::base::resources::Resources>,
    #[prost(uint32, tag = "2")]
    pub count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    BidStateInvalid = 0,
    BidOpen = 1,
    BidActive = 2,
    BidLost = 3,
    BidClosed = 4,
}
