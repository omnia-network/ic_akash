const NANOS_IN_SECOND: u64 = 1_000_000_000;

pub fn get_time_nanos() -> u64 {
    #[cfg(target_family = "wasm")]
    {
        ic_cdk::api::time()
    }

    #[cfg(not(target_family = "wasm"))]
    {
        1704063600 * 1_000_000_000 // 2024-01-01T00:00:00 in nanoseconds
    }
}

pub fn get_time_seconds() -> u64 {
    get_time_nanos() / NANOS_IN_SECOND
}

pub fn get_date_time() -> Result<chrono::DateTime<chrono::Utc>, String> {
    let timestamp_s = get_time_seconds();
    let timestamp_s = timestamp_s.try_into().map_err(|_| {
        format!(
            "Failed to convert timestamp {} from into seconds",
            timestamp_s
        )
    })?;

    chrono::DateTime::from_timestamp(timestamp_s, 0)
        .ok_or_else(|| format!("Failed to convert timestamp {} to DateTime", timestamp_s))
}
