use prost::Message;

use crate::config::get_config;

use super::proto::provider::{
    provider::Provider,
    query::{QueryProviderRequest, QueryProviderResponse},
};

pub async fn fetch_provider(provider_address: String) -> Result<Provider, String> {
    let config = get_config();

    let query = QueryProviderRequest {
        owner: provider_address,
    };

    let abci_res = ic_tendermint_rpc::abci_query(
        config.tendermint_rpc_url(),
        Some(String::from("/akash.provider.v1beta3.Query/Provider")),
        query.encode_to_vec(),
        None,
        false,
    )
    .await?;

    let res = QueryProviderResponse::decode(abci_res.response.value.as_slice())
        .map_err(|e| e.to_string())?;

    Ok(res.provider.unwrap())
}
