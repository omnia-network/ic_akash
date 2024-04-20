use prost::Message;

use super::proto::provider::{
    query::{QueryProviderRequest, QueryProviderResponse},
    Provider,
};

pub async fn fetch_provider(rpc_url: String, provider_address: String) -> Result<Provider, String> {
    let query = QueryProviderRequest {
        owner: provider_address,
    };

    let abci_res = ic_tendermint_rpc::abci_query(
        rpc_url,
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
