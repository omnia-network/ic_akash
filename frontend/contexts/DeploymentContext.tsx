import {type GetDeploymentsResult} from "@/declarations/backend.did";
import {getDeploymentCreatedDate} from "@/helpers/deployment";
import {extractOk, type OkType} from "@/helpers/result";
import {createX509, X509CertificateData} from "@/lib/certificate";
import {type BackendActor} from "@/services/backend";
import {createContext, useCallback, useContext, useState} from "react";
import {getCurrentUser, setUserMutualTlsCertificate} from "@/services/user";

export type Deployments = OkType<GetDeploymentsResult>;

type DeploymentContextType = {
  tlsCertificateData: X509CertificateData | null;
  loadOrCreateCertificate: (actor: BackendActor) => Promise<X509CertificateData | null>;
  deployments: Deployments;
  fetchDeployments: (actor: BackendActor) => Promise<void>;
};

const DeploymentContext = createContext<DeploymentContextType | null>(null);

type DeploymentProviderProps = {
  children?: React.ReactNode;
}

export const DeploymentProvider: React.FC<DeploymentProviderProps> = ({children}) => {
  const [tlsCertificateData, setCertificateData] = useState<X509CertificateData | null>(null);
  const [deployments, setDeployments] = useState<Deployments>([]);

  const fetchDeployments = useCallback(async (actor: BackendActor) => {
    const res = await actor.get_deployments();

    const _deployments = extractOk(res);
    setDeployments(
      _deployments.sort((el1, el2) =>
        getDeploymentCreatedDate(el2.deployment).getTime() - getDeploymentCreatedDate(el1.deployment).getTime()
      )
    );

    console.log("deployments", _deployments);
  }, []);

  const loadOrCreateCertificate = useCallback(async (actor: BackendActor): Promise<X509CertificateData | null> => {
    const mutualTlsCertificateStringified = (await getCurrentUser(actor)).mutual_tls_certificate;
    let certData = null;
    if (mutualTlsCertificateStringified) {
      certData = JSON.parse(mutualTlsCertificateStringified);
    }
    
    if (!certData) {
      try {
        const akashAddress = extractOk(await actor.address());
        certData = await createX509(akashAddress);
        extractOk(await actor.create_certificate(
          Buffer.from(certData.cert, "utf-8").toString("base64"),
          Buffer.from(certData.pubKey, "utf-8").toString("base64"),
        ));

        await setUserMutualTlsCertificate(actor, JSON.stringify(certData));
      } catch (e) {
        console.error(e);
        alert("Failed to create certificate, see console for details");
        return null;
      }
    }

    setCertificateData(certData);

    return certData!;
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
