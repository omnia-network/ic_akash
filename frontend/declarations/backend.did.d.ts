import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ApiError { 'code' : number, 'message' : string }
export type CreateDeploymentResult = { 'Ok' : DeploymentId } |
  { 'Err' : ApiError };
export type CreateUserResult = { 'Ok' : UserId } |
  { 'Err' : ApiError };
export interface Deployment {
  'sdl' : string,
  'created_at' : TimestampNs,
  'user_id' : UserId,
  'state' : DeploymentState,
}
export type DeploymentId = string;
export type DeploymentState = { 'Initialized' : null } |
  { 'DeploymentCreated' : null } |
  { 'Closed' : null } |
  { 'Active' : null } |
  { 'LeaseCreated' : null };
export type EmptyResult = { 'Ok' : null } |
  { 'Err' : ApiError };
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
export interface _SERVICE {
  'create_deployment' : ActorMethod<[string], CreateDeploymentResult>,
  'create_user' : ActorMethod<[], CreateUserResult>,
  'get_deployment' : ActorMethod<[string], GetDeploymentResult>,
  'get_deployments' : ActorMethod<[], GetDeploymentsResult>,
  'get_user' : ActorMethod<[], GetUserResult>,
  'promote_user_to_admin' : ActorMethod<[UserId], EmptyResult>,
}
