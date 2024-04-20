const NANOS_IN_SECOND: u64 = 1_000_000_000;

pub fn get_time_nanos() -> u64 {
    ic_cdk::api::time()
}

pub fn get_time_seconds() -> u64 {
    get_time_nanos() / NANOS_IN_SECOND
}
