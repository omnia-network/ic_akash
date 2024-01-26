use cosmrs::{crypto::PublicKey, AccountId};

pub fn get_account_id_from_public_key(public_key: &PublicKey) -> Result<AccountId, String> {
    public_key.account_id("akash").map_err(|e| e.to_string())
}
