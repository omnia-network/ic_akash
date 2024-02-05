import { Identity } from "@dfinity/agent";
import { createContext, useContext } from "react";

type AuthContextType = {
  identity: Identity | null;
};

const AuthContext = createContext<AuthContextType | null>(null);

type AuthProviderProps = {
  children?: React.ReactNode;
};

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  return (
    <AuthContext.Provider value={{ identity: null }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuthContext = () => {
  return useContext(AuthContext)!;
};
