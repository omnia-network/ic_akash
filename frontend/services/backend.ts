import { Actor, ActorSubclass, HttpAgent, Identity } from "@dfinity/agent";
import { type _SERVICE, idlFactory } from "@/declarations/backend.did";
import { AccountIdentifier, LedgerCanister } from "@dfinity/ledger-icp";
import { Principal } from "@dfinity/principal";
import { extractOk } from "@/helpers/result";

export type BackendActor = ActorSubclass<_SERVICE>;

const network = process.env.DFX_NETWORK || "local";

export const icHost = network === "ic" ? "https://icp-api.io" : "http://127.0.0.1:4943";
export const canisterId = Principal.fromText(process.env.CANISTER_ID_BACKEND!);
export const icWsGatewayUrl = process.env.NEXT_PUBLIC_IC_WS_GATEWAY_URL!;

export const BACKEND_LEDGER_ACCOUNT_ID = AccountIdentifier.fromPrincipal({
  principal: canisterId,
});

export const createBackendAgent = async (identity: Identity) => {
  const agent = new HttpAgent({
    host: icHost,
    identity,
  });

  if (network !== "ic") {
    try {
      await agent.fetchRootKey();
    } catch (err) {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    }
  }

  return agent;
};

export const createBackendActor = async (identity: Identity) => {
  const agent = await createBackendAgent(identity);
  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId,
  });
};

export const transferE8sToBackend = async (ledger: LedgerCanister, amount: bigint, backendActor: BackendActor) => {
  const blockHeight = await ledger.transfer({
    to: BACKEND_LEDGER_ACCOUNT_ID,
    amount,
  });

  const res = await backendActor.update_akt_balance(blockHeight);
  extractOk(res);
};
