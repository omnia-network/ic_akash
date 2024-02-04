use crate::api::AppMessage;
use ic_cdk::print;
use ic_websocket_cdk::ClientPrincipal;

pub fn send_app_message(client_principal: ClientPrincipal, msg: AppMessage) {
    print(format!("Sending message: {:?}", msg));
    if let Err(e) = ic_websocket_cdk::send(client_principal, msg.candid_serialize()) {
        print(format!("Could not send message: {}", e));
    }
}
