use ic_cdk::{
    api::management_canister::http_request::{
        HttpMethod, HttpResponse, TransformArgs, TransformContext,
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
    block::Request as BlockRequest, tx::Request as TxRequest, tx_sync::Request as TxSyncRequest,
};
use request::{Request, Wrapper};
use response::Response;
use tendermint::{block::Height, hash::Algorithm, Hash};
use utils::{make_http_request, sha256};

/// assume requests are at most 1kb
const REQUEST_SIZE: u128 = 1_000;
/// refuse responses that return more than 10kb
const MAX_RESPONSE_SIZE: u64 = 10_000;
/// assume deployment on an application subnet
const SUBNET_SIZE: u128 = 13;
/// cycles cost for each HTTP outcall
const PER_CALL_COST: u128 = (3_000_000 + 60_000 * SUBNET_SIZE) * SUBNET_SIZE;
/// cycles cost for each byte in the request
const PER_REQUEST_BYTES_COST: u128 = 400 * SUBNET_SIZE;
/// cycles cost for each byte in the response
const PER_RESPONSE_BYTES_COST: u128 = 800 * SUBNET_SIZE;
// previous calculations according to: https://internetcomputer.org/docs/current/developer-docs/gas-cost#special-features

/// price calculated according to: https://internetcomputer.org/docs/current/developer-docs/integrations/https-outcalls/https-outcalls-how-it-works#pricing
const MAX_CYCLES_PER_OUTCALL: u128 = (PER_CALL_COST
    + PER_REQUEST_BYTES_COST * REQUEST_SIZE
    + PER_RESPONSE_BYTES_COST * MAX_RESPONSE_SIZE as u128)
    * (SUBNET_SIZE / 13);

// TODO: fix deserialization
// pub async fn latest_block(url: String) -> Result<<BlockRequest as Request>::Response, String> {
//     let request = BlockRequest::default();
//     let request_body = Wrapper::new(request).await.into_json().into_bytes();

//     let response = make_http_request(url, HttpMethod::GET, Some(request_body), None).await?;
//     <BlockRequest as Request>::Response::from_string(&response.body)
// }

pub async fn abci_info(url: String) -> Result<<AbciInfoRequest as Request>::Response, String> {
    let request_body = Wrapper::new(AbciInfoRequest).await.into_json().into_bytes();

    let response = make_http_request(
        url,
        HttpMethod::GET,
        Some(request_body),
        None,
        MAX_RESPONSE_SIZE,
        MAX_CYCLES_PER_OUTCALL,
    )
    .await?;
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

    let response = make_http_request(
        url,
        HttpMethod::POST,
        Some(request_body),
        Some(TransformContext::from_name(
            "abci_transform".to_string(),
            vec![],
        )),
        MAX_RESPONSE_SIZE,
        MAX_CYCLES_PER_OUTCALL,
    )
    .await?;
    <AbciQueryRequest as Request>::Response::from_string(&response.body)
}

pub async fn check_tx(url: String, hash_hex: String) -> Result<(), String> {
    let request = TxRequest::new(
        Hash::from_hex_upper(Algorithm::Sha256, &hash_hex.to_uppercase()).unwrap(),
        true,
    );
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let response = make_http_request(
        url,
        HttpMethod::GET,
        Some(request_body),
        None,
        MAX_RESPONSE_SIZE,
        MAX_CYCLES_PER_OUTCALL,
    )
    .await?;
    let response_body = <TxRequest as Request>::Response::from_string(&response.body);
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
    let request = TxSyncRequest::new(tx_raw.clone());
    let request_body = Wrapper::new(request).await.into_json().into_bytes();

    let response = make_http_request(
        url.clone(),
        HttpMethod::POST,
        Some(request_body),
        Some(TransformContext::from_name(
            "broadcast_tx_sync_transform".to_string(),
            vec![],
        )),
        MAX_RESPONSE_SIZE,
        MAX_CYCLES_PER_OUTCALL,
    )
    .await?;

    if response.status != 200 {
        return Err(format!(
            "incorrect status. Expected 200, received: {:?}",
            response.status
        ));
    }

    if is_mainnet {
        // when dpeloyed on mainnet the response should eb 'Err' and contain 'tx already exists in cache' even if the transaction is accepted by the Akash Network
        // this is due to the majority of replicas sending a duplicate request to the Network and thus receiving the error as a response
        if let Err(e) = <TxSyncRequest as Request>::Response::from_string(&response.body) {
            if e.contains("tx already exists in cache") {
                // the transaction has been processed
                Ok(hex::encode(&sha256(&tx_raw)))
            } else {
                Err(e)
            }
        } else {
            Err("response should contain 'tx already exists in cache'".to_string())
        }
    } else {
        // when testing locally only one request is made and therefore the response is 'Ok' if the transaction is accepted by the Akash Network
        Ok(hex::encode(&sha256(&tx_raw)))
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
