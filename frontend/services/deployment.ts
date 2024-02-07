import axios from "axios";
import { X509CertificateData } from "@/lib/certificate";
import { wait } from "@/helpers/timer";

export const sendManifestToProvider = async (
  providerUrl: string,
  manifest: string,
  certData: X509CertificateData,
) => {
  let response;
  for (let i = 1; i <= 3; i++) {
    console.log("Try #" + i);
    try {
      if (!response) {
        response = await axios.post(providerUrl, {
          method: "PUT",
          certPem: certData.cert,
          keyPem: certData.privKey,
          body: manifest,
          timeout: 60_000,
        });

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
