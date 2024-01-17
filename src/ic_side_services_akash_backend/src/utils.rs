pub fn get_time_nanos() -> u64 {
    ic_cdk::api::time()
}

pub fn get_time_micros() -> u64 {
    get_time_nanos() / 1000
}

pub fn get_time_millis() -> u64 {
    get_time_nanos() / 1_000_000
}
