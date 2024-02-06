import { type GetDeploymentsResult } from "@/declarations/backend.did";
import { type OkType, extractOk } from "@/helpers/result";
import { X509CertificateData, createX509, loadCertificate, saveCertificate } from "@/lib/certificate";
import { type BackendActor } from "@/services/backend";
import { DelegationIdentity } from "@dfinity/identity";
import { createContext, useCallback, useContext, useState } from "react";

export type Deployments = OkType<GetDeploymentsResult>;

type DeploymentContextType = {
  tlsCertificateData: X509CertificateData | null;
  loadOrCreateCertificate: (actor: BackendActor, identity: DelegationIdentity) => Promise<void>;
  deployments: Deployments;
  fetchDeployments: (actor: BackendActor) => Promise<void>;
};

const DeploymentContext = createContext<DeploymentContextType | null>(null);

type DeploymentProviderProps = {
  children?: React.ReactNode;
}

export const DeploymentProvider: React.FC<DeploymentProviderProps> = ({ children }) => {
  const [tlsCertificateData, setCertificateData] = useState<X509CertificateData | null>(null);
  const [deployments, setDeployments] = useState<Deployments>([]);

  const fetchDeployments = useCallback(async (actor: BackendActor) => {
    const res = await actor.get_deployments();

    const _deployments = extractOk(res);
    setDeployments(_deployments);
  }, []);

  const loadOrCreateCertificate = useCallback(async (actor: BackendActor, identity: DelegationIdentity) => {
    let certData = loadCertificate();
    if (certData) {
      setCertificateData(certData);
    } else {
      try {
        const akashAddress = extractOk(await actor.address());
        certData = await createX509(identity, akashAddress);
        extractOk(await actor.create_certificate(certData.cert, certData.pubKey));

        saveCertificate(certData);
      } catch (e) {
        console.error(e);
        alert("Failed to create certificate, see console for details");
      }
    }
  }, []);

  return (
    <DeploymentContext.Provider
      value={{
        tlsCertificateData,
        loadOrCreateCertificate,
        deployments,
        fetchDeployments,
      }}
    >
      {children}
    </DeploymentContext.Provider>
  );
};

export const useDeploymentContext = () => {
  return useContext(DeploymentContext)!;
};
