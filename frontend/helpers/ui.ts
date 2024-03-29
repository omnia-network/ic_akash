import { AccountIdentifier } from "@dfinity/ledger-icp";
import { Principal } from "@dfinity/principal";

const E8S_PER_ICP = 100_000_000;

export const shortPrincipal = (principal: Principal): string => {
  const principalText = principal.toText();
  return `${principalText.slice(0, 5)}...${principalText.slice(-3)}`;
};

export const shortAccountId = (accountId: AccountIdentifier): string => {
  const accountIdText = accountId.toHex();
  return `${accountIdText.slice(0, 5)}...${accountIdText.slice(-3)}`;
};

export const e8sToIcp = (e8s: bigint): number => {
  return Number(e8s / BigInt(E8S_PER_ICP)) + Number(e8s % BigInt(E8S_PER_ICP)) / E8S_PER_ICP;
};

export const displayIcp = (icp: number): string => {
  return icp.toLocaleString("en-US", { maximumFractionDigits: 2, minimumFractionDigits: 2 }) + " ICP";
};

export const displayE8sAsIcp = (e8s: bigint): string => {
  return displayIcp(e8sToIcp(e8s));
};
