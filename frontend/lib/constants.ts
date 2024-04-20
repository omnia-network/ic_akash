import { DeploymentTier, type TierParams } from "@/types/deployment";

export const E8S_PER_ICP = 100_000_000;

export const DEPLOYMENT_TIERS: Record<DeploymentTier, TierParams> = {
  [DeploymentTier.SMALL]: {
    cpuSize: { Small: null },
    memorySize: { Small: null },
    storageSize: { Small: null },
    titleText: "Small",
    subtitleText: "0.5 vCPU | 0.5 GB RAM | 500 MB Storage",
    isEnabled: true,
  },
  [DeploymentTier.MEDIUM]: {
    cpuSize: { Medium: null },
    memorySize: { Medium: null },
    storageSize: { Medium: null },
    titleText: "Medium",
    subtitleText: "1 vCPU | 1 GB RAM | 5 GB Storage",
    isEnabled: false,
  },
  [DeploymentTier.LARGE]: {
    cpuSize: { Large: null },
    memorySize: { Large: null },
    storageSize: { Large: null },
    titleText: "Large",
    subtitleText: "2 vCPU | 2 GB RAM | 10 GB Storage",
    isEnabled: false,
  },
};
