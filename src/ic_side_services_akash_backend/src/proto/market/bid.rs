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
