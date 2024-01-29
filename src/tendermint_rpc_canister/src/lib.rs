use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
    TransformContext,
};
mod endpoints;
mod id;
mod method;
mod rand;
mod request;
mod response;
mod response_error;
mod serializers;
mod uuid;
mod version;

use endpoints::{
    abci_info::Request as AbciInfoRequest, abci_query::Request as AbciQueryRequest,
    block::Request as BlockRequest, tx_sync::Request as TxSyncRequest,
};
use request::{Request, Wrapper};
use response::Response;
use tendermint::block::Height;

// TODO: fix deserialization
// #[ic_cdk::update]
// async fn latest_block() -> Result<(), String> {
//     let request = BlockRequest::default();
//     let request_body = Wrapper::new(request).await.into_json().into_bytes();

//     let response = make_rpc_request(HttpMethod::GET, Some(request_body), None).await?;
//     let parsed_response = <BlockRequest as Request>::Response::from_string(&response.body);
//     ic_cdk::api::print(format!("{:?}", parsed_response));

//     Ok(())
// }

#[ic_cdk::update]
async fn abci_info() -> Result<(), String> {
    let request_body = Wrapper::new(AbciInfoRequest).await.into_json().into_bytes();

    let response = make_rpc_request(HttpMethod::GET, Some(request_body), None).await?;
    let parsed_response = <AbciInfoRequest as Request>::Response::from_string(&response.body);
    ic_cdk::api::print(format!("{:?}", parsed_response));

    Ok(())
}

#[ic_cdk::update]
async fn abci_query(
    path: Option<String>,
    str_data: String,
    height: Option<u64>,
    prove: bool,
) -> Result<(), String> {
    let data = hex::decode(str_data).map_err(|e| e.to_string())?;
    let request = AbciQueryRequest {
        path,
        data,
        height: height.map(|h| Height::new(h)),
        prove,
    };
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let response = make_rpc_request(HttpMethod::POST, Some(request_body), None).await?;
    let parsed_response = <AbciQueryRequest as Request>::Response::from_string(&response.body);
    ic_cdk::api::print(format!("{:?}", parsed_response));

    Ok(())
}

#[ic_cdk::update]
async fn broadcast_tx_sync(tx_raw: String) -> Result<(), String> {
    let tx = hex::decode(tx_raw).map_err(|e| e.to_string())?;
    let request = TxSyncRequest::new(tx);
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let response = make_rpc_request(HttpMethod::POST, Some(request_body), None).await?;
    let parsed_response = <TxSyncRequest as Request>::Response::from_string(&response.body);
    ic_cdk::api::print(format!("{:?}", parsed_response));

    Ok(())
}

async fn make_rpc_request(
    method: HttpMethod,
    request_body: Option<Vec<u8>>,
    transform: Option<TransformContext>,
) -> Result<HttpResponse, String> {
    let url = "https://rpc.sandbox-01.aksh.pw";

    let request_headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: None, //optional for request
        method,
        headers: request_headers,
        body: request_body,
        transform,
    };

    match http_request(request).await {
        Ok((response,)) => Ok(response),
        Err((r, m)) => Err(format!(
            "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
        )),
    }
}
