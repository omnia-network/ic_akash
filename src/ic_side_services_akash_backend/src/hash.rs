use ic_tendermint_rpc::hash;

pub fn sha256(input: &[u8]) -> [u8; 32] {
    hash::sha256(input)
}
