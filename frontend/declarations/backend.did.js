export const idlFactory = ({ IDL }) => {
  const DeploymentId = IDL.Text;
  const ApiError = IDL.Record({ 'code' : IDL.Nat16, 'message' : IDL.Text });
  const CreateDeploymentResult = IDL.Variant({
    'Ok' : DeploymentId,
    'Err' : ApiError,
  });
  const UserId = IDL.Principal;
  const CreateUserResult = IDL.Variant({ 'Ok' : UserId, 'Err' : ApiError });
  const TimestampNs = IDL.Nat64;
  const DeploymentState = IDL.Variant({
    'Initialized' : IDL.Null,
    'DeploymentCreated' : IDL.Null,
    'Closed' : IDL.Null,
    'Active' : IDL.Null,
    'LeaseCreated' : IDL.Null,
  });
  const Deployment = IDL.Record({
    'sdl' : IDL.Text,
    'created_at' : TimestampNs,
    'user_id' : UserId,
    'state' : DeploymentState,
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
  const EmptyResult = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : ApiError });
  return IDL.Service({
    'create_deployment' : IDL.Func([IDL.Text], [CreateDeploymentResult], []),
    'create_user' : IDL.Func([], [CreateUserResult], []),
    'get_deployment' : IDL.Func([IDL.Text], [GetDeploymentResult], ['query']),
    'get_deployments' : IDL.Func([], [GetDeploymentsResult], ['query']),
    'get_user' : IDL.Func([], [GetUserResult], ['query']),
    'promote_user_to_admin' : IDL.Func([UserId], [EmptyResult], []),
  });
};
export const init = ({ IDL }) => { return [IDL.Bool]; };
