import { E8S_PER_ICP } from "@/lib/constants";
import { AccountIdentifier } from "@dfinity/ledger-icp";
import { Principal } from "@dfinity/principal";

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

export const icpToE8s = (icp: number): bigint => {
  return BigInt(Math.round(icp * E8S_PER_ICP)); // should not overflow if icp is not too big
}

export const displayIcp = (icp: number, options?: Intl.NumberFormatOptions): string => {
  return icp.toLocaleString("en-US", {
    maximumFractionDigits: 2,
    minimumFractionDigits: 2,
    ...options
  }) + " ICP";
};

export const displayE8sAsIcp = (e8s: bigint, options?: Intl.NumberFormatOptions): string => {
  return displayIcp(e8sToIcp(e8s), options);
};
