"use client";

import { BackButton } from "@/components/back-button";
import { LoadingButton } from "@/components/loading-button";
import { useToast } from "@/components/ui/use-toast";
import { useDeploymentContext } from "@/contexts/DeploymentContext";
import { type OnWsMessageCallback, type OnWsOpenCallback, useIcContext, type OnWsErrorCallback } from "@/contexts/IcContext";
import { type DeploymentUpdate } from "@/declarations/backend.did";
import { extractDeploymentCreated } from "@/helpers/deployment";
import { extractOk } from "@/helpers/result";
import { sendManifestToProvider } from "@/services/deployment";
import { useRouter } from "next/navigation";
import { useCallback, useEffect, useState } from "react";
import { Textarea } from "@/components/ui/textarea"
import { TEST_DEPLOYMENT_CONFIG } from "@/fixtures/deployment";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Info } from "lucide-react";

export default function NewDeployment() {
  const router = useRouter();
  const { backendActor, openWs, closeWs, setWsCallbacks } = useIcContext();
  const { tlsCertificateData, loadOrCreateCertificate, fetchDeployments } = useDeploymentContext();
  const [isLoading, setIsLoading] = useState(false);
  const [isDeploying, setIsDeploying] = useState(false);
  const [deploymentSteps, setDeploymentSteps] = useState<Array<DeploymentUpdate>>([]);
  const { toast } = useToast();

  const toastError = useCallback((message: string) => {
    setIsLoading(false);

    toast({
      variant: "destructive",
      title: "Something went wrong.",
      description: message,
    });
  }, [toast]);

  const onWsOpen: OnWsOpenCallback = useCallback(async () => {
    console.log("ws open");

    setIsDeploying(true);
    try {
      await loadOrCreateCertificate(backendActor!);

      const res = await backendActor!.create_test_deployment();
      const deploymentId = extractOk(res);

      console.log("deployment id", deploymentId);

      setDeploymentSteps([
        { Initialized: null }
      ]);

    } catch (e) {
      console.error(e);
      toastError("Failed to create deployment, see console for details");
    }

    setIsLoading(false);
  }, [backendActor, loadOrCreateCertificate, toastError]);

  const onWsMessage: OnWsMessageCallback = useCallback(async (ev) => {
    console.log("ws message");

    const deploymentUpdate = ev.data;
    console.log("deployment update", deploymentUpdate);
    setDeploymentSteps((prev) => [...prev, deploymentUpdate.update]);

    if ("FailedOnCanister" in deploymentUpdate.update) {
      const err = deploymentUpdate.update.FailedOnCanister.reason;
      console.error("Failed to deploy", err);
      toastError("Failed to deploy, see console for details");
      setIsDeploying(false);
      return;
    }

    let leaseCreated = false;

    try {
      if ("LeaseCreated" in deploymentUpdate.update) {
        const { manifest_sorted_json, dseq } = extractDeploymentCreated(deploymentSteps.find((el) => el.hasOwnProperty("DeploymentCreated"))!);
        const { provider_url } = deploymentUpdate.update.LeaseCreated;

        const manifestUrl = new URL(`/deployment/${dseq}/manifest`, provider_url);

        await sendManifestToProvider(manifestUrl.toString(), manifest_sorted_json, tlsCertificateData!);

        leaseCreated = true;
      }
    } catch (e) {
      console.error(e);

      try {
        const stepFailed = {
          FailedOnClient: {
            reason: JSON.stringify(e),
          },
        };

        setDeploymentSteps((prev) => [...prev, stepFailed]);

        extractOk(await backendActor!.update_deployment(deploymentUpdate.id, stepFailed));
      } catch (e) {
        console.error("Failed to update deployment", e);
      }

      toastError("Failed to send manifest to provider, see console for details");
      setIsDeploying(false);
    }

    try {
      if (leaseCreated) {
        closeWs();

        const stepActive = {
          Active: null,
        };
        setDeploymentSteps((prev) => [...prev, stepActive]);
        extractOk(await backendActor!.update_deployment(deploymentUpdate.id, stepActive));

        await fetchDeployments(backendActor!);

        setIsDeploying(false);
        router.push("/dashboard");
      }
    } catch (e) {
      console.error(e);
      toastError("Failed to complete deployment, see console for details");
      setIsDeploying(false);
    }
  }, [tlsCertificateData, setDeploymentSteps, deploymentSteps, backendActor, fetchDeployments, router, closeWs, toastError]);

  const onWsError: OnWsErrorCallback = useCallback((err) => {
    console.error("WebSocket error:", err);
    toastError("The WebSocket connection returned an error.");
  }, [toastError]);

  const handleDeploy = useCallback(async () => {
    setIsLoading(true);

    openWs({
      onOpen: onWsOpen,
      onMessage: onWsMessage,
      onClose: () => {
        console.log("ws close");
      },
      onError: onWsError,
    });
  }, [onWsOpen, onWsMessage, onWsError, openWs]);

  useEffect(() => {
    setWsCallbacks({
      onOpen: onWsOpen,
      onMessage: onWsMessage,
      onClose: () => {
        console.log("ws close");
      },
      onError: onWsError,
    });
  }, [onWsOpen, onWsMessage, onWsError, setWsCallbacks]);

  return (
    <div className="flex-1 space-y-4 p-8 pt-6">
      <div className="flex items-center justify-start">
        <BackButton />
        <h2 className="ml-4 text-3xl font-bold tracking-tight">Create Deployment</h2>
      </div>
      <div className="grid gap-4 md:grid-cols-2">
        <div className="flex flex-col gap-4">
          <Textarea
            value={TEST_DEPLOYMENT_CONFIG}
            rows={TEST_DEPLOYMENT_CONFIG.split("\n").length}
            disabled
          />
          <LoadingButton onClick={handleDeploy} isLoading={isLoading || isDeploying}>Deploy Peerjs Server</LoadingButton>
          <Alert>
            <Info className="h-4 w-4" />
            <AlertTitle>Coming soon</AlertTitle>
            <AlertDescription>
              In the next versions, you will be able to deploy your own services.
            </AlertDescription>
          </Alert>
        </div>
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
