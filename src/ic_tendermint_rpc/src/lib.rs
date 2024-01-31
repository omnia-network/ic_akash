use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
        TransformArgs, TransformContext,
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
// pub async fn latest_block() -> Result<<BlockRequest as Request>::Response, String> {
//     let request = BlockRequest::default();
//     let request_body = Wrapper::new(request).await.into_json().into_bytes();

//     let response = make_rpc_request(HttpMethod::GET, Some(request_body), None).await?;
//     <BlockRequest as Request>::Response::from_string(&response.body)
// }

pub async fn abci_info() -> Result<<AbciInfoRequest as Request>::Response, String> {
    let request_body = Wrapper::new(AbciInfoRequest).await.into_json().into_bytes();

    let response = make_rpc_request(HttpMethod::GET, Some(request_body), None).await?;
    <AbciInfoRequest as Request>::Response::from_string(&response.body)
}

pub async fn abci_query(
    path: Option<String>,
    data: Vec<u8>,
    height: Option<u64>,
    prove: bool,
) -> Result<<AbciQueryRequest as Request>::Response, String> {
    let request = AbciQueryRequest {
        path,
        data,
        height: height.map(|h| Height::new(h)),
        prove,
    };
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let response = make_rpc_request(HttpMethod::POST, Some(request_body), None).await?;
    <AbciQueryRequest as Request>::Response::from_string(&response.body)
}

pub async fn broadcast_tx_sync(
    tx_raw: Vec<u8>,
) -> Result<<TxSyncRequest as Request>::Response, String> {
    let request = TxSyncRequest::new(tx_raw);
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let response = make_rpc_request(HttpMethod::POST, Some(request_body), None).await?;
    <TxSyncRequest as Request>::Response::from_string(&response.body)
}

pub async fn make_rpc_request(
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

    // TODO: configure the amount of cycles properly
    match http_request(request, 5_000_000_000).await {
        Ok((response,)) => Ok(response),
        Err((r, m)) => Err(format!(
            "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
        )),
    }
}

#[query]
fn abci_transform(raw: TransformArgs) -> HttpResponse {
    // the body of responses made to the abci endpoints are identical, therefore we can include the whole body in the response
    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        ..Default::default()
    }
}

#[query]
fn broadcast_tx_sync_transform(raw: TransformArgs) -> HttpResponse {
    // headers that you want to add to the response
    let headers = vec![
        // HttpHeader {
        //     name: "header-key".to_string(),
        //     value: "header-value".to_string(),
        // }
    ];

    // the response to the first request should be accepted and return 'Ok' while the others should be 'Err' and contain "tx already exists in cache"
    // as the transformed response is accepted if at least 2f+1 replicas are in agreement and, in the worst case, at most one honest replica (the one that sent the first request) disagrees
    // (received 'Ok' instead of 'Err'), as long as at most f-1 replicas misreport the response they received, there will be agreement in the transformed response
    // which is expected to be 'Err' containing "tx already exists in cache"
    // !!! this assumes at most f-1 (instead of f) replicas are malicious, as the one replica might honestly support 'Ok' as a response if it's request was the first accepted by the Akash Network !!!
    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
        ..Default::default()
    }
}
