import { type Deployment, type DeploymentState } from "@/declarations/backend.did";

export const extractDeploymentCreated = (dep: DeploymentState) => {
  if ("DeploymentCreated" in dep) {
    return dep.DeploymentCreated;
  }

  throw new Error("DeploymentState does not contain a DeploymentCreated");
};

export const extractLeaseCreated = (dep: DeploymentState) => {
  if ("LeaseCreated" in dep) {
    return dep.LeaseCreated;
  }

  throw new Error("DeploymentState does not contain a LeaseCreated");
};

export const extractDeploymentClosed = (dep: DeploymentState) => {
  if ("Closed" in dep) {
    return dep.Closed;
  }

  throw new Error("DeploymentState does not contain a Closed");
};

export const extractDeploymentFailed = (dep: DeploymentState) => {
  if ("Failed" in dep) {
    return dep.Failed;
  }

  throw new Error("DeploymentState does not contain a Failed");
};

export const extractDeploymentActive = (dep: DeploymentState) => {
  if ("Active" in dep) {
    return dep.Active;
  }

  throw new Error("DeploymentState does not contain an Active");
};

export const extractDeploymentInitialized = (dep: DeploymentState) => {
  if ("Initialized" in dep) {
    return dep.Initialized;
  }

  throw new Error("DeploymentState does not contain an Initialized");
};

export const isDeploymentInState = (dep: Deployment, state: string) => {
  return dep.state_history.findIndex(([_, update]) => getDeploymentStateName(update) === state) !== -1;
};

export const getLastDeploymentState = (dep: Deployment) => {
  return dep.state_history[dep.state_history.length - 1][1];
};

export const getDeploymentStateName = (dep: DeploymentState) => {
  return Object.keys(dep)[0];
}

export const isDeploymentClosed = (dep: Deployment) => {
  return isDeploymentInState(dep, "Closed");
};

export const isDeploymentFailedOnCanister = (dep: Deployment) => {
  return isDeploymentInState(dep, "FailedOnCanister");
};

export const isDeploymentFailedOnClient = (dep: Deployment) => {
  return isDeploymentInState(dep, "FailedOnClient");
};

export const isDeploymentFailed = (dep: Deployment) => {
  return isDeploymentFailedOnCanister(dep) || isDeploymentFailedOnClient(dep);
};

export const isDeploymentActive = (dep: Deployment) => {
  return isDeploymentInState(dep, "Active");
}

export const getDeploymentCreatedDate = (dep: Deployment) => {
  const initializedStep = dep.state_history.find(([_, update]) => getDeploymentStateName(update) === "Initialized");

  if (!initializedStep) {
    throw new Error("Deployment does not contain an Initialized step");
  }

  return getDeploymentStateDate(initializedStep);
};

export const getDeploymentStateDate = ([timestampNs, _]: [bigint, DeploymentState]) => {
  return new Date(Number(timestampNs / BigInt(1_000_000)));
}
