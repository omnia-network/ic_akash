use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
    TransformContext,
};

/// assume deployment on an application subnet
const SUBNET_SIZE: u128 = 13;
/// cycles cost for each HTTP outcall
const PER_CALL_COST: u128 = (3_000_000 + 60_000 * SUBNET_SIZE) * SUBNET_SIZE;
/// cycles cost for each byte in the request
const PER_REQUEST_BYTES_COST: u128 = 400 * SUBNET_SIZE;
/// cycles cost for each byte in the response
const PER_RESPONSE_BYTES_COST: u128 = 800 * SUBNET_SIZE;
// previous calculations according to: https://internetcomputer.org/docs/current/developer-docs/gas-cost#special-features

pub async fn make_http_request(
    url: String,
    method: HttpMethod,
    request_body: Option<Vec<u8>>,
    request_headers: Vec<HttpHeader>,
    transform: Option<TransformContext>,
    request_size: u128,
    max_response_size: u64,
) -> Result<HttpResponse, String> {
    let request = CanisterHttpRequestArgument {
        url,
        max_response_bytes: Some(max_response_size),
        method,
        headers: request_headers,
        body: request_body,
        transform,
    };

    match http_request(
        request,
        max_cycles_per_outcall(request_size, max_response_size),
    )
    .await
    {
        Ok((response,)) => Ok(response),
        Err((r, m)) => Err(format!(
            "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
        )),
    }
}

/// price calculated according to: https://internetcomputer.org/docs/current/developer-docs/integrations/https-outcalls/https-outcalls-how-it-works#pricing
fn max_cycles_per_outcall(request_size: u128, max_response_size: u64) -> u128 {
    (PER_CALL_COST
        + PER_REQUEST_BYTES_COST * request_size
        + PER_RESPONSE_BYTES_COST * max_response_size as u128)
        * (SUBNET_SIZE / 13)
}
