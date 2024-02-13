import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type ApiEmptyResult = { 'Ok' : null } |
  { 'Err' : ApiError };
export interface ApiError { 'code' : number, 'message' : string }
export type ApiStringResult = { 'Ok' : string } |
  { 'Err' : ApiError };
export interface CanisterOutputCertifiedMessages {
  'messages' : Array<CanisterOutputMessage>,
  'cert' : Uint8Array | number[],
  'tree' : Uint8Array | number[],
  'is_end_of_queue' : boolean,
}
export interface CanisterOutputMessage {
  'key' : string,
  'content' : Uint8Array | number[],
  'client_key' : ClientKey,
}
export interface CanisterWsCloseArguments { 'client_key' : ClientKey }
export type CanisterWsCloseResult = { 'Ok' : null } |
  { 'Err' : string };
export interface CanisterWsGetMessagesArguments { 'nonce' : bigint }
export type CanisterWsGetMessagesResult = {
    'Ok' : CanisterOutputCertifiedMessages
  } |
  { 'Err' : string };
export interface CanisterWsMessageArguments { 'msg' : WebsocketMessage }
export type CanisterWsMessageResult = { 'Ok' : null } |
  { 'Err' : string };
export interface CanisterWsOpenArguments {
  'gateway_principal' : GatewayPrincipal,
  'client_nonce' : bigint,
}
export type CanisterWsOpenResult = { 'Ok' : null } |
  { 'Err' : string };
export interface ClientKey {
  'client_principal' : ClientPrincipal,
  'client_nonce' : bigint,
}
export type ClientPrincipal = Principal;
export type CreateDeploymentResult = { 'Ok' : DeploymentId } |
  { 'Err' : ApiError };
export type CreateUserResult = { 'Ok' : UserId } |
  { 'Err' : ApiError };
export interface Deployment {
  'sdl' : string,
  'user_id' : UserId,
  'state_history' : Array<[TimestampNs, DeploymentUpdate]>,
}
export type DeploymentId = string;
export type DeploymentUpdate = { 'Initialized' : null } |
  { 'Failed' : { 'reason' : string } } |
  {
    'DeploymentCreated' : {
      'manifest_sorted_json' : string,
      'dseq' : bigint,
      'tx_hash' : string,
    }
  } |
  { 'Closed' : null } |
  { 'Active' : null } |
  { 'LeaseCreated' : { 'provider_url' : string, 'tx_hash' : string } };
export interface DeploymentUpdateWsMessage {
  'id' : string,
  'update' : DeploymentUpdate,
}
export type GatewayPrincipal = Principal;
export type GetDeploymentResult = {
    'Ok' : { 'id' : DeploymentId, 'deployment' : Deployment }
  } |
  { 'Err' : ApiError };
export type GetDeploymentsResult = {
    'Ok' : Array<{ 'id' : DeploymentId, 'deployment' : Deployment }>
  } |
  { 'Err' : ApiError };
export type GetUserResult = { 'Ok' : User } |
  { 'Err' : ApiError };
export type TimestampNs = bigint;
export interface User { 'role' : UserRole, 'created_at' : TimestampNs }
export type UserId = Principal;
export type UserRole = { 'Admin' : null } |
  { 'Deployer' : null };
export interface WebsocketMessage {
  'sequence_num' : bigint,
  'content' : Uint8Array | number[],
  'client_key' : ClientKey,
  'timestamp' : bigint,
  'is_service_message' : boolean,
}
export interface _SERVICE {
  'address' : ActorMethod<[], ApiStringResult>,
  'balance' : ActorMethod<[], ApiStringResult>,
  'close_deployment' : ActorMethod<[string], ApiEmptyResult>,
  'create_certificate' : ActorMethod<[string, string], ApiStringResult>,
  'create_deployment' : ActorMethod<[string], CreateDeploymentResult>,
  'create_test_deployment' : ActorMethod<[], CreateDeploymentResult>,
  'create_user' : ActorMethod<[], CreateUserResult>,
  'get_deployment' : ActorMethod<[string], GetDeploymentResult>,
  'get_deployments' : ActorMethod<[], GetDeploymentsResult>,
  'get_user' : ActorMethod<[], GetUserResult>,
  'promote_user_to_admin' : ActorMethod<[UserId], ApiEmptyResult>,
  'update_deployment' : ActorMethod<[string, DeploymentUpdate], ApiEmptyResult>,
  'ws_close' : ActorMethod<[CanisterWsCloseArguments], CanisterWsCloseResult>,
  'ws_get_messages' : ActorMethod<
    [CanisterWsGetMessagesArguments],
    CanisterWsGetMessagesResult
  >,
  'ws_message' : ActorMethod<
    [CanisterWsMessageArguments, [] | [DeploymentUpdateWsMessage]],
    CanisterWsMessageResult
  >,
  'ws_open' : ActorMethod<[CanisterWsOpenArguments], CanisterWsOpenResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
