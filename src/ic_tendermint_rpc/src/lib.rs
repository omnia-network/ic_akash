use candid::Nat;
use ic_cdk::{
    api::management_canister::http_request::{
        HttpHeader, HttpMethod, HttpResponse, TransformArgs, TransformContext,
    },
    print, query,
};
mod endpoints;
mod id;
mod method;
mod request;
mod response;
mod response_error;
mod serializers;
mod version;

use endpoints::{
    abci_info::Request as AbciInfoRequest, abci_query::Request as AbciQueryRequest,
    tx::Request as TxRequest, tx_sync::Request as TxSyncRequest,
};
use request::{Request, Wrapper};
use response::Response;
use tendermint::{block::Height, hash::Algorithm, Hash};
use utils::{make_http_request, sha256};

/// assume requests are at most 5kb
const REQUEST_SIZE: u128 = 5_000;
/// refuse responses that return more than 15kb
const MAX_RESPONSE_SIZE: u64 = 15_000;

// TODO: fix deserialization
// pub async fn latest_block(url: String) -> Result<<BlockRequest as Request>::Response, String> {
//     let request = BlockRequest::default();
//     let request_body = Wrapper::new(request).await.into_json().into_bytes();

//     let response = make_http_request(url, HttpMethod::GET, Some(request_body), None).await?;
//     <BlockRequest as Request>::Response::from_string(&response.body)
// }

pub async fn abci_info(url: String) -> Result<<AbciInfoRequest as Request>::Response, String> {
    let request_body = Wrapper::new(AbciInfoRequest).await.into_json().into_bytes();

    let request_headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    let response = make_http_request(
        url,
        HttpMethod::GET,
        Some(request_body),
        request_headers,
        None,
        REQUEST_SIZE,
        MAX_RESPONSE_SIZE,
    )
    .await?;
    <AbciInfoRequest as Request>::Response::from_string(response.body)
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
        height: height.map(Height::new),
        prove,
    };
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let request_headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    let response = make_http_request(
        url,
        HttpMethod::POST,
        Some(request_body),
        request_headers,
        Some(TransformContext::from_name(
            "abci_transform".to_string(),
            vec![],
        )),
        REQUEST_SIZE,
        MAX_RESPONSE_SIZE,
    )
    .await?;
    <AbciQueryRequest as Request>::Response::from_string(response.body)
}

pub async fn check_tx(url: String, hash_hex: String) -> Result<(), String> {
    let request = TxRequest::new(
        Hash::from_hex_upper(Algorithm::Sha256, &hash_hex.to_uppercase()).unwrap(),
        true,
    );
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let request_headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    let response = make_http_request(
        url,
        HttpMethod::GET,
        Some(request_body),
        request_headers,
        None,
        REQUEST_SIZE,
        MAX_RESPONSE_SIZE,
    )
    .await?;
    let response_body = <TxRequest as Request>::Response::from_string(response.body);
    if let Ok(response_body) = response_body {
        print(format!(
            "[check_tx] response: {:?}",
            response_body.tx_result
        ));
    }

    Ok(())
}

pub async fn broadcast_tx_sync(
    is_mainnet: bool,
    url: String,
    tx_raw: Vec<u8>,
) -> Result<String, String> {
    let status_ok = Nat::from(200u16);

    let request = TxSyncRequest::new(tx_raw.clone());
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let request_headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    let response = make_http_request(
        url.clone(),
        HttpMethod::POST,
        Some(request_body),
        request_headers,
        Some(TransformContext::from_name(
            "broadcast_tx_sync_transform".to_string(),
            vec![],
        )),
        REQUEST_SIZE,
        MAX_RESPONSE_SIZE,
    )
    .await?;

    if response.status != status_ok {
        return Err(format!(
            "incorrect status. Expected 200, received: {:?}",
            response.status
        ));
    }

    // When deployed on mainnet the response should be an 'Err' that contains 'tx already exists in cache'
    // even if the transaction is accepted by the Akash Network.
    // This is due to the majority of replicas sending the same request to the Akash Network
    // and thus receiving the error as a response
    let tx_raw = match (
        <TxSyncRequest as Request>::Response::from_string(&response.body),
        is_mainnet,
    ) {
        (Ok(response), _) if response.code.is_err() => {
            return Err(format!("JSON RPC error: {:?}", response))
        }
        (Ok(response), true) => {
            return Err(format!(
                "response should contain 'tx already exists in cache', received: {:?} instead",
                response
            ))
        }
        (Ok(_), false) => tx_raw,
        (Err(e), true) if e.contains("tx already exists in cache") => tx_raw,
        (Err(e), _) => return Err(e),
    };

    Ok(hex::encode(sha256(&tx_raw)))
}

#[query]
fn abci_transform(raw: TransformArgs) -> HttpResponse {
    // the body of responses made to the abci endpoints are identical, therefore we can include the whole body in the response
    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers: vec![],
    }
}

#[query]
fn broadcast_tx_sync_transform(raw: TransformArgs) -> HttpResponse {
    // the response to the first request should be accepted and return 'Ok' while the others should be 'Err' and contain "tx already exists in cache"
    // as the transformed response is accepted if at least 2f+1 replicas are in agreement and, in the worst case, at most one honest replica (the one that sent the first request) disagrees
    // (received 'Ok' instead of 'Err'), as long as at most f-1 replicas misreport the response they received, there will be agreement in the transformed response
    // which is expected to be 'Err' containing "tx already exists in cache"
    // !!! this assumes at most f-1 (instead of f) replicas are malicious, as the one replica might honestly support 'Ok' as a response if it's request was the first accepted by the Akash Network !!!
    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers: vec![],
    }
}
