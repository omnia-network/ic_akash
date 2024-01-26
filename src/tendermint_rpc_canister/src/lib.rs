use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
mod base64string;
mod endpoints;
mod request;
use endpoints::tx_sync::Request;
use request::Wrapper;

#[ic_cdk::update]
async fn broadcast_tx_sync(tx_raw: String) -> Result<(), String> {
    let tx = hex::decode(tx_raw).map_err(|e| e.to_string())?;

    let request = Request::new(tx);

    let request_body = Wrapper::new(request).into_json().into_bytes();

    ic_cdk::api::print(format!("{:?}", request_body));

    // let host = "putsreq.com";
    let url = "https://rpc.sandbox-01.aksh.pw";

    // 2.2 prepare headers for the system http_request call
    //Note that `HttpHeader` is declared in line 4
    let request_headers = vec![
        // HttpHeader {
        //     name: "Host".to_string(),
        //     value: format!("{host}:443"),
        // },
        // HttpHeader {
        //     name: "User-Agent".to_string(),
        //     value: "demo_HTTP_POST_canister".to_string(),
        // },
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
        method: HttpMethod::POST,
        headers: request_headers,
        body: Some(request_body),
        transform: None,
    };

    ic_cdk::api::print(format!("{:?}", request));

    match http_request(request).await {
        //4. DECODE AND RETURN THE RESPONSE

        //See:https://docs.rs/ic-cdk/latest/ic_cdk/api/management_canister/http_request/struct.HttpResponse.html
        Ok((response,)) => {
            //if successful, `HttpResponse` has this structure:
            // pub struct HttpResponse {
            //     pub status: Nat,
            //     pub headers: Vec<HttpHeader>,
            //     pub body: Vec<u8>,
            // }

            //You need to decode that Vec<u8> that is the body into readable text.
            //To do this:
            //  1. Call `String::from_utf8()` on response.body
            //  3. Use a switch to explicitly call out both cases of decoding the Blob into ?Text
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");
            ic_cdk::api::print(format!("{:?}", str_body));

            //The API response will looks like this:
            // { successful: true }

            //Return the body as a string and end the method
            let result: String = format!(
                "{}. See more info of the request sent at: {}/inspect",
                str_body, url
            );
            result
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            //Return the error as a string and end the method
            message
        }
    };

    Ok(())
}
