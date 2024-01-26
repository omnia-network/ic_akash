use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
    TransformContext,
};
mod base64string;
mod endpoints;
mod rand;
mod request;
mod uuid;

use endpoints::{block::Request as BlockRequest, tx_sync::Request as TxSyncRequest};
use request::Wrapper;

#[ic_cdk::update]
async fn latest_block() -> Result<(), String> {
    let request = BlockRequest::default();
    let request_body = Wrapper::new(request).into_json().into_bytes();

    let response = make_rpc_request(HttpMethod::GET, Some(request_body), None).await?;
    let str_body =
        String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");
    ic_cdk::api::print(format!("{:?}", str_body));

    Ok(())
}

#[ic_cdk::update]
async fn broadcast_tx_sync(tx_raw: String) -> Result<(), String> {
    let tx = hex::decode(tx_raw).map_err(|e| e.to_string())?;
    let request = TxSyncRequest::new(tx);
    let request_body = Wrapper::new(request).into_json().into_bytes();

    let response = make_rpc_request(HttpMethod::POST, Some(request_body), None).await?;
    let str_body =
        String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");
    ic_cdk::api::print(format!("{:?}", str_body));

    Ok(())
}

async fn make_rpc_request(
    method: HttpMethod,
    request_body: Option<Vec<u8>>,
    transform: Option<TransformContext>,
) -> Result<HttpResponse, String> {
    let url = "https://rpc.sandbox-01.aksh.pw";

    let request_headers = vec![
        // //For the purposes of this exercise, Idempotency-Key" is hard coded, but in practice
        // //it should be generated via code and unique to each POST request. Common to create helper methods for this
        // HttpHeader {
        //     name: "Idempotency-Key".to_string(),
        //     value: "UUID-123456789".to_string(),
        // },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
    ];

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
