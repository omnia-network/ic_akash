/// QueryBidsRequest is request type for the Query/Bids RPC method.
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/market/v1beta4/query.proto#L84
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidsRequest {
    #[prost(message, tag = "1")]
    pub filters: ::core::option::Option<super::bid::BidFilters>,
    #[prost(message, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmrs::proto::cosmos::base::query::v1beta1::PageRequest>,
}

/// QueryBidsResponse is response type for the Query/Bids RPC method.
///
/// from https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/market/v1beta4/query.proto#L93
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidsResponse {
    #[prost(message, repeated, tag = "1")]
    pub bids: ::prost::alloc::vec::Vec<QueryBidResponse>,
    #[prost(message, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmrs::proto::cosmos::base::query::v1beta1::PageRequest>,
}

/// QueryBidResponse is response type for the Query/Bid RPC method
///
/// https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/market/v1beta4/query.proto#L110
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidResponse {
    #[prost(message, optional, tag = "1")]
    pub bid: ::core::option::Option<super::bid::Bid>,
    #[prost(message, optional, tag = "2")]
    pub escrow_account: ::core::option::Option<super::super::escrow::types::Account>,
}
