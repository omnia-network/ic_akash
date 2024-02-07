use candid::{encode_one, Principal};

use super::utils::test_env::TestEnv;

#[test]
fn should_get_address() {
    let env = TestEnv::new();
    let canister_id = env.get_backend_canister_id();
    let address = env.pic.update_call(
        canister_id,
        Principal::anonymous(),
        "address",
        encode_one(()).unwrap(),
    );

    assert!(address.is_ok());
}
