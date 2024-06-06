import { MTlsCertificateData } from "@/declarations/backend.did";
import * as x509 from "@peculiar/x509";

export type X509CertificateData = MTlsCertificateData;

const SIGNING_ALGORITHM = {
  name: "ECDSA",
  hash: "SHA-256",
};

const CERTIFICATE_DURATION_DAYS = 365;

export const createX509 = async (canisterAkashAddress: String): Promise<X509CertificateData> => {
  const keyPair = await window.crypto.subtle.generateKey(
    {
      name: SIGNING_ALGORITHM.name,
      namedCurve: 'P-256',
    },
    true,
    ['sign'],
  );

  const commonName = `/CN=${canisterAkashAddress}`;
  const notBefore = new Date();
  notBefore.setHours(0);
  notBefore.setMinutes(0);
  notBefore.setSeconds(0);
  const notAfter = new Date(notBefore.getTime() + CERTIFICATE_DURATION_DAYS * 24 * 60 * 60 * 1000);

  const cert = await x509.X509CertificateGenerator.create({
    serialNumber: (BigInt(new Date().getTime()) * BigInt(1000)).toString(16),
    issuer: commonName,
    subject: commonName,
    notBefore,
    notAfter,
    extensions: [
      new x509.KeyUsagesExtension(x509.KeyUsageFlags.keyEncipherment | x509.KeyUsageFlags.dataEncipherment, true),
      new x509.ExtendedKeyUsageExtension([x509.ExtendedKeyUsage.clientAuth]),
      new x509.BasicConstraintsExtension(true, undefined, true),
    ],
    signingAlgorithm: SIGNING_ALGORITHM,
    publicKey: keyPair.publicKey,
    signingKey: keyPair.privateKey,
  });

  const spki = await window.crypto.subtle.exportKey("spki", keyPair.publicKey);
  const pkcs8 = await window.crypto.subtle.exportKey("pkcs8", keyPair.privateKey);

  const pub_key = `-----BEGIN EC PUBLIC KEY-----\n${formatPEM(
    Buffer.from(spki).toString("base64")
  )}\n-----END EC PUBLIC KEY-----`
  const priv_key = `-----BEGIN PRIVATE KEY-----\n${formatPEM(
    Buffer.from(pkcs8).toString("base64")
  )}\n-----END PRIVATE KEY-----`;

  const certData: X509CertificateData = {
    cert: cert.toString("pem"),
    pub_key,
    priv_key,
  };

  return certData;
};

const formatPEM = (pemString: string): string => {
  return pemString.replace(/(.{64})/g, "$1\n");
};
