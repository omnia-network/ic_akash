import {X509CertificateData} from "@/lib/certificate";
import {wait} from "@/helpers/timer";
import {DeploymentState, MTlsCertificateData} from "@/declarations/backend.did";
import {extractDeploymentCreated} from "@/helpers/deployment";

const PROVIDER_PROXY_URL = "https://akash-provider-proxy.omnia-network.com/";

// from https://github.com/akash-network/cloudmos/blob/main/deploy-web/src/utils/deploymentUtils.ts
export const sendManifestToProvider = async (
  manifestUrl: string,
  manifest: string,
  certData: X509CertificateData,
) => {
  // wait for 5 sec for provider to have lease
  await wait(5_000);

  const method = "POST";
  const headers: HeadersInit = {
    "Content-Type": "application/json",
  }
  const body: BodyInit = JSON.stringify({
    method: "PUT",
    url: manifestUrl,
    certPem: certData.cert,
    keyPem: certData.priv_key,
    body: manifest,
    timeout: 60_000,
  });

  for (let i = 1; i <= 3; i++) {
    console.log("Try sending manifest #" + i);
    try {
      const response = await fetch(PROVIDER_PROXY_URL, {
        method,
        headers,
        body,
      });

      if (!response.ok) {
        const errorMessage = await response.text();
        throw new Error(`Response status: ${response.status} - response body: ${errorMessage}`);
      }

      // everything went fine, exit the loop
      break;

    } catch (err: any) {
      if (err.message?.includes && err.message.includes("no lease for deployment") && i < 3) {
        console.warn("Lease not found, retrying in 6 seconds...");
        await wait(6_000); // Waiting for 6 sec
      } else {
        throw new Error(err?.message || err);
      }
    }
  }
};

export const queryLeaseStatus = async (queryLeaseUrl: string, certData: X509CertificateData) => {
  const res = await fetch(PROVIDER_PROXY_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      method: "GET",
      url: queryLeaseUrl,
      certPem: certData.cert,
      keyPem: certData.priv_key,
    }),
  });

  if (!res.ok) {
    const err = await res.text();
    throw new Error(`Failed to query lease status: ${err}`);
  }

  return await res.json();
};

export const sendManifestToProviderFlow = async (deploymentState: DeploymentState, deploymentCreatedState: DeploymentState, cert: MTlsCertificateData) => {
  try {
    if ("LeaseCreated" in deploymentState) {
      const {manifest_sorted_json, dseq} = extractDeploymentCreated(deploymentCreatedState);

      const {provider_url} = deploymentState.LeaseCreated;

      const manifestUrl = new URL(
        `/deployment/${dseq}/manifest`,
        provider_url
      );

      await sendManifestToProvider(
        manifestUrl.toString(),
        manifest_sorted_json,
        cert!
      );
    } else {
      throw new Error("Deployment state is not in LeaseCreated state");
    }
  } catch (e) {
    console.error("Failed to send manifest to provider", e);
    throw e;
  }
}
