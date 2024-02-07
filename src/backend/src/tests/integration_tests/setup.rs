use super::utils::test_env::TestEnv;
use crate::api::ApiResult;
use candid::{decode_one, encode_one, Principal};
use pocket_ic::WasmResult;

#[test]
fn should_get_address() {
    if let ApiResult::Err(e) = call_address() {
        panic!("{:?}", e);
    }
}

#[test]
fn should_get_balance() {
    if let ApiResult::Err(e) = call_balance() {
        panic!("{:?}", e);
    }
}

fn call_address() -> ApiResult<String> {
    let env = TestEnv::new();
    let canister_id = env.get_backend_canister_id();
    let res = env
        .pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            "address",
            encode_one(()).unwrap(),
        )
        .expect("Failed to call canister");

    match res {
        WasmResult::Reply(bytes) => decode_one(&bytes).expect("Failed to decode reply"),
        _ => panic!("Expected reply"),
    }
}

fn call_balance() -> ApiResult<String> {
    let env = TestEnv::new();
    let canister_id = env.get_backend_canister_id();
    let res = env
        .pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            "balance",
            encode_one(()).unwrap(),
        )
        .expect("Failed to call canister");

    match res {
        WasmResult::Reply(bytes) => decode_one(&bytes).expect("Failed to decode reply"),
        _ => panic!("Expected reply"),
    }
}
