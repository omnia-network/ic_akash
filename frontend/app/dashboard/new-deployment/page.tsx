"use client";

import { BackButton } from "@/components/back-button";
import { LoadingButton } from "@/components/loading-button";
import { useToast } from "@/components/ui/use-toast";
import { useDeploymentContext } from "@/contexts/DeploymentContext";
import {
  type OnWsMessageCallback,
  type OnWsOpenCallback,
  useIcContext,
  type OnWsErrorCallback,
} from "@/contexts/IcContext";
import { type DeploymentState } from "@/declarations/backend.did";
import { extractDeploymentCreated } from "@/helpers/deployment";
import { extractOk } from "@/helpers/result";
import { sendManifestToProvider } from "@/services/deployment";
import { useRouter } from "next/navigation";
import { useCallback, useEffect, useMemo, useState } from "react";
import { Textarea } from "@/components/ui/textarea";
import { TEST_DEPLOYMENT_CONFIG } from "@/fixtures/deployment";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { AlertCircle, Milestone } from "lucide-react";
import { DEPLOYMENT_PRICE_E8S } from "@/lib/constants";
import { displayE8sAsIcp } from "@/helpers/ui";
import { transferE8sToBackend } from "@/services/backend";

export default function NewDeployment() {
  const router = useRouter();
  const { backendActor, openWs, closeWs, setWsCallbacks, ledgerCanister, ledgerData, refreshLedgerData } = useIcContext();
  const { tlsCertificateData, loadOrCreateCertificate, fetchDeployments } =
    useDeploymentContext();
  const [isLoading, setIsLoading] = useState(false);
  const [isDeploying, setIsDeploying] = useState(false);
  const [deploymentSteps, setDeploymentSteps] = useState<
    Array<DeploymentState>
  >([]);
  const userHasEnoughBalance = useMemo(() =>
    ledgerData.balanceE8s !== null && ledgerData.balanceE8s > DEPLOYMENT_PRICE_E8S,
    [ledgerData.balanceE8s]
  );
  const [paymentStatus, setPaymentStatus] = useState<string | null>(null);
  const { toast } = useToast();

  const toastError = useCallback(
    (message: string) => {
      setIsLoading(false);

      toast({
        variant: "destructive",
        title: "Something went wrong.",
        description: message,
      });
    },
    [toast]
  );

  const onWsOpen: OnWsOpenCallback = useCallback(async () => {
    console.log("ws open");

    setIsDeploying(true);
    try {
      await loadOrCreateCertificate(backendActor!);

      const res = await backendActor!.create_test_deployment();
      const deploymentId = extractOk(res);

      console.log("deployment id", deploymentId);

      setDeploymentSteps([{ Initialized: null }]);
    } catch (e) {
      console.error(e);
      toastError("Failed to create deployment, see console for details");
    }

    setIsLoading(false);
  }, [backendActor, loadOrCreateCertificate, toastError]);

  const onWsMessage: OnWsMessageCallback = useCallback(
    async (ev) => {
      console.log("ws message");

      const deploymentUpdate = ev.data;
      console.log("deployment update", deploymentUpdate);
      setDeploymentSteps((prev) => [...prev, deploymentUpdate.update]);

      if ("FailedOnCanister" in deploymentUpdate.update) {
        const err = deploymentUpdate.update.FailedOnCanister.reason;
        console.error("Failed to deploy", err);
        toastError("Failed to deploy, see console for details");
        return;
      }

      let leaseCreated = false;

      try {
        if ("LeaseCreated" in deploymentUpdate.update) {
          const { manifest_sorted_json, dseq } = extractDeploymentCreated(
            deploymentSteps.find((el) =>
              el.hasOwnProperty("DeploymentCreated")
            )!
          );
          const { provider_url } = deploymentUpdate.update.LeaseCreated;

          const manifestUrl = new URL(
            `/deployment/${dseq}/manifest`,
            provider_url
          );

          await sendManifestToProvider(
            manifestUrl.toString(),
            manifest_sorted_json,
            tlsCertificateData!
          );

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

          extractOk(
            await backendActor!.update_deployment_state(
              deploymentUpdate.id,
              stepFailed
            )
          );
        } catch (e) {
          console.error("Failed to update deployment", e);
        }

        toastError(
          "Failed to send manifest to provider, see console for details"
        );
        setIsDeploying(false);
      }

      try {
        if (leaseCreated) {
          closeWs();

          const stepActive = {
            Active: null,
          };
          setDeploymentSteps((prev) => [...prev, stepActive]);
          extractOk(
            await backendActor!.update_deployment_state(
              deploymentUpdate.id,
              stepActive
            )
          );

          await fetchDeployments(backendActor!);

          setIsDeploying(false);
          router.push("/dashboard");
        }
      } catch (e) {
        console.error(e);
        toastError("Failed to complete deployment, see console for details");
        setIsDeploying(false);
      }
    },
    [
      tlsCertificateData,
      setDeploymentSteps,
      deploymentSteps,
      backendActor,
      fetchDeployments,
      router,
      closeWs,
      toastError,
    ]
  );

  const onWsError: OnWsErrorCallback = useCallback(
    (err) => {
      console.error("WebSocket error:", err);
      toastError("The WebSocket connection returned an error.");
    },
    [toastError]
  );

  const handleDeploy = useCallback(async () => {
    if (!backendActor || !ledgerCanister) {
      toastError("Backend actor or ledger canister not found");
      return;
    }

    if (!userHasEnoughBalance) {
      toastError("Insufficient balance");
      return;
    }

    setIsLoading(true);

    try {
      setPaymentStatus("Sending ICP to backend canister...");
      await transferE8sToBackend(ledgerCanister, DEPLOYMENT_PRICE_E8S, backendActor);
      await refreshLedgerData();
      setPaymentStatus("Sending ICP to backend canister... DONE");
    } catch (e) {
      console.error(e);
      toastError("Failed to transfer funds, see console for details");
      setIsLoading(false);
      return;
    }

    openWs({
      onOpen: onWsOpen,
      onMessage: onWsMessage,
      onClose: () => {
        console.log("ws close");
      },
      onError: onWsError,
    });
  }, [
    onWsOpen,
    onWsMessage,
    onWsError,
    openWs,
    userHasEnoughBalance,
    toastError,
    backendActor,
    ledgerCanister,
    refreshLedgerData,
  ]);

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
        <h2 className="ml-4 text-3xl font-bold tracking-tight">
          Create Deployment
        </h2>
      </div>
      <div className="grid gap-4 md:grid-cols-2">
        <div className="flex flex-col gap-4">
          <div className="flex flex-col gap-2">
            <h5 className="font-bold">
              Configuration:
            </h5>
            <Textarea
              value={TEST_DEPLOYMENT_CONFIG}
              rows={TEST_DEPLOYMENT_CONFIG.split("\n").length}
              disabled
            />
            <Alert>
              <Milestone className="h-4 w-4" />
              <AlertTitle>Coming soon</AlertTitle>
              <AlertDescription>
                In the next versions, you will be able to deploy your own
                services.
              </AlertDescription>
            </Alert>
          </div>
          <div className="flex flex-col gap-2">
            <h5 className="font-bold">
              Price:
            </h5>
            <pre>{displayE8sAsIcp(DEPLOYMENT_PRICE_E8S)}</pre>
            {(!(isLoading || isDeploying) && !userHasEnoughBalance) && (
              <Alert variant="destructive">
                <AlertCircle className="h-4 w-4" />
                <AlertTitle>Insufficient balance</AlertTitle>
                <AlertDescription>
                  <p>Please top up your account.</p>
                  <p className="mt-2">Your Ledger Account ID is:</p>
                  <pre
                    className="w-fit px-2 py-1 rounded bg-secondary"
                  >
                    {ledgerData.accountId?.toHex()}
                  </pre>
                  <p className="mt-2">If you&apos;ve already topped up your account, please refresh the balance on the top bar.</p>
                </AlertDescription>
              </Alert>
            )}
          </div>
          <LoadingButton
            onClick={handleDeploy}
            isLoading={isLoading || isDeploying}
            disabled={!userHasEnoughBalance}
          >
            Deploy service
          </LoadingButton>
        </div>
        <div className="flex flex-col gap-4">
          {paymentStatus && (
            <div className="flex flex-col gap-3">
              <h5 className="font-bold">Payment Status:</h5>
              <p>{paymentStatus}</p>
            </div>
          )}
          {deploymentSteps.length > 0 && (
            <div className="flex flex-col gap-3">
              <h5 className="font-bold">Deployment Steps:</h5>
              <div className="flex flex-col gap-2">
                {deploymentSteps
                  .map((el) => Object.keys(el)[0])
                  .map((el, idx) => (
                    <p key={idx}>
                      {idx + 1}. {el}
                    </p>
                  ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
