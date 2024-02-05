import { Actor, HttpAgent } from "@dfinity/agent";
// @ts-ignore
import { type _SERVICE, idlFactory } from "@/declarations/backend.did.js";

const canisterId = process.env.CANISTER_ID_BACKEND!;

const createActor = () => {
  const agent = new HttpAgent({
    host: process.env.NEXT_PUBLIC_IC_HOST,
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
}


export const backend = createActor();
