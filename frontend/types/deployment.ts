import { CpuSize, MemorySize, StorageSize } from "@/declarations/backend.did";

export enum DeploymentTier {
  SMALL = "small",
  MEDIUM = "medium",
  LARGE = "large",
};

export type TierParams = {
  cpuSize: CpuSize;
  memorySize: MemorySize;
  storageSize: StorageSize;
  titleText: string;
  subtitleText: string;
  isEnabled: boolean;
};
