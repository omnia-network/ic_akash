import { Actor, ActorSubclass, HttpAgent, Identity } from "@dfinity/agent";
// @ts-ignore
import { type _SERVICE, idlFactory } from "@/declarations/backend.did";

export type BackendActor = ActorSubclass<_SERVICE>;

export const icHost = process.env.NEXT_PUBLIC_IC_HOST!;
export const canisterId = process.env.CANISTER_ID_BACKEND!;
export const icWsGatewayUrl = process.env.NEXT_PUBLIC_IC_WS_GATEWAY_URL!;

export const createBackendActor = (identity: Identity) => {
  const agent = new HttpAgent({
    host: icHost,
    identity,
  });

  if (process.env.DFX_NETWORK !== "ic") {
    agent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    });
  }

  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId,
  });
};
