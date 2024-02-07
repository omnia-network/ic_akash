import { type Deployment, type DeploymentUpdate } from "@/declarations/backend.did";

export const extractDeploymentCreated = (dep: DeploymentUpdate) => {
  if ("DeploymentCreated" in dep) {
    return dep.DeploymentCreated;
  }

  throw new Error("DeploymentUpdate does not contain a DeploymentCreated");
};

export const extractDeaseCreated = (dep: DeploymentUpdate) => {
  if ("LeaseCreated" in dep) {
    return dep.LeaseCreated;
  }

  throw new Error("DeploymentUpdate does not contain a LeaseCreated");
};

export const extractDeploymentClosed = (dep: DeploymentUpdate) => {
  if ("Closed" in dep) {
    return dep.Closed;
  }

  throw new Error("DeploymentUpdate does not contain a Closed");
};

export const extractDeploymentFailed = (dep: DeploymentUpdate) => {
  if ("Failed" in dep) {
    return dep.Failed;
  }

  throw new Error("DeploymentUpdate does not contain a Failed");
};

export const extractDeploymentActive = (dep: DeploymentUpdate) => {
  if ("Active" in dep) {
    return dep.Active;
  }

  throw new Error("DeploymentUpdate does not contain an Active");
};

export const extractDeploymentInitialized = (dep: DeploymentUpdate) => {
  if ("Initialized" in dep) {
    return dep.Initialized;
  }

  throw new Error("DeploymentUpdate does not contain an Initialized");
};

export const isDeploymentInState = (dep: Deployment, state: string) => {
  return dep.state_history.findIndex(([_, update]) => getDeploymentUpdateName(update) === state) !== -1;
};

export const getLastDeploymentUpdate = (dep: Deployment) => {
  return dep.state_history[dep.state_history.length - 1][1];
};

export const getDeploymentUpdateName = (dep: DeploymentUpdate) => {
  return Object.keys(dep)[0];
}

export const isDeploymentClosed = (dep: Deployment) => {
  return isDeploymentInState(dep, "Closed");
};

export const isDeploymentFailed = (dep: Deployment) => {
  return isDeploymentInState(dep, "Failed");
};

export const getDeploymentCreatedDate = (dep: Deployment) => {
  const initializedStep = dep.state_history.find(([_, update]) => getDeploymentUpdateName(update) === "Initialized");

  if (!initializedStep) {
    throw new Error("Deployment does not contain an Initialized step");
  }

  return getDeploymentUpdateDate(initializedStep);
};

export const getDeploymentUpdateDate = ([timestampNs, _]: [bigint, DeploymentUpdate]) => {
  return new Date(Number(timestampNs / BigInt(1_000_000)));
}
