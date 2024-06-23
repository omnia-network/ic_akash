use crate::api::{log_error, DeploymentUpdateWsMessage};
use ic_websocket_cdk::ClientPrincipal;

pub fn send_canister_update(client_principal: ClientPrincipal, update: DeploymentUpdateWsMessage) {
    if let Err(e) = ic_websocket_cdk::send(client_principal, update.candid_serialize()) {
        log_error!(format!("Failed to send ws message: {:?}", e), "websocket")
    }
}
