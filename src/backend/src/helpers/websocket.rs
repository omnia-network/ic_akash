use crate::api::DeploymentUpdate;
use ic_cdk::print;
use ic_websocket_cdk::ClientPrincipal;

pub fn send_canister_update(client_principal: ClientPrincipal, update: DeploymentUpdate) {
    print(format!("Sending message: {:?}", update));
    if let Err(e) = ic_websocket_cdk::send(client_principal, update.candid_serialize()) {
        print(format!("Could not send message: {}", e));
    }
}
