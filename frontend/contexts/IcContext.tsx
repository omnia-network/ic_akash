"use client";

import { type DeploymentUpdateWsMessage, type _SERVICE } from "@/declarations/backend.did";
import { type BackendActor, canisterId, createBackendActor, icHost, icWsGatewayUrl } from "@/services/backend";
import { SignIdentity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { DelegationIdentity } from "@dfinity/identity";
import IcWebSocket, { createWsConfig } from "ic-websocket-js";
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";

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

type IcContextType = {
  identity: DelegationIdentity | null;
  isLoggedIn: boolean;
  isLoading: boolean;
  login: () => Promise<[DelegationIdentity, BackendActor]>;
  logout: () => Promise<void>;
  backendActor: BackendActor | null;
  wsActor: WebSocketActor | null;
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
  const [wsCallbacks, setWsCallbacks] = useState<WsCallbacks>({});
  const [isLoading, setIsLoading] = useState(true);
  const isLoggedIn = useMemo(() => identity !== null && !identity.getPrincipal().isAnonymous(), [identity]);

  const login = useCallback(async () => {
    if (isLoggedIn) {
      throw new Error("Already logged in");
    }

    setIsLoading(true);

    const authClient = await AuthClient.create({
      keyType: "ECDSA",
    });

    return new Promise<[DelegationIdentity, BackendActor]>((resolve, reject) => {
      authClient.login({
        identityProvider: process.env.DFX_NETWORK === "ic"
          ? "https://identity.ic0.app"
          : `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943/`,
        maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000), // 7 days in nanoseconds
        onSuccess: () => {
          const identity = authClient.getIdentity() as DelegationIdentity;
          setIdentity(identity);
          const backendActor = createBackendActor(identity);
          setBackendActor(backendActor);

          setIsLoading(false);
          resolve([identity, backendActor]);
        },
        onError: (err) => {
          console.error(err);

          setIsLoading(false);
          reject(err);
        },
      });
    })
  }, [isLoggedIn]);

  const logout = useCallback(async () => {
    if (!isLoggedIn) {
      throw new Error("Not logged in");
    }

    const authClient = await AuthClient.create();
    authClient.logout();

    setIdentity(null);
    setBackendActor(null);
  }, [isLoggedIn]);

  useEffect(() => {
    const loadIdentity = async () => {
      setIsLoading(true);

      const authClient = await AuthClient.create({
        keyType: "ECDSA",
      });
      const _identity = authClient.getIdentity();

      setIdentity(_identity as DelegationIdentity);
      const _backendActor = createBackendActor(_identity);
      setBackendActor(_backendActor);

      setIsLoading(false);
    };

    loadIdentity();
  }, []);

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
