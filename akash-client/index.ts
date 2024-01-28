import { loadOrCreateCertificate } from "./src/cert";
import * as https from "https";

const main = async () => {
  const { csr, publicKey, privateKey } = await loadOrCreateCertificate("akash1435dj4zjfz59rux9akthdcf6cy7h693fte6ge2");

  console.log("Cert PEM:", csr);
  console.log("Pub Key PEM:", publicKey);
  console.log("Priv Key PEM:", privateKey);

  console.log("Cert base64:", Buffer.from(csr).toString("base64"));
  console.log("Pub Key base64:", Buffer.from(publicKey).toString("base64"));

  const sortedJSONManifest = "";
  const path = `/deployment/${0}/manifest`;

  const uri = new URL("https://httpbin.org/put");
  const agent = new https.Agent({
    cert: csr,
    key: privateKey,
  });

  await new Promise((resolve, reject) => {
    const req = https.request({
      hostname: uri.hostname,
      port: uri.port,
      path: path,
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        "Accept": "application/json",
        "Content-Length": sortedJSONManifest.length,
      },
      agent: agent,
    }, (res) => {
      res.on("error", reject);

      res.on("data", chunk => {
        console.log("Response:", chunk.toString());
      });

      if (res.statusCode !== 200) {
        return reject(`Could not send manifest: ${res.statusCode}`);
      }

      resolve("ok");
    });

    req.on("error", reject);
    req.write(sortedJSONManifest);
    req.end();
  });
};

main();
