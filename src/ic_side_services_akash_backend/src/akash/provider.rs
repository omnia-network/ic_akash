use prost::Message;

use super::proto::provider::{
    provider::Provider,
    query::{QueryProviderRequest, QueryProviderResponse},
};

pub async fn fetch_provider(provider_address: String) -> Result<Provider, String> {
    let query = QueryProviderRequest {
        owner: provider_address,
    };

    let abci_res = ic_tendermint_rpc::abci_query(
        Some(String::from("/akash.provider.v1beta3.QueryProviderRequest")),
        query.encode_to_vec(),
        None,
        false,
    )
    .await?;

    let res = QueryProviderResponse::decode(abci_res.response.value.as_slice())
        .map_err(|e| e.to_string())?;

    Ok(res.provider.unwrap())
}
