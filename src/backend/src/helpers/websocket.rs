use crate::api::DeploymentUpdateWsMessage;
use ic_websocket_cdk::ClientPrincipal;

pub fn send_canister_update(client_principal: ClientPrincipal, update: DeploymentUpdateWsMessage) {
    // print(format!("Sending message: {:?}", update));
    if ic_websocket_cdk::send(client_principal, update.candid_serialize()).is_err() {
        // print(format!("Could not send message: {}", e));
    }
}
