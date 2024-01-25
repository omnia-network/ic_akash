#[ic_cdk::update]
fn broadcast_tx_sync(tx_raw: String) -> Result<(), String> {
    hex::decode(tx_raw).map_err(|e| e.to_string())?;
    Ok(())
}
