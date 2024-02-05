import { Actor, HttpAgent } from "@dfinity/agent";
// @ts-ignore
import { idlFactory } from "@/declarations/backend.did";

const canisterId = process.env.CANISTER_ID_BACKEND!;

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

export const backend = Actor.createActor(idlFactory, {
  agent,
  canisterId,
});
