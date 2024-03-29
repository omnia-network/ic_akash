import { X509CertificateData } from "@/lib/certificate";
import { wait } from "@/helpers/timer";

const PROVIDER_PROXY_URL = "https://akash-provider-proxy.omnia-network.com/";

// from https://github.com/akash-network/cloudmos/blob/main/deploy-web/src/utils/deploymentUtils.ts
export const sendManifestToProvider = async (
  manifestUrl: string,
  manifest: string,
  certData: X509CertificateData,
) => {
  // wait for 5 sec for provider to have lease
  await wait(5_000);

  let response: Response | undefined = undefined;
  for (let i = 1; i <= 3; i++) {
    console.log("Try sending manifest #" + i);
    try {
      if (!response) {
        response = await fetch(PROVIDER_PROXY_URL, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            method: "PUT",
            url: manifestUrl,
            certPem: certData.cert,
            keyPem: certData.privKey,
            body: manifest,
            timeout: 60_000,
          }),
        });

        if (!response.ok) {
          throw new Error(await response.text());
        }

        i = 3;
      }
    } catch (err: any) {
      if (err.includes && err.includes("no lease for deployment") && i < 3) {
        console.log("Lease not found, retrying...");
        await wait(6_000); // Waiting for 6 sec
      } else {
        throw new Error(err?.response?.data || err);
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
      keyPem: certData.privKey,
    }),
  });

  if (!res.ok) {
    const err = await res.text();
    throw new Error(`Failed to query lease status: ${err}`);
  }

  return await res.json();
};
