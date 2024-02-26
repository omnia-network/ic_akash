use crate::{
    akash::{
        address::get_account_id_from_public_key,
        auth::get_account,
        bank::{create_send_tx, get_balance},
        bids::fetch_bids,
        certificate::create_certificate_tx,
        deployment::{
            close_deployment_tx, create_deployment_tx, deposit_deployment_tx,
            update_deployment_sdl_tx,
        },
        lease::create_lease_tx,
        provider::fetch_provider,
        sdl::SdlV3,
    },
    api::{init_config, Config, ConfigMemory},
};
use cosmrs::AccountId;
use std::str::FromStr;
use utils::base64_decode;

pub struct AkashService {
    config_memory: ConfigMemory,
}

impl AkashService {
    pub fn default() -> Self {
        Self {
            config_memory: init_config(),
        }
    }

    pub fn get_config(&self) -> Config {
        self.config_memory.get().clone()
    }

    pub async fn address(&self) -> Result<String, String> {
        let config = self.get_config();

        let public_key = config.public_key().await?;

        Ok(get_account_id_from_public_key(&public_key)?.to_string())
    }

    pub async fn balance(&self) -> Result<u64, String> {
        let config = self.get_config();

        let public_key = config.public_key().await?;

        let balance = get_balance(config.tendermint_rpc_url(), &public_key)
            .await
            .and_then(|coin| Ok(coin.amount))?;

        Ok(balance
            .parse()
            .map_err(|e| format!("could not parse balance: {:?}", e))?)
    }

    pub async fn send(&self, to_address: String, amount: u64) -> Result<String, String> {
        let config = self.get_config();

        let public_key = config.public_key().await?;
        let rpc_url = config.tendermint_rpc_url();

        let account = get_account(rpc_url.clone(), &public_key).await?;

        let recipient_account_id =
            AccountId::from_str(to_address.as_str()).map_err(|e| e.to_string())?;

        let tx_raw = create_send_tx(
            &public_key,
            recipient_account_id,
            amount,
            &account,
            &config.ecdsa_key(),
        )
        .await?;

        let tx_hash =
            ic_tendermint_rpc::broadcast_tx_sync(config.is_mainnet(), rpc_url, tx_raw).await?;

        Ok(tx_hash)
    }

    pub async fn create_certificate(
        &self,
        cert_pem_base64: String,
        pub_key_pem_base64: String,
    ) -> Result<String, String> {
        let config = self.get_config();

        let public_key = config.public_key().await?;
        let rpc_url = config.tendermint_rpc_url();

        let cert_pem = base64_decode(&cert_pem_base64)?;
        let pub_key_pem = base64_decode(&pub_key_pem_base64)?;

        let account = get_account(rpc_url.clone(), &public_key).await?;

        let tx_raw = create_certificate_tx(
            &public_key,
            cert_pem,
            pub_key_pem,
            &account,
            &config.ecdsa_key(),
        )
        .await?;

        let tx_hash =
            ic_tendermint_rpc::broadcast_tx_sync(config.is_mainnet(), rpc_url, tx_raw).await?;

        Ok(tx_hash)
    }

    pub async fn create_deployment(&self, sdl: SdlV3) -> Result<(String, u64, String), String> {
        let config = self.get_config();

        let public_key = config.public_key().await?;
        let rpc_url = config.tendermint_rpc_url();

        let account = get_account(rpc_url.clone(), &public_key).await?;

        let abci_info_res = ic_tendermint_rpc::abci_info(rpc_url.clone()).await?;
        let dseq = abci_info_res.response.last_block_height.value();

        let tx_raw =
            create_deployment_tx(&public_key, &sdl, dseq, &account, &config.ecdsa_key()).await?;

        let tx_hash =
            ic_tendermint_rpc::broadcast_tx_sync(config.is_mainnet(), rpc_url, tx_raw).await?;

        // print(&format!(
        //     "[create_deployment] tx_hash: {}, dseq: {}",
        //     tx_hash, dseq
        // ));

        Ok((tx_hash, dseq, sdl.manifest_sorted_json()))
    }

