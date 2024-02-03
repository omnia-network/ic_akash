import { loadOrCreateCertificate } from "./src/cert";
import * as https from "https";

const main = async () => {
  const { csr, publicKey, privateKey } = await loadOrCreateCertificate("akash18xlfcnadxvadgu8tpamnk5etmgledgy7r35rm9");

  console.log("Cert PEM:", csr);
  console.log("Pub Key PEM:", publicKey);
  console.log("Priv Key PEM:", privateKey);

  console.log("Cert base64:", Buffer.from(csr).toString("base64"));
  console.log("Pub Key base64:", Buffer.from(publicKey).toString("base64"));

  const bidIdDSeq = 3100874;
  const sortedJSONManifest = `[{"name":"dcloud","services":[{"args":null,"command":["/ic-ws-gateway/ic_websocket_gateway","--gateway-address","0.0.0.0:8080","--ic-network-url","https://icp-api.io","--polling-interval","400"],"count":1,"env":null,"expose":[{"endpointSequenceNumber":0,"externalPort":80,"global":true,"hosts":["akash-gateway.icws.io"],"httpOptions":{"maxBodySize":1048576,"nextCases":["error","timeout"],"nextTimeout":0,"nextTries":3,"readTimeout":60000,"sendTimeout":60000},"ip":"","port":8080,"proto":"TCP","service":""}],"image":"omniadevs/ic-websocket-gateway","name":"ic-websocket-gateway","params":null,"resources":{"cpu":{"units":{"val":"500"}},"endpoints":[{"sequence_number":0}],"gpu":{"units":{"val":"0"}},"id":1,"memory":{"size":{"val":"536870912"}},"storage":[{"name":"default","size":{"val":"536870912"}}]}}]}]`;
  const path = `/deployment/${bidIdDSeq}/manifest`;

  const uri = new URL(path, "https://provider.nxql.bhuceramics.store:8443");
  const agent = new https.Agent({
    cert: csr,
    key: privateKey,
    rejectUnauthorized: false,
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
      timeout: 60_000,
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

  const leaseStatus = await queryLeaseStatus(bidIdDSeq, uri, agent);
  console.log("Lease status:", JSON.stringify(leaseStatus, null, 2));
};

const queryLeaseStatus = async (dseq: number, providerUri: URL, agent: https.Agent) => {
  const leasePath = `/lease/${dseq}/${1}/${1}/status`;

  const uri = new URL(providerUri);

  return new Promise((resolve, reject) => {
    const req = https.request({
      hostname: uri.hostname,
      port: uri.port,
      path: leasePath,
      method: "GET",
      agent: agent,
    }, (res) => {
      if (res.statusCode !== 200) {
        return reject(`Could not query lease status: ${res.statusCode}`);
      }

      let data = "";

      res.on("data", chunk => data += chunk);
      res.on("end", () => resolve(JSON.parse(data)));
    });

    req.on("error", reject);
    req.end();
  });
}

main();
