export const idlFactory = ({ IDL }) => {
  const ApiError = IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text });
  const ApiStringResult = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : ApiError });
  const ApiEmptyResult = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : ApiError });
  const DeploymentId = IDL.Text;
  const CreateDeploymentResult = IDL.Variant({
    'Ok' : DeploymentId,
    'Err' : ApiError,
  });
  const UserId = IDL.Principal;
  const CreateUserResult = IDL.Variant({ 'Ok' : UserId, 'Err' : ApiError });
  const TimestampNs = IDL.Nat64;
  const DeploymentUpdate = IDL.Variant({
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
    'sdl' : IDL.Text,
    'user_id' : UserId,
    'state_history' : IDL.Vec(IDL.Tuple(TimestampNs, DeploymentUpdate)),
  });
  const GetDeploymentResult = IDL.Variant({
    'Ok' : IDL.Record({ 'id' : DeploymentId, 'deployment' : Deployment }),
    'Err' : ApiError,
  });
  const GetDeploymentsResult = IDL.Variant({
    'Ok' : IDL.Vec(
      IDL.Record({ 'id' : DeploymentId, 'deployment' : Deployment })
    ),
    'Err' : ApiError,
  });
  const UserRole = IDL.Variant({ 'Admin' : IDL.Null, 'Deployer' : IDL.Null });
  const User = IDL.Record({ 'role' : UserRole, 'created_at' : TimestampNs });
  const GetUserResult = IDL.Variant({ 'Ok' : User, 'Err' : ApiError });
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
    'update' : DeploymentUpdate,
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
    'balance' : IDL.Func([], [ApiStringResult], []),
    'close_deployment' : IDL.Func([IDL.Text], [ApiEmptyResult], []),
    'create_certificate' : IDL.Func(
        [IDL.Text, IDL.Text],
        [ApiStringResult],
        [],
      ),
    'create_deployment' : IDL.Func([IDL.Text], [CreateDeploymentResult], []),
    'create_test_deployment' : IDL.Func([], [CreateDeploymentResult], []),
    'create_user' : IDL.Func([], [CreateUserResult], []),
    'get_deployment' : IDL.Func([IDL.Text], [GetDeploymentResult], ['query']),
    'get_deployments' : IDL.Func([], [GetDeploymentsResult], ['query']),
    'get_user' : IDL.Func([], [GetUserResult], ['query']),
    'promote_user_to_admin' : IDL.Func([UserId], [ApiEmptyResult], []),
    'update_deployment' : IDL.Func(
        [IDL.Text, DeploymentUpdate],
        [ApiEmptyResult],
        [],
      ),
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
