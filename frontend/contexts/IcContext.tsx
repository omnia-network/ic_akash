"use client";

import { type DeploymentUpdateWsMessage, type _SERVICE } from "@/declarations/backend.did";
import { type BackendActor, canisterId, createBackendActor, icHost, icWsGatewayUrl, createBackendAgent } from "@/services/backend";
import { SignIdentity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { DelegationIdentity } from "@dfinity/identity";
import { AccountIdentifier, LedgerCanister } from "@dfinity/ledger-icp";
import { Principal } from "@dfinity/principal";
import IcWebSocket, { createWsConfig } from "ic-websocket-js";
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";

// there values are hard coded in dfx.json
const LEDGER_CANISTER_ID = Principal.fromText("ryjl3-tyaaa-aaaaa-aaaba-cai");
const INTERNET_IDENTITY_CANISTER_ID = Principal.fromText("rdmx6-jaaaa-aaaaa-aaadq-cai");

type WebSocketActor = IcWebSocket<_SERVICE, DeploymentUpdateWsMessage>;
export type OnWsOpenCallback = NonNullable<WebSocketActor["onopen"]>;
export type OnWsMessageCallback = NonNullable<WebSocketActor["onmessage"]>;
export type OnWsCloseCallback = NonNullable<WebSocketActor["onclose"]>;
export type OnWsErrorCallback = NonNullable<WebSocketActor["onerror"]>;

type WsCallbacks = {
  onOpen?: OnWsOpenCallback;
  onMessage?: OnWsMessageCallback;
  onClose?: OnWsCloseCallback;
  onError?: OnWsErrorCallback;
};

type LedgerData = {
  accountId: AccountIdentifier | null;
  balance: bigint | null;
  isLoading: boolean;
};

const defaultLedgerData: LedgerData = {
  accountId: null,
  balance: null,
  isLoading: false,
};

type IcContextType = {
  identity: DelegationIdentity | null;
  isLoggedIn: boolean;
  isLoading: boolean;
  login: () => Promise<[DelegationIdentity, BackendActor]>;
  logout: () => Promise<void>;
  backendActor: BackendActor | null;
  wsActor: WebSocketActor | null;
  ledgerCanister: LedgerCanister | null;
  refreshLedgerData: () => Promise<void>;
  ledgerData: LedgerData;
  openWs: (wsCallbacks: WsCallbacks) => void;
  setWsCallbacks: (wsCallbacks: WsCallbacks) => void;
  closeWs: () => void;
};

const IcContext = createContext<IcContextType | null>(null);

type IcProviderProps = {
  children?: React.ReactNode;
};

export const IcProvider: React.FC<IcProviderProps> = ({ children }) => {
  const [identity, setIdentity] = useState<DelegationIdentity | null>(null);
  const [backendActor, setBackendActor] = useState<BackendActor | null>(null);
  const [wsActor, setWsActor] = useState<WebSocketActor | null>(null);
  const [ledgerCanister, setLedgerCanister] = useState<LedgerCanister | null>(null);
  const [ledgerData, setLedgerData] = useState<LedgerData>(defaultLedgerData);
  const [wsCallbacks, setWsCallbacks] = useState<WsCallbacks>({});
  const [isLoading, setIsLoading] = useState(true);
  const isLoggedIn = useMemo(() => identity !== null && !identity.getPrincipal().isAnonymous(), [identity]);

  const refreshLedgerData = useCallback(async () => {
    if (!ledgerCanister || !identity) {
      throw new Error("No ledger canister or identity");
    }

    const accountId = AccountIdentifier.fromPrincipal({ principal: identity.getPrincipal() });

    setLedgerData((prev) => ({ ...prev, accountId, isLoading: true }));
    const balance = await ledgerCanister.accountBalance({ accountIdentifier: accountId });
    setLedgerData((prev) => ({ ...prev, accountId, balance, isLoading: false }));
  }, [ledgerCanister, identity]);

  const setContext = useCallback(async (authClient: AuthClient): Promise<[DelegationIdentity, BackendActor]> => {
    const id = authClient.getIdentity() as DelegationIdentity;
    setIdentity(id);

    const actor = createBackendActor(id);
    setBackendActor(actor);

    const ledger = LedgerCanister.create({
      agent: createBackendAgent(id),
      canisterId: LEDGER_CANISTER_ID,
    });
    setLedgerCanister(ledger);

    const accountId = AccountIdentifier.fromPrincipal({ principal: id.getPrincipal() });
    setLedgerData((prev) => ({ ...prev, accountId, isLoading: true }));
    const balance = await ledger.accountBalance({ accountIdentifier: accountId });
    setLedgerData((prev) => ({ ...prev, accountId, balance, isLoading: false }));

    return [id, actor];
  }, []);

  const login = useCallback(async () => {
    if (isLoggedIn) {
      throw new Error("Already logged in");
    }

    setIsLoading(true);

    const authClient = await AuthClient.create();

    return new Promise<[DelegationIdentity, BackendActor]>((resolve, reject) => {
      authClient.login({
        identityProvider: process.env.DFX_NETWORK === "ic"
          ? "https://identity.ic0.app"
          : `http://${INTERNET_IDENTITY_CANISTER_ID.toText()}.localhost:4943/`,
        maxTimeToLive: BigInt(7 * 24 * 60 * 60) * BigInt(1000 * 1000 * 1000), // 7 days in nanoseconds
        onSuccess: async () => {
          const [id, actor] = await setContext(authClient);

          setIsLoading(false);
          resolve([id, actor]);
        },
        onError: (err) => {
          console.error(err);

          setIsLoading(false);
          reject(err);
        },
      });
    })
  }, [isLoggedIn, setContext]);

  const logout = useCallback(async () => {
    if (!isLoggedIn) {
      throw new Error("Not logged in");
    }

    const authClient = await AuthClient.create();
    authClient.logout();

    setIdentity(null);
    setBackendActor(null);
    setLedgerCanister(null);
    setLedgerData(defaultLedgerData);
  }, [isLoggedIn]);

  useEffect(() => {
    (async () => {
      setIsLoading(true);

      const authClient = await AuthClient.create();
      await setContext(authClient);

      setIsLoading(false);
    })();
  }, [setContext]);

  const updateWsCallbacks = useCallback((wsActor: WebSocketActor, inputWsCallbacks: WsCallbacks) => {
    wsActor.onopen = inputWsCallbacks.onOpen || null;
    wsActor.onmessage = inputWsCallbacks.onMessage || null;
    wsActor.onclose = inputWsCallbacks.onClose || null;
    wsActor.onerror = inputWsCallbacks.onError || null;
  }, []);

  const openWs = useCallback((inputWsCallbacks: WsCallbacks) => {
    if (!isLoggedIn) {
      throw new Error("Not logged in");
    }

    const wsConfig = createWsConfig({
      canisterId,
      networkUrl: icHost,
      canisterActor: backendActor!,
      identity: identity as SignIdentity,
    });

    const ws = new IcWebSocket(icWsGatewayUrl, undefined, wsConfig);
    updateWsCallbacks(ws, inputWsCallbacks);
    setWsCallbacks(inputWsCallbacks);

    setWsActor(ws);
  }, [isLoggedIn, backendActor, identity, updateWsCallbacks]);

  const closeWs = useCallback(() => {
    if (wsActor) {
      wsActor.close();
      setWsActor(null);
      setWsCallbacks({});
    }
  }, [wsActor]);

  useEffect(() => {
    if (wsActor) {
      updateWsCallbacks(wsActor, wsCallbacks);
    }
  }, [wsActor, wsCallbacks, updateWsCallbacks]);

  return (
    <IcContext.Provider value={{
      identity,
      isLoggedIn,
      isLoading,
      login,
      logout,
      backendActor,
      wsActor,
      ledgerCanister,
      ledgerData,
      refreshLedgerData,
      openWs,
      setWsCallbacks,
      closeWs,
    }}>
      {children}
    </IcContext.Provider>
  );
};

export const useIcContext = () => {
  return useContext(IcContext)!;
};
