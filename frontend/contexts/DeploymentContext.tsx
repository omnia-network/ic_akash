import { type GetDeploymentsResult } from "@/declarations/backend.did";
import { getDeploymentCreatedDate } from "@/helpers/deployment";
import { extractOk, type OkType } from "@/helpers/result";
import { createX509, X509CertificateData } from "@/lib/certificate";
import { type BackendActor } from "@/services/backend";
import { createContext, useCallback, useContext, useState } from "react";
import { getCurrentUser } from "@/services/user";
import { completeDeployment, confirmDeployment, updateDeploymentState } from "@/services/deployment";

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

export const DeploymentProvider: React.FC<DeploymentProviderProps> = ({ children }) => {
  const [tlsCertificateData, setCertificateData] = useState<X509CertificateData | null>(null);
  const [deployments, setDeployments] = useState<Deployments>([]);

  const loadOrCreateCertificate = useCallback(async (actor: BackendActor): Promise<X509CertificateData | null> => {
    let certData = (await getCurrentUser(actor)).mtls_certificate[0] || null;

    if (!certData) {
      try {
        const akashAddress = extractOk(await actor.address());
        certData = await createX509(akashAddress);
        extractOk(await actor.create_certificate(certData));
      } catch (e) {
        console.error(e);
        alert("Failed to create certificate, see console for details");
        return null;
      }
    }

    setCertificateData(certData);

    return certData!;
  }, []);

  const completeDeployments = useCallback(async (actor: BackendActor, deployments: Deployments): Promise<number> => {
    let updatedCount = 0;

    for (const deployment of deployments) {
      const lastState = deployment.deployment.state_history[deployment.deployment.state_history.length - 1][1];
      if ("LeaseCreated" in lastState) {
        try {
          const deploymentCreatedState = deployment.deployment.state_history.find(([_, state]) => "DeploymentCreated" in state)![1];
          const cert = await loadOrCreateCertificate(actor);

          await confirmDeployment(
            lastState,
            deploymentCreatedState,
            cert!
          );
          await completeDeployment(
            actor,
            deployment.id,
          );

          updatedCount++;
        } catch (e) {
          console.error("Failed to update deployment:", e);

          try {
            const stepFailed = {
              FailedOnClient: {
                reason: JSON.stringify(e),
              },
            };
            await updateDeploymentState(actor, deployment.id, stepFailed);

            updatedCount++;
          } catch (e) {
            console.error("Failed to update deployment:", e);
            throw e;
          }
        }

      }
    }

    return updatedCount;
  }, [loadOrCreateCertificate]);

  const fetchDeployments = useCallback(async (actor: BackendActor) => {
    const res = await actor.get_deployments();

    const _deployments = extractOk(res);
    setDeployments(
      _deployments.sort((el1, el2) =>
        getDeploymentCreatedDate(el2.deployment).getTime() - getDeploymentCreatedDate(el1.deployment).getTime()
      )
    );
    console.log("deployments", _deployments);

    const updatedCount = await completeDeployments(actor, _deployments);
    if (updatedCount > 0) {
      return await fetchDeployments(actor);
    }
  }, [completeDeployments]);

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
