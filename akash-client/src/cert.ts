import * as x509 from "@peculiar/x509";
import * as fs from "fs";
import { arrayBufferToString, toBase64 } from "pvutils";

const ALG = {
  name: "ECDSA",
  hash: "SHA-256",
};
const CERTIFICATE_DURATION_DAYS = 365;
const CERT_FILE_PATH = "./data/cert/cert.json";

export type Pems = {
  csr: string;
  publicKey: string;
  privateKey: string;
}

const createCertificate = async (address: string): Promise<Pems> => {
  const keyPair = await crypto.subtle.generateKey({
    name: ALG.name,
    namedCurve: "P-256",
  }, true, ["sign"]);

  const commonName = `CN=${address}`;
  const notBefore = new Date();
  notBefore.setHours(0);
  notBefore.setMinutes(0);
  notBefore.setSeconds(0);
  const notAfter = new Date(notBefore.getTime() + CERTIFICATE_DURATION_DAYS * 24 * 60 * 60 * 1000);

  const cert = await x509.X509CertificateGenerator.create({
    serialNumber: (BigInt(Date.now()) * BigInt(1000)).toString(16),
    issuer: commonName,
    subject: commonName,
    notBefore,
    notAfter,
    signingAlgorithm: ALG,
    extensions: [
      new x509.BasicConstraintsExtension(true, undefined, true),
      new x509.ExtendedKeyUsageExtension([x509.ExtendedKeyUsage.clientAuth]),
      new x509.KeyUsagesExtension(x509.KeyUsageFlags.dataEncipherment | x509.KeyUsageFlags.keyEncipherment, true),
    ],
    publicKey: keyPair.publicKey,
    signingKey: keyPair.privateKey,
  });

  const certPem = cert.toString();
  const spki = await crypto.subtle.exportKey("spki", keyPair.publicKey);
  const pkcs8 = await crypto.subtle.exportKey("pkcs8", keyPair.privateKey);

  const pems = {
    csr: certPem,
    privateKey: `-----BEGIN PRIVATE KEY-----\n${formatPEM(
      toBase64(arrayBufferToString(pkcs8))
    )}\n-----END PRIVATE KEY-----`,
    publicKey: `-----BEGIN EC PUBLIC KEY-----\n${formatPEM(
      toBase64(arrayBufferToString(spki))
    )}\n-----END EC PUBLIC KEY-----`,
  };

  return pems;
};

// add line break every 64th character
const formatPEM = (pemString: string): string => {
  return pemString.replace(/(.{64})/g, "$1\n");
};

const saveCertificate = (certificate: Pems) => {
  const json = JSON.stringify(certificate);
  fs.writeFileSync(CERT_FILE_PATH, json);
};

const loadCertificate = (path: string): Pems => {
  const json = fs.readFileSync(path, "utf8");

  try {
    return JSON.parse(json);
  } catch (e) {
    throw new Error(`Could not parse certificate: ${e} `);
  }
};

export const loadOrCreateCertificate = async (address: string) => {
  if (fs.existsSync(CERT_FILE_PATH)) {
    return loadCertificate(CERT_FILE_PATH);
  }

  // if not, create a new one
  const certificate = await createCertificate(address);

  // save the certificate to the fixtures folder
  saveCertificate(certificate);
  return certificate;
};
