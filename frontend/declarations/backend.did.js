export const idlFactory = ({ IDL }) => {
  const ApiError = IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text });
  const ApiStringResult = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : ApiError });
  const ApiNatResult = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : ApiError });
  const ApiEmptyResult = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : ApiError });
  const CpuSize = IDL.Variant({
    'Large' : IDL.Null,
    'Small' : IDL.Null,
    'Medium' : IDL.Null,
  });
  const MemorySize = IDL.Variant({
    'Large' : IDL.Null,
    'Small' : IDL.Null,
    'Medium' : IDL.Null,
  });
  const StorageSize = IDL.Variant({
    'Large' : IDL.Null,
    'Small' : IDL.Null,
    'Medium' : IDL.Null,
  });
  const DeploymentParams = IDL.Record({
    'cpu' : CpuSize,
    'memory' : MemorySize,
    'storage' : StorageSize,
    'name' : IDL.Text,
    'volume_mount' : IDL.Opt(IDL.Text),
    'command' : IDL.Vec(IDL.Text),
    'env_vars' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'image' : IDL.Text,
    'ports' : IDL.Vec(
      IDL.Record({
        'container_port' : IDL.Nat32,
        'domain' : IDL.Opt(IDL.Text),
        'host_port' : IDL.Nat32,
      })
    ),
  });
  const DeploymentId = IDL.Text;
  const CreateDeploymentResult = IDL.Variant({
    'Ok' : DeploymentId,
    'Err' : ApiError,
  });
  const UserId = IDL.Principal;
  const CreateUserResult = IDL.Variant({ 'Ok' : UserId, 'Err' : ApiError });
  const TimestampNs = IDL.Nat64;
  const DeploymentState = IDL.Variant({
    'FailedOnClient' : IDL.Record({ 'reason' : IDL.Text }),
    'Initialized' : IDL.Null,
    'DeploymentCreated' : IDL.Record({
      'manifest_sorted_json' : IDL.Text,
      'dseq' : IDL.Nat64,
      'tx_hash' : IDL.Text,
    }),
    'Closed' : IDL.Null,
    'Active' : IDL.Null,
    'LeaseCreated' : IDL.Record({
      'provider_url' : IDL.Text,
      'tx_hash' : IDL.Text,
    }),
    'FailedOnCanister' : IDL.Record({ 'reason' : IDL.Text }),
  });
  const Deployment = IDL.Record({
    'user_id' : UserId,
    'icp_price' : IDL.Float64,
    'state_history' : IDL.Vec(IDL.Tuple(TimestampNs, DeploymentState)),
    'params' : DeploymentParams,
  });
  const GetDeploymentResult = IDL.Variant({
    'Ok' : IDL.Record({ 'id' : DeploymentId, 'deployment' : Deployment }),
    'Err' : ApiError,
  });
  const ApiFloatResult = IDL.Variant({ 'Ok' : IDL.Float64, 'Err' : ApiError });
  const GetDeploymentsResult = IDL.Variant({
    'Ok' : IDL.Vec(
      IDL.Record({ 'id' : DeploymentId, 'deployment' : Deployment })
    ),
    'Err' : ApiError,
  });
  const UserRole = IDL.Variant({ 'Admin' : IDL.Null, 'Deployer' : IDL.Null });
  const User = IDL.Record({
    'akt_balance' : IDL.Float64,
    'payments' : IDL.Vec(IDL.Nat64),
    'role' : UserRole,
    'created_at' : TimestampNs,
  });
  const GetUserResult = IDL.Variant({ 'Ok' : User, 'Err' : ApiError });
  const LogLevel = IDL.Variant({
    'info' : IDL.Null,
    'warn' : IDL.Null,
    'error' : IDL.Null,
  });
  const LogsFilterRequest = IDL.Record({
    'context_contains_any' : IDL.Opt(IDL.Vec(IDL.Text)),
    'level' : IDL.Opt(LogLevel),
    'message_contains_any' : IDL.Opt(IDL.Vec(IDL.Text)),
    'after_timestamp_ms' : IDL.Opt(IDL.Nat64),
    'before_timestamp_ms' : IDL.Opt(IDL.Nat64),
  });
  const LogEntry = IDL.Record({
    'context' : IDL.Opt(IDL.Text),
    'date_time' : IDL.Text,
    'level' : LogLevel,
    'message' : IDL.Text,
  });
  const ListLogsResponse = IDL.Variant({
    'Ok' : IDL.Record({ 'logs' : IDL.Vec(LogEntry) }),
    'Err' : ApiError,
  });
  const BlockIndex = IDL.Nat64;
  const GetBlocksArgs = IDL.Record({
    'start' : BlockIndex,
    'length' : IDL.Nat64,
  });
  const Memo = IDL.Nat64;
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const AccountIdentifier = IDL.Vec(IDL.Nat8);
  const TimeStamp = IDL.Record({ 'timestamp_nanos' : IDL.Nat64 });
  const Operation = IDL.Variant({
    'Approve' : IDL.Record({
      'fee' : Tokens,
      'from' : AccountIdentifier,
      'allowance_e8s' : IDL.Int,
      'allowance' : Tokens,
      'expires_at' : IDL.Opt(TimeStamp),
      'spender' : AccountIdentifier,
    }),
    'Burn' : IDL.Record({
      'from' : AccountIdentifier,
      'amount' : Tokens,
      'spender' : IDL.Opt(AccountIdentifier),
    }),
    'Mint' : IDL.Record({ 'to' : AccountIdentifier, 'amount' : Tokens }),
    'Transfer' : IDL.Record({
      'to' : AccountIdentifier,
      'fee' : Tokens,
      'from' : AccountIdentifier,
      'amount' : Tokens,
    }),
    'TransferFrom' : IDL.Record({
      'to' : AccountIdentifier,
      'fee' : Tokens,
      'from' : AccountIdentifier,
      'amount' : Tokens,
      'spender' : AccountIdentifier,
    }),
  });
  const Transaction = IDL.Record({
    'memo' : Memo,
    'icrc1_memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'operation' : IDL.Opt(Operation),
    'created_at_time' : TimeStamp,
  });
  const Block = IDL.Record({
    'transaction' : Transaction,
    'timestamp' : TimeStamp,
    'parent_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const BlockRange = IDL.Record({ 'blocks' : IDL.Vec(Block) });
  const QueryArchiveError = IDL.Variant({
    'BadFirstBlockIndex' : IDL.Record({
      'requested_index' : BlockIndex,
      'first_valid_index' : BlockIndex,
    }),
    'Other' : IDL.Record({
      'error_message' : IDL.Text,
      'error_code' : IDL.Nat64,
    }),
  });
  const QueryArchiveResult = IDL.Variant({
    'Ok' : BlockRange,
    'Err' : QueryArchiveError,
  });
  const QueryArchiveFn = IDL.Func(
      [GetBlocksArgs],
      [QueryArchiveResult],
      ['query'],
    );
  const ArchivedBlocksRange = IDL.Record({
    'callback' : QueryArchiveFn,
    'start' : BlockIndex,
    'length' : IDL.Nat64,
  });
  const QueryBlocksResponse = IDL.Record({
    'certificate' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'blocks' : IDL.Vec(Block),
    'chain_length' : IDL.Nat64,
    'first_block_index' : BlockIndex,
    'archived_blocks' : IDL.Vec(ArchivedBlocksRange),
  });
  const QueryBlocksResult = IDL.Variant({
    'Ok' : QueryBlocksResponse,
    'Err' : ApiError,
  });
  const ClientPrincipal = IDL.Principal;
  const ClientKey = IDL.Record({
    'client_principal' : ClientPrincipal,
    'client_nonce' : IDL.Nat64,
  });
  const CanisterWsCloseArguments = IDL.Record({ 'client_key' : ClientKey });
  const CanisterWsCloseResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : IDL.Text,
  });
  const CanisterWsGetMessagesArguments = IDL.Record({ 'nonce' : IDL.Nat64 });
  const CanisterOutputMessage = IDL.Record({
    'key' : IDL.Text,
    'content' : IDL.Vec(IDL.Nat8),
    'client_key' : ClientKey,
  });
  const CanisterOutputCertifiedMessages = IDL.Record({
    'messages' : IDL.Vec(CanisterOutputMessage),
    'cert' : IDL.Vec(IDL.Nat8),
    'tree' : IDL.Vec(IDL.Nat8),
    'is_end_of_queue' : IDL.Bool,
  });
  const CanisterWsGetMessagesResult = IDL.Variant({
    'Ok' : CanisterOutputCertifiedMessages,
    'Err' : IDL.Text,
  });
  const WebsocketMessage = IDL.Record({
    'sequence_num' : IDL.Nat64,
    'content' : IDL.Vec(IDL.Nat8),
    'client_key' : ClientKey,
    'timestamp' : IDL.Nat64,
    'is_service_message' : IDL.Bool,
  });
  const CanisterWsMessageArguments = IDL.Record({ 'msg' : WebsocketMessage });
  const DeploymentUpdateWsMessage = IDL.Record({
    'id' : IDL.Text,
    'update' : DeploymentState,
  });
  const CanisterWsMessageResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : IDL.Text,
  });
  const GatewayPrincipal = IDL.Principal;
  const CanisterWsOpenArguments = IDL.Record({
    'gateway_principal' : GatewayPrincipal,
    'client_nonce' : IDL.Nat64,
  });
  const CanisterWsOpenResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : IDL.Text,
  });
  return IDL.Service({
    'address' : IDL.Func([], [ApiStringResult], []),
    'balance' : IDL.Func([], [ApiNatResult], []),
    'check_tx' : IDL.Func([IDL.Text], [ApiEmptyResult], []),
    'close_deployment' : IDL.Func([IDL.Text], [ApiEmptyResult], []),
    'create_certificate' : IDL.Func(
        [IDL.Text, IDL.Text],
        [ApiStringResult],
        [],
      ),
    'create_deployment' : IDL.Func(
        [DeploymentParams],
        [CreateDeploymentResult],
        [],
      ),
    'create_test_deployment' : IDL.Func([], [CreateDeploymentResult], []),
    'create_user' : IDL.Func([], [CreateUserResult], []),
    'deposit_deployment' : IDL.Func(
        [IDL.Text, IDL.Nat64],
        [ApiEmptyResult],
        [],
      ),
    'get_deployment' : IDL.Func([IDL.Text], [GetDeploymentResult], ['query']),
    'get_deployment_icp_price' : IDL.Func([], [ApiFloatResult], []),
    'get_deployments' : IDL.Func([], [GetDeploymentsResult], ['query']),
    'get_my_user' : IDL.Func([], [GetUserResult], ['query']),
    'get_user' : IDL.Func([IDL.Principal], [GetUserResult], ['query']),
    'list_logs' : IDL.Func([LogsFilterRequest], [ListLogsResponse], ['query']),
    'promote_user_to_admin' : IDL.Func([UserId], [ApiEmptyResult], []),
    'query_blocks' : IDL.Func(
        [GetBlocksArgs],
        [QueryBlocksResult],
        ['composite_query'],
      ),
    'update_akt_balance' : IDL.Func([IDL.Nat64], [ApiFloatResult], []),
    'update_deployment_state' : IDL.Func(
        [IDL.Text, DeploymentState],
        [ApiEmptyResult],
        [],
      ),
    'update_test_deployment_sdl' : IDL.Func([IDL.Text], [ApiEmptyResult], []),
    'ws_close' : IDL.Func(
        [CanisterWsCloseArguments],
        [CanisterWsCloseResult],
        [],
      ),
    'ws_get_messages' : IDL.Func(
        [CanisterWsGetMessagesArguments],
        [CanisterWsGetMessagesResult],
        ['query'],
      ),
    'ws_message' : IDL.Func(
        [CanisterWsMessageArguments, IDL.Opt(DeploymentUpdateWsMessage)],
        [CanisterWsMessageResult],
        [],
      ),
    'ws_open' : IDL.Func([CanisterWsOpenArguments], [CanisterWsOpenResult], []),
  });
};
export const init = ({ IDL }) => { return [IDL.Bool]; };
