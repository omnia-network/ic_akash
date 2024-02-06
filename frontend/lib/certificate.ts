import { DelegationIdentity } from "@dfinity/identity";
import * as x509 from "@peculiar/x509";
import { AsnConvert } from "@peculiar/asn1-schema";
import * as asn1X509 from "@peculiar/asn1-x509";
import { BufferSourceConverter, Convert } from "pvtsutils";
import { container } from "tsyringe";

export type X509CertificateData = {
  /**
   * Base64 encoded certificate
   */
  cert: string;
  /**
   * Base64 encoded public key
   */
  pubKey: string;
};

const SIGNING_ALGORITHM = {
  name: "ECDSA",
  hash: "SHA-256",
  publicExponent: new Uint8Array([1, 0, 1]),
  modulusLength: 2048,
};

const CERTIFICATE_DURATION_DAYS = 365;

const CERTIFICATE_STORAGE_KEY = "x509-certificate";

// from https://github.com/PeculiarVentures/x509/blob/master/src/x509_cert_generator.ts
export const createX509 = async (identity: DelegationIdentity, canisterAkashAddress: String): Promise<X509CertificateData> => {
  const serialNumber = BufferSourceConverter.toUint8Array(Convert.FromHex((BigInt(new Date().getTime()) * BigInt(1000)).toString(16)));
  const commonName = `/CN=${canisterAkashAddress}`;
  const subject = new x509.Name(commonName);
  const issuer = new x509.Name(commonName);
  const notBefore = new Date();
  notBefore.setHours(0);
  notBefore.setMinutes(0);
  notBefore.setSeconds(0);
  const notAfter = new Date(notBefore.getTime() + CERTIFICATE_DURATION_DAYS * 24 * 60 * 60 * 1000);
  const extensions = [
    new x509.BasicConstraintsExtension(true, undefined, true),
    new x509.ExtendedKeyUsageExtension([x509.ExtendedKeyUsage.clientAuth]),
    new x509.KeyUsagesExtension(x509.KeyUsageFlags.keyEncipherment | x509.KeyUsageFlags.dataEncipherment, true),
  ];
  const spki = identity.getPublicKey().derKey!;

  const asnX509 = new asn1X509.Certificate({
    tbsCertificate: new asn1X509.TBSCertificate({
      version: asn1X509.Version.v3,
      serialNumber,
      validity: new asn1X509.Validity({
        notBefore,
        notAfter,
      }),
      extensions: new asn1X509.Extensions(extensions.map(o => AsnConvert.parse(o.rawData, asn1X509.Extension))),
      subjectPublicKeyInfo: AsnConvert.parse(spki, asn1X509.SubjectPublicKeyInfo),
      subject: AsnConvert.parse(subject.toArrayBuffer(), asn1X509.Name),
      issuer: AsnConvert.parse(issuer.toArrayBuffer(), asn1X509.Name),
    }),
  });

  const algProv = container.resolve<x509.AlgorithmProvider>(x509.diAlgorithmProvider);
  asnX509.tbsCertificate.signature = asnX509.signatureAlgorithm = algProv.toAsnAlgorithm(SIGNING_ALGORITHM);

  // Sign 
  const tbs = AsnConvert.serialize(asnX509.tbsCertificate);
  const signatureValue = await identity.sign(tbs);

  // Convert WebCrypto signature to ASN.1 format
  const signatureFormatters = container.resolveAll<x509.IAsnSignatureFormatter>(x509.diAsnSignatureFormatter).reverse();
  let asnSignature: ArrayBuffer | null = null;
  for (const signatureFormatter of signatureFormatters) {
    asnSignature = signatureFormatter.toAsnSignature(SIGNING_ALGORITHM, signatureValue);
    if (asnSignature) {
      break;
    }
  }
  if (!asnSignature) {
    throw Error("Cannot convert ASN.1 signature value to WebCrypto format");
  }

  asnX509.signatureValue = asnSignature;

  const cert = new x509.X509Certificate(AsnConvert.serialize(asnX509));

  // TODO: verify that the certificate signature is valid
  // even after signing in again with the Internet Identity

  const encoder = new TextEncoder();

  const certData: X509CertificateData = {
    cert: Convert.ToBase64(encoder.encode(cert.toString("pem"))),
    pubKey: Convert.ToBase64(encoder.encode(cert.publicKey.toString("pem").replaceAll("PUBLIC KEY", "EC PUBLIC KEY"))),
  };

  return certData;
};

export const loadCertificate = (): X509CertificateData | null => {
  const cert = localStorage.getItem(CERTIFICATE_STORAGE_KEY);

  if (!cert) {
    return null;
  }

  return JSON.parse(cert);
};

export const saveCertificate = (cert: X509CertificateData) => {
  localStorage.setItem(CERTIFICATE_STORAGE_KEY, JSON.stringify(cert));
}
