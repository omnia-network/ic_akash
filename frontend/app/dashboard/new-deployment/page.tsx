"use client";

import { BackButton } from "@/components/back-button";
import { useToast } from "@/components/ui/use-toast";
import { useDeploymentContext } from "@/contexts/DeploymentContext";
import {
  type OnWsMessageCallback,
  type OnWsOpenCallback,
  useIcContext,
  type OnWsErrorCallback,
} from "@/contexts/IcContext";
import type { DeploymentParams, DeploymentState } from "@/declarations/backend.did";
import { extractDeploymentCreated } from "@/helpers/deployment";
import { extractOk } from "@/helpers/result";
import { sendManifestToProvider } from "@/services/deployment";
import { useRouter } from "next/navigation";
import { useCallback, useEffect, useMemo, useState } from "react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { AlertCircle } from "lucide-react";
import { displayE8sAsIcp, icpToE8s } from "@/helpers/ui";
import { transferE8sToBackend } from "@/services/backend";
import { Spinner } from "@/components/spinner";
import { NewDeploymentForm } from "@/components/new-deployment-form";

const FETCH_DEPLOYMENT_PRICE_INTERVAL_MS = 30_000; // 30 seconds

export default function NewDeployment() {
  const router = useRouter();
  const { backendActor, openWs, closeWs, setWsCallbacks, ledgerCanister, ledgerData, refreshLedgerData } = useIcContext();
  const { tlsCertificateData, loadOrCreateCertificate, fetchDeployments } =
    useDeploymentContext();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isDeploying, setIsDeploying] = useState(false);
  const [deploymentSteps, setDeploymentSteps] = useState<
    Array<DeploymentState>
  >([]);
  const [deploymentE8sPrice, setDeploymentE8sPrice] = useState<bigint | null>(null);
  const [fetchDeploymentPriceInterval, setFetchDeploymentPriceInterval] = useState<NodeJS.Timeout | null>(null);
  const userHasEnoughBalance = useMemo(() =>
    ledgerData.balanceE8s !== null && deploymentE8sPrice !== null && ledgerData.balanceE8s > deploymentE8sPrice,
    [ledgerData.balanceE8s, deploymentE8sPrice]
  );
  const [paymentStatus, setPaymentStatus] = useState<string | null>(null);
  const [deploymentParams, setDeploymentParams] = useState<DeploymentParams | null>(null);
  const { toast } = useToast();

  const toastError = useCallback(
    (message: string) => {
      setIsSubmitting(false);

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
      if (!backendActor) {
        throw new Error("No backend actor");
      }

      if (!deploymentParams) {
        throw new Error("No deployment params");
      }

      await loadOrCreateCertificate(backendActor!);

      const res = await backendActor.create_deployment(deploymentParams);
      const deploymentId = extractOk(res);

      console.log("deployment id", deploymentId);

      setDeploymentSteps([{ Initialized: null }]);
    } catch (e) {
      console.error(e);
      toastError("Failed to create deployment, see console for details");
    }

    setIsSubmitting(false);
  }, [backendActor, loadOrCreateCertificate, toastError, deploymentParams]);

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

  const handleDeploy = useCallback(async (values: DeploymentParams) => {
    if (!backendActor || !ledgerCanister) {
      toastError("Backend actor or ledger canister not found");
      return;
    }

    if (!userHasEnoughBalance) {
      toastError("Insufficient balance");
      return;
    }

    if (!deploymentE8sPrice) {
      toastError("Deployment price not fetched");
      return;
    }

    setDeploymentParams(values);
    setIsSubmitting(true);

    try {
      setPaymentStatus(`Sending ~${displayE8sAsIcp(deploymentE8sPrice)} to backend canister...`);

      await transferE8sToBackend(
        ledgerCanister,
        deploymentE8sPrice,
        backendActor
      );
      await refreshLedgerData();

      if (fetchDeploymentPriceInterval !== null) {
        clearInterval(fetchDeploymentPriceInterval);
        setFetchDeploymentPriceInterval(null);
      }

      setPaymentStatus(prev => prev + " DONE");
    } catch (e) {
      console.error(e);
      toastError("Failed to transfer funds, see console for details");
      setPaymentStatus(prev => prev + " FAILED");
      setDeploymentParams(null);
      setIsSubmitting(false);
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
    deploymentE8sPrice,
    fetchDeploymentPriceInterval,
  ]);

  const fetchDeploymentPrice = useCallback(async () => {
    if (!backendActor) {
      return;
    }

    try {
      const res = await backendActor.get_deployment_icp_price();
      const icpPrice = extractOk(res);

      // add 1 ICP to cover price fluctuation
      //
      // TODO: backend canister should first create the deployment
      // and store the price, so that the frontend can fetch it
      // and transfer the correct ICP amount accordingly
      setDeploymentE8sPrice(icpToE8s(icpPrice + 1));
    } catch (e) {
      console.error("Failed to fetch deployment price", e);
      toastError("Failed to fetch deployment price, see console for details");
    }
  }, [backendActor, toastError]);

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

  useEffect(() => {
    fetchDeploymentPrice();

    const interval = setInterval(() => {
      fetchDeploymentPrice();
    }, FETCH_DEPLOYMENT_PRICE_INTERVAL_MS);

    setFetchDeploymentPriceInterval(interval);

    return () => clearInterval(interval);
  }, [fetchDeploymentPrice]);

  return (
    <div className="flex-1 space-y-4 p-8 pt-6">
      <div className="flex items-center justify-start">
        <BackButton />
        <h2 className="ml-4 text-3xl font-bold tracking-tight">
          Create Deployment
        </h2>
      </div>
      <div className="grid gap-16 md:grid-cols-2">
        <div>
          <NewDeploymentForm
            isLoading={isSubmitting || isDeploying}
            isSubmitDisabled={!userHasEnoughBalance}
            onSubmit={handleDeploy}
          />
        </div>
        <div className="flex flex-col gap-4 h-fit sticky top-20">
          <div className="flex flex-col gap-2">
            <h5 className="font-bold">
              Price (est.):
            </h5>
            {deploymentE8sPrice !== null ? (
              <pre>~{displayE8sAsIcp(deploymentE8sPrice, { maximumFractionDigits: 6 })}</pre>
            ) : (
              <Spinner />
            )}
            {(!(isSubmitting || isDeploying) && (deploymentE8sPrice !== null) && !userHasEnoughBalance) && (
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
          {paymentStatus && (
            <div className="flex flex-col gap-3">
              <h5 className="font-bold">Payment Status:</h5>
              <p>{paymentStatus}</p>
            </div>
          )}
          {(isDeploying || deploymentSteps.length > 0) && (
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
                {isDeploying && <Spinner />}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
