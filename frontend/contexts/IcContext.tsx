"use client";

import { _SERVICE } from "@/declarations/backend.did";
import { createBackendActor } from "@/services/backend";
import { ActorSubclass, Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";

type BackendActor = ActorSubclass<_SERVICE>;

type IcContextType = {
  identity: Identity | null;
  isLoggedIn: boolean;
  isLoading: boolean;
  login: () => Promise<void>;
  logout: () => Promise<void>;
  backendActor: BackendActor | null;
};

const IcContext = createContext<IcContextType | null>(null);

type IcProviderProps = {
  children?: React.ReactNode;
};

export const IcProvider: React.FC<IcProviderProps> = ({ children }) => {
  const [identity, setIdentity] = useState<Identity | null>(null);
  const [backendActor, setBackendActor] = useState<BackendActor | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const isLoggedIn = useMemo(() => identity !== null && !identity.getPrincipal().isAnonymous(), [identity]);

  const login = useCallback(async () => {
    if (isLoggedIn) {
      throw new Error("Already logged in");
    }

    setIsLoading(true);

    const authClient = await AuthClient.create();

    return new Promise<void>((resolve, reject) => {
      authClient.login({
        identityProvider: process.env.DFX_NETWORK === "ic"
          ? "https://identity.ic0.app"
          : `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943/`,
        maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000), // 7 days in nanoseconds
        onSuccess: () => {
          const identity = authClient.getIdentity();
          setIdentity(identity);
          const backendActor = createBackendActor(identity);
          setBackendActor(backendActor);

          setIsLoading(false);
          resolve();
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
    const getIdentity = async () => {
      setIsLoading(true);

      const authClient = await AuthClient.create();
      const identity = authClient.getIdentity();
      setIdentity(identity);
      const backendActor = createBackendActor(identity);
      setBackendActor(backendActor);

      setIsLoading(false);
    };

    getIdentity();
  }, []);

  return (
    <IcContext.Provider value={{
      identity,
      isLoggedIn,
      isLoading,
      login,
      logout,
      backendActor
    }}>
      {children}
    </IcContext.Provider>
  );
};

export const useIcContext = () => {
  return useContext(IcContext)!;
};
