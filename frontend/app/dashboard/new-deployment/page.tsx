"use client";

import { BackButton } from "@/components/back-button";
import { LoadingButton } from "@/components/loading-button";
import { useDeploymentContext } from "@/contexts/DeploymentContext";
import { type OnWsMessageCallback, type OnWsOpenCallback, useIcContext } from "@/contexts/IcContext";
import { DeploymentUpdate } from "@/declarations/backend.did";
import { extractDeploymentCreated } from "@/helpers/deployment";
import { extractOk } from "@/helpers/result";
import { sendManifestToProvider } from "@/services/deployment";
import { useCallback, useEffect, useState } from "react";

export default function NewDeployment() {
  const { identity, backendActor, openWs, setWsCallbacks } = useIcContext();
  const { tlsCertificateData, loadOrCreateCertificate, fetchDeployments } = useDeploymentContext();
  const [isLoading, setIsLoading] = useState(false);
  const [isDeploying, setIsDeploying] = useState(false);
  const [deploymentSteps, setDeploymentSteps] = useState<Array<DeploymentUpdate>>([]);

  const onWsOpen: OnWsOpenCallback = useCallback(async () => {
    console.log("ws open");

    setIsDeploying(true);
    try {
      await loadOrCreateCertificate(backendActor!, identity!);

      const res = await backendActor!.create_test_deployment();
      const deploymentId = extractOk(res);

      console.log("deployment id", deploymentId);

      setDeploymentSteps([
        { Initialized: null }
      ]);

    } catch (e) {
      console.error(e);
      alert("Failed to create deployment, see console for details");
    }

    setIsLoading(false);
  }, [backendActor, identity, loadOrCreateCertificate]);

  const onWsMessage: OnWsMessageCallback = useCallback(async (ev) => {
    console.log("ws message");

    const deploymentUpdate = ev.data;
    console.log("deployment update", deploymentUpdate);
    setDeploymentSteps((prev) => [...prev, deploymentUpdate.update]);

    if ("Failed" in deploymentUpdate.update) {
      const err = deploymentUpdate.update.Failed.reason;
      console.error("Failed to deploy", err);
      alert("Failed to deploy, see console for details");
      return;
    }

    let leaseCreated = false;

    try {
      if ("LeaseCreated" in deploymentUpdate.update) {
        const { manifest_sorted_json } = extractDeploymentCreated(deploymentSteps.find((el) => el.hasOwnProperty("DeploymentCreated"))!);
        const { provider_url } = deploymentUpdate.update.LeaseCreated;

        await sendManifestToProvider(provider_url, manifest_sorted_json, tlsCertificateData!);

        leaseCreated = true;
      }
    } catch (e) {
      console.error(e);

      try {
        const stepFailed = {
          Failed: {
            reason: JSON.stringify(e),
          },
        };

        setDeploymentSteps((prev) => [...prev, stepFailed]);

        extractOk(await backendActor!.update_deployment(deploymentUpdate.id, stepFailed));
      } catch (e) {
        console.error("Failed to update deployment", e);
      }

      alert("Failed to send manifest to provider, see console for details");
      setIsDeploying(false);
    }

    try {
      if (leaseCreated) {
        const stepActive = {
          Active: null,
        };

        setDeploymentSteps((prev) => [...prev, stepActive]);

        extractOk(await backendActor!.update_deployment(deploymentUpdate.id, stepActive));

        await fetchDeployments(backendActor!);

        setIsDeploying(false);
      }
    } catch (e) {
      console.error(e);
      alert("Failed to complete deployment, see console for details");
      setIsDeploying(false);
    }
  }, [tlsCertificateData, setDeploymentSteps, deploymentSteps, backendActor, fetchDeployments]);

  const handleDeploy = async () => {
    setIsLoading(true);

    openWs({
      onOpen: onWsOpen,
      onMessage: onWsMessage,
      onClose: () => {
        console.log("ws close");
      },
      onError: (err) => {
        console.error(err);
        setIsLoading(false);
      },
    });
  };

  useEffect(() => {
    setWsCallbacks({
      onOpen: onWsOpen,
      onMessage: onWsMessage,
      onClose: () => {
        console.log("ws close");
      },
      onError: (err) => {
        console.error(err);
        setIsLoading(false);
      },
    });
  }, [onWsOpen, onWsMessage, setWsCallbacks]);

  return (
    <div className="flex-1 space-y-4 p-8 pt-6">
      <div className="flex items-center justify-start">
        <BackButton />
        <h2 className="ml-4 text-3xl font-bold tracking-tight">Create Deployment</h2>
      </div>
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <LoadingButton onClick={handleDeploy} isLoading={isLoading || isDeploying}>Deploy</LoadingButton>
        {deploymentSteps.length > 0 && (
          <div className="flex flex-col gap-3">
            <p className="text-sm font-medium">Deployment Steps</p>
            <div className="flex flex-col gap-2">
              {deploymentSteps.map((el) => Object.keys(el)[0]).map((el, idx) => (
                <p key={idx}>{idx + 1}. {el}</p>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
