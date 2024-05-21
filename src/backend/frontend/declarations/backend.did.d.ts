import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [SubAccount],
}
export interface AccountBalanceArgs { 'account' : AccountIdentifier }
export type AccountIdentifier = Uint8Array | number[];
export type ApiEmptyResult = { 'Ok' : null } |
  { 'Err' : ApiError };
export interface ApiError { 'code' : number, 'message' : string }
export type ApiFloatResult = { 'Ok' : number } |
  { 'Err' : ApiError };
export type ApiNatResult = { 'Ok' : bigint } |
  { 'Err' : ApiError };
export type ApiStringResult = { 'Ok' : string } |
  { 'Err' : ApiError };
export interface ArchivedBlocksRange {
  'callback' : QueryArchiveFn,
  'start' : BlockIndex,
  'length' : bigint,
}
export interface Block {
  'transaction' : Transaction,
  'timestamp' : TimeStamp,
  'parent_hash' : [] | [Uint8Array | number[]],
}
export type BlockIndex = bigint;
export interface BlockRange { 'blocks' : Array<Block> }
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
export type CpuSize = { 'Large' : null } |
  { 'Small' : null } |
  { 'Medium' : null };
export type CreateDeploymentResult = { 'Ok' : DeploymentId } |
  { 'Err' : ApiError };
export type CreateUserResult = { 'Ok' : UserId } |
  { 'Err' : ApiError };
export interface Deployment {
  'user_id' : UserId,
  'icp_price' : number,
  'state_history' : Array<[TimestampNs, DeploymentState]>,
  'params' : DeploymentParams,
}
export type DeploymentId = string;
export interface DeploymentParams {
  'cpu' : CpuSize,
  'memory' : MemorySize,
  'storage' : StorageSize,
  'name' : string,
  'volume_mount' : [] | [string],
  'command' : Array<string>,
  'env_vars' : Array<[string, string]>,
  'image' : string,
  'ports' : Array<
    {
      'container_port' : number,
      'domain' : [] | [string],
      'host_port' : number,
    }
  >,
}
export type DeploymentState = { 'FailedOnClient' : { 'reason' : string } } |
  { 'Initialized' : null } |
  {
    'DeploymentCreated' : {
      'manifest_sorted_json' : string,
      'dseq' : bigint,
      'tx_hash' : string,
    }
  } |
  { 'Closed' : null } |
  { 'Active' : null } |
  { 'LeaseCreated' : { 'provider_url' : string, 'tx_hash' : string } } |
  { 'FailedOnCanister' : { 'reason' : string } };
export interface DeploymentUpdateWsMessage {
  'id' : string,
  'update' : DeploymentState,
}
export type GatewayPrincipal = Principal;
export interface GetBlocksArgs { 'start' : BlockIndex, 'length' : bigint }
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
export type ListLogsResponse = { 'Ok' : { 'logs' : Array<LogEntry> } } |
  { 'Err' : ApiError };
export interface LogEntry {
  'context' : [] | [string],
  'date_time' : string,
  'level' : LogLevel,
  'message' : string,
}
export type LogLevel = { 'info' : null } |
  { 'warn' : null } |
  { 'error' : null };
export interface LogsFilterRequest {
  'context_contains_any' : [] | [Array<string>],
  'level' : [] | [LogLevel],
  'message_contains_any' : [] | [Array<string>],
  'after_timestamp_ms' : [] | [bigint],
  'before_timestamp_ms' : [] | [bigint],
}
export type Memo = bigint;
export type MemorySize = { 'Large' : null } |
  { 'Small' : null } |
  { 'Medium' : null };
export type Operation = {
    'Approve' : {
      'fee' : Tokens,
      'from' : AccountIdentifier,
      'allowance_e8s' : bigint,
      'allowance' : Tokens,
      'expires_at' : [] | [TimeStamp],
      'spender' : AccountIdentifier,
    }
  } |
  {
    'Burn' : {
      'from' : AccountIdentifier,
      'amount' : Tokens,
      'spender' : [] | [AccountIdentifier],
    }
  } |
  { 'Mint' : { 'to' : AccountIdentifier, 'amount' : Tokens } } |
  {
    'Transfer' : {
      'to' : AccountIdentifier,
      'fee' : Tokens,
      'from' : AccountIdentifier,
      'amount' : Tokens,
    }
  } |
  {
    'TransferFrom' : {
      'to' : AccountIdentifier,
      'fee' : Tokens,
      'from' : AccountIdentifier,
      'amount' : Tokens,
      'spender' : AccountIdentifier,
    }
  };
