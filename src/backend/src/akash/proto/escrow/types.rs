/// Account stores state for an escrow account
///
/// https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/escrow/v1beta3/types.proto#L23
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(message, tag = "1")]
    pub id: ::core::option::Option<AccountId>,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(enumeration = "State", tag = "3")]
    pub state: i32,
    #[prost(message, tag = "4")]
    pub balance: ::core::option::Option<cosmrs::proto::cosmos::base::v1beta1::DecCoin>,
    #[prost(message, tag = "5")]
    pub transferred: ::core::option::Option<cosmrs::proto::cosmos::base::v1beta1::DecCoin>,
    #[prost(int64, tag = "6")]
    pub settled_at: i64,
    #[prost(string, tag = "7")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, tag = "8")]
    pub funds: ::core::option::Option<cosmrs::proto::cosmos::base::v1beta1::DecCoin>,
}

/// AccountID is the account identifier
///
/// https://github.com/akash-network/akash-api/blob/40e1584bc52f8753296e07a562265a034bf35bef/proto/node/akash/escrow/v1beta3/types.proto#L10
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountId {
    #[prost(string, tag = "1")]
    pub scope: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub xid: ::prost::alloc::string::String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[allow(clippy::enum_variant_names)]
pub enum State {
    AccountStateInvalid = 0,
    AccountOpen = 1,
    AccountClosed = 2,
    AccountOverdrawn = 3,
}
