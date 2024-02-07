import { X509CertificateData } from "@/lib/certificate";
import { wait } from "@/helpers/timer";

export const sendManifestToProvider = async (
  providerUrl: string,
  manifest: string,
  certData: X509CertificateData,
) => {
  let response: Response | undefined = undefined;
  for (let i = 1; i <= 3; i++) {
    console.log("Try sending manifest #" + i);
    try {
      if (!response) {
        response = await fetch("https://providerproxy.cloudmos.io/", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            method: "PUT",
            url: providerUrl,
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
