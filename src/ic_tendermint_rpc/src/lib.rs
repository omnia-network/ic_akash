use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
        TransformArgs, TransformContext,
    },
    print, query,
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
    block::Request as BlockRequest, tx::Request as TxRequest, tx_sync::Request as TxSyncRequest,
};
use request::{Request, Wrapper};
use response::Response;
use tendermint::{block::Height, hash::Algorithm, Hash};

// TODO: fix deserialization
// pub async fn latest_block(url: String) -> Result<<BlockRequest as Request>::Response, String> {
//     let request = BlockRequest::default();
//     let request_body = Wrapper::new(request).await.into_json().into_bytes();

//     let response = make_rpc_request(url, HttpMethod::GET, Some(request_body), None).await?;
//     <BlockRequest as Request>::Response::from_string(&response.body)
// }

pub async fn abci_info(url: String) -> Result<<AbciInfoRequest as Request>::Response, String> {
    let request_body = Wrapper::new(AbciInfoRequest).await.into_json().into_bytes();

    let response = make_rpc_request(url, HttpMethod::GET, Some(request_body), None).await?;
    <AbciInfoRequest as Request>::Response::from_string(&response.body)
}

pub async fn abci_query(
    url: String,
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

    let response = make_rpc_request(url, HttpMethod::POST, Some(request_body), None).await?;
    <AbciQueryRequest as Request>::Response::from_string(&response.body)
}

pub async fn check_tx(url: String, tx_hash: String) -> Result<(), String> {
    // tx_hash is the hash returned in the 'Ok' response of 'broadcast_tx_sync'
    let request = TxRequest::new(
        Hash::from_hex_upper(Algorithm::Sha256, &tx_hash).unwrap(),
        true,
    );
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let response = make_rpc_request(url, HttpMethod::GET, Some(request_body), None).await?;
    let response_body = <TxRequest as Request>::Response::from_string(&response.body);
    print(format!("[check_tx] response: {:?}", response_body));

    Ok(())
}

pub async fn broadcast_tx_sync(url: String, tx_raw: Vec<u8>) -> Result<(), String> {
    let request = TxSyncRequest::new(tx_raw);
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let response = make_rpc_request(
        url,
        HttpMethod::POST,
        Some(request_body),
        Some(TransformContext::from_name(
            "broadcast_tx_sync_transform".to_string(),
            vec![],
        )),
    )
    .await?;

    if response.status != 200 {
        return Err(format!(
            "incorrect status. Expected 200, received: {:?}",
            response.status
        ));
    }
    if let Err(e) = <TxSyncRequest as Request>::Response::from_string(&response.body) {
        if e.contains("tx already exists in cache") {
            // the transaction has been processed
            Ok(())
        } else {
            Err(e)
        }
    } else {
        Err("response should contain 'tx already exists in cache'".to_string())
    }
}

async fn make_rpc_request(
    url: String,
    method: HttpMethod,
    request_body: Option<Vec<u8>>,
    transform: Option<TransformContext>,
) -> Result<HttpResponse, String> {
    let request_headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    let request = CanisterHttpRequestArgument {
        url,
        max_response_bytes: None, //optional for request
        method,
        headers: request_headers,
        body: request_body,
        transform,
    };

    // TODO: configure the amount of cycles properly
    match http_request(request, 21_000_000_000).await {
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
