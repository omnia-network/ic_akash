use ic_cdk::{print, query, update};

use ic_websocket_cdk::{
    CanisterWsCloseArguments, CanisterWsCloseResult, CanisterWsGetMessagesArguments,
    CanisterWsGetMessagesResult, CanisterWsMessageArguments, CanisterWsMessageResult,
    CanisterWsOpenArguments, CanisterWsOpenResult, OnCloseCallbackArgs, OnOpenCallbackArgs,
    WsHandlers, WsInitParams,
};

use crate::{api::DeploymentUpdate, helpers::send_canister_update};

pub fn init_ic_websocket() {
    let handlers = WsHandlers {
        on_open: Some(on_open),
        on_message: None,
        on_close: Some(on_close),
    };

    let params = WsInitParams::new(handlers);

    ic_websocket_cdk::init(params);
}
// method called by the client to open a WS connection to the canister (relayed by the WS Gateway)
#[update]
fn ws_open(args: CanisterWsOpenArguments) -> CanisterWsOpenResult {
    ic_websocket_cdk::ws_open(args)
}

// method called by the Ws Gateway when closing the IcWebSocket connection for a client
#[update]
fn ws_close(args: CanisterWsCloseArguments) -> CanisterWsCloseResult {
    ic_websocket_cdk::ws_close(args)
}

// method called by the client to send a message to the canister (relayed by the WS Gateway)
#[update]
fn ws_message(
    args: CanisterWsMessageArguments,
    msg_type: Option<DeploymentUpdate>,
) -> CanisterWsMessageResult {
    ic_websocket_cdk::ws_message(args, msg_type)
}

// method called by the WS Gateway to get messages for all the clients it serves
#[query]
fn ws_get_messages(args: CanisterWsGetMessagesArguments) -> CanisterWsGetMessagesResult {
    ic_websocket_cdk::ws_get_messages(args)
}

fn on_open(args: OnOpenCallbackArgs) {
    print(format!("Client {} connected", args.client_principal));

    send_canister_update(args.client_principal, DeploymentUpdate::Initialized);
}

fn on_close(args: OnCloseCallbackArgs) {
    print(format!("Client {} disconnected", args.client_principal));
}
