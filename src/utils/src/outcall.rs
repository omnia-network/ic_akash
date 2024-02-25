use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
    TransformContext,
};

pub async fn make_http_request(
    url: String,
    method: HttpMethod,
    request_body: Option<Vec<u8>>,
    transform: Option<TransformContext>,
    max_response_size: u64,
    max_cycles_per_outcall: u128,
) -> Result<HttpResponse, String> {
    let request_headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    let request = CanisterHttpRequestArgument {
        url,
        max_response_bytes: Some(max_response_size),
        method,
        headers: request_headers,
        body: request_body,
        transform,
    };

    match http_request(request, max_cycles_per_outcall).await {
        Ok((response,)) => Ok(response),
        Err((r, m)) => Err(format!(
            "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
        )),
    }
}
