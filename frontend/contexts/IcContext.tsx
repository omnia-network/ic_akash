import { Identity } from "@dfinity/agent";
import { createContext, useContext } from "react";

type IcContextType = {
  identity: Identity | null;
};

const IcContext = createContext<IcContextType | null>(null);

type IcProviderProps = {
  children?: React.ReactNode;
};

export const IcProvider: React.FC<IcProviderProps> = ({ children }) => {
  return (
    <IcContext.Provider value={{ identity: null }}>
      {children}
    </IcContext.Provider>
  );
};

export const useIcContext = () => {
  return useContext(IcContext)!;
};
