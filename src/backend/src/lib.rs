mod akash;
mod api;
mod fixtures;
mod helpers;
mod tests;

// In the following, we register a custom getrandom implementation because
// otherwise getrandom (which is a dependency of some packages) fails to compile.
// This is necessary because getrandom by default fails to compile for the
// wasm32-unknown-unknown target (which is required for deploying a canister).
getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}
