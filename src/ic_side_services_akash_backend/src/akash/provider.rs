use prost::Message;

use super::proto::provider::{
    provider::Provider,
    query::{QueryProviderRequest, QueryProviderResponse},
};

pub async fn fetch_provider(provider_address: String) -> Result<Provider, String> {
    let query = QueryProviderRequest {
        owner: provider_address,
    };

    // abci_query

    let res = QueryProviderResponse::decode(vec![].as_slice()).map_err(|e| e.to_string())?;

    Ok(res.provider.unwrap())
}
