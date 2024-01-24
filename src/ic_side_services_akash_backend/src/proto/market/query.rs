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