    pub async fn deposit_deployment(&self, dseq: u64, amount_uakt: u64) -> Result<(), String> {
        let config = self.get_config();
        let public_key = config.public_key().await?;
        let rpc_url = config.tendermint_rpc_url();

        let account = get_account(rpc_url.clone(), &public_key).await?;

        let tx_raw = deposit_deployment_tx(
            &public_key,
            dseq,
            amount_uakt,
            &account,
            &config.ecdsa_key(),
        )
        .await?;

        let tx_hash =
            ic_tendermint_rpc::broadcast_tx_sync(config.is_mainnet(), rpc_url, tx_raw).await?;

        // print(&format!(
        //     "[deposit_deployment] tx_hash: {}, dseq: {}",
        //     tx_hash, dseq
        // ));

        Ok(())
    }

    pub async fn update_deployment_sdl(
        &self,
        dseq: u64,
        sdl: SdlV3,
    ) -> Result<(String, u64, String), String> {
        let config = self.get_config();

        let public_key = config.public_key().await?;
        let rpc_url = config.tendermint_rpc_url();

        let account = get_account(rpc_url.clone(), &public_key).await?;

        let tx_raw =
            update_deployment_sdl_tx(&public_key, &sdl, dseq, &account, &config.ecdsa_key())
                .await?;

        let tx_hash =
            ic_tendermint_rpc::broadcast_tx_sync(config.is_mainnet(), rpc_url, tx_raw).await?;

        // print(&format!(
        //     "[update_deployment_sdl] tx_hash: {}, dseq: {}",
        //     tx_hash, dseq
        // ));

        Ok((tx_hash, dseq, sdl.manifest_sorted_json()))
    }

    pub async fn check_tx(&self, tx_hash_hex: String) -> Result<(), String> {
        let config = self.get_config();

        ic_tendermint_rpc::check_tx(config.tendermint_rpc_url(), tx_hash_hex).await
    }

    pub async fn create_lease(&self, dseq: u64) -> Result<(String, String), String> {
        let config = self.get_config();

        let public_key = config.public_key().await?;
        let account_id = get_account_id_from_public_key(&public_key)?;
        let rpc_url = config.tendermint_rpc_url();

        let account = get_account(rpc_url.clone(), &public_key).await?;

        let bids = fetch_bids(rpc_url.clone(), &account_id, dseq).await?;
        // print(format!("[create_lease] bids: {:?}", bids));

        // TODO: take the "best" bid
        // SAFETY:
        // 'create_lease' is called by the 'handle_create_lease' function which is itself called by the 'fetch_bids' function
        // the latter makes sure that there is at least one bid before calling 'create_lease' so accessing the first bid is safe
        let bid = bids[0].bid.clone().unwrap();
        let bid_id = bid.bid_id.unwrap();

        let tx_raw =
            create_lease_tx(&public_key, bid_id.clone(), &account, &config.ecdsa_key()).await?;

        let tx_hash =
            ic_tendermint_rpc::broadcast_tx_sync(config.is_mainnet(), rpc_url, tx_raw).await?;

        // TODO: query lease to see if everything is ok

        let provider = fetch_provider(config.tendermint_rpc_url(), bid_id.provider).await?;

        Ok((tx_hash, provider.hostURI))
    }

    pub async fn close_deployment(&self, dseq: u64) -> Result<String, String> {
        let config = self.get_config();

        let public_key = config.public_key().await?;
        let rpc_url = config.tendermint_rpc_url();

        let account = get_account(rpc_url.clone(), &public_key).await?;

        let tx_raw = close_deployment_tx(&public_key, dseq, &account, &config.ecdsa_key()).await?;

        let tx_hash =
            ic_tendermint_rpc::broadcast_tx_sync(config.is_mainnet(), rpc_url, tx_raw).await?;

        Ok(tx_hash)
    }
}
