use ic_cdk::api::management_canister::main::raw_rand;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<Option<ChaCha20Rng>> = const { RefCell::new(None) };
}

async fn with_rng<T>(cb: impl FnOnce(&mut ChaCha20Rng) -> T) -> Result<T, String> {
    let is_init = RNG.with_borrow(|rng| rng.is_some());

    if !is_init {
        let seed = get_seed().await?;

        let rng = ChaCha20Rng::from_seed(seed);
        RNG.with(|option_rng| {
            option_rng.borrow_mut().get_or_insert(rng);
        });
    }

    RNG.with_borrow_mut(|rng| {
        let rng = rng
            .as_mut()
            .ok_or_else(|| "Failed to initialize random number generator".to_string())?;

        Ok(cb(rng))
    })
}

async fn get_seed() -> Result<[u8; 32], String> {
    let (seed,) = raw_rand().await.map_err(|(code, msg)| {
        format!("System API call to `raw_rand` failed: ({:?}) {}", code, msg).to_string()
    })?;

    seed.try_into().map_err(|err| {
        format!(
            "System API call to `raw_rand` did not return 32 bytes: ({:?})",
            err
        )
        .to_string()
    })
}

pub async fn with_random_bytes<const N: usize, T>(
    cb: impl FnOnce([u8; N]) -> T,
) -> Result<T, String> {
    with_rng(|rng| {
        let mut bytes = [0u8; N];
        rng.fill_bytes(&mut bytes);

        cb(bytes)
    })
    .await
}