export type QueryArchiveError = {
    'BadFirstBlockIndex' : {
      'requested_index' : BlockIndex,
      'first_valid_index' : BlockIndex,
    }
  } |
  { 'Other' : { 'error_message' : string, 'error_code' : bigint } };
export type QueryArchiveFn = ActorMethod<[GetBlocksArgs], QueryArchiveResult>;
export type QueryArchiveResult = { 'Ok' : BlockRange } |
  { 'Err' : QueryArchiveError };
export interface QueryBlocksResponse {
  'certificate' : [] | [Uint8Array | number[]],
  'blocks' : Array<Block>,
  'chain_length' : bigint,
  'first_block_index' : BlockIndex,
  'archived_blocks' : Array<ArchivedBlocksRange>,
}
export type QueryBlocksResult = { 'Ok' : QueryBlocksResponse } |
  { 'Err' : ApiError };
export type StorageSize = { 'Large' : null } |
  { 'Small' : null } |
  { 'Medium' : null };
export type SubAccount = Uint8Array | number[];
export interface TimeStamp { 'timestamp_nanos' : bigint }
export type TimestampNs = bigint;
export interface Tokens { 'e8s' : bigint }
export interface Transaction {
  'memo' : Memo,
  'icrc1_memo' : [] | [Uint8Array | number[]],
  'operation' : [] | [Operation],
  'created_at_time' : TimeStamp,
}
export interface TransferArgs {
  'to' : AccountIdentifier,
  'fee' : Tokens,
  'memo' : Memo,
  'from_subaccount' : [] | [SubAccount],
  'created_at_time' : [] | [TimeStamp],
  'amount' : Tokens,
}
export type TransferError = {
    'TxTooOld' : { 'allowed_window_nanos' : bigint }
  } |
  { 'BadFee' : { 'expected_fee' : Tokens } } |
  { 'TxDuplicate' : { 'duplicate_of' : BlockIndex } } |
  { 'TxCreatedInFuture' : null } |
  { 'InsufficientFunds' : { 'balance' : Tokens } };
export interface TransferFee { 'transfer_fee' : Tokens }
export type TransferFeeArg = {};
export type TransferResult = { 'Ok' : BlockIndex } |
  { 'Err' : TransferError };
export interface User {
  'akt_balance' : number,
  'payments' : BigUint64Array | bigint[],
  'role' : UserRole,
  'created_at' : TimestampNs,
}
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
  'balance' : ActorMethod<[], ApiNatResult>,
  'check_tx' : ActorMethod<[string], ApiEmptyResult>,
  'close_deployment' : ActorMethod<[string], ApiEmptyResult>,
  'create_certificate' : ActorMethod<[string, string], ApiStringResult>,
  'create_deployment' : ActorMethod<[DeploymentParams], CreateDeploymentResult>,
  'create_test_deployment' : ActorMethod<[], CreateDeploymentResult>,
  'create_user' : ActorMethod<[], CreateUserResult>,
  'deposit_deployment' : ActorMethod<[string, bigint], ApiEmptyResult>,
  'get_deployment' : ActorMethod<[string], GetDeploymentResult>,
  'get_deployment_icp_price' : ActorMethod<[], ApiFloatResult>,
  'get_deployments' : ActorMethod<[], GetDeploymentsResult>,
  'get_my_user' : ActorMethod<[], GetUserResult>,
  'get_user' : ActorMethod<[Principal], GetUserResult>,
  'list_logs' : ActorMethod<[LogsFilterRequest], ListLogsResponse>,
  'promote_user_to_admin' : ActorMethod<[UserId], ApiEmptyResult>,
  'query_blocks' : ActorMethod<[GetBlocksArgs], QueryBlocksResult>,
  'set_mutual_tls_certificate' : ActorMethod<[string], ApiEmptyResult>,
  'update_akt_balance' : ActorMethod<[bigint], ApiFloatResult>,
  'update_deployment_state' : ActorMethod<
    [string, DeploymentState],
    ApiEmptyResult
  >,
  'update_test_deployment_sdl' : ActorMethod<[string], ApiEmptyResult>,
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
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
