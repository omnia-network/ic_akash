"use client";

import { BackButton } from "@/components/back-button";
import { LoadingButton } from "@/components/loading-button";
import { useDeploymentContext } from "@/contexts/DeploymentContext";
import { type OnWsMessageCallback, type OnWsOpenCallback, useIcContext } from "@/contexts/IcContext";
import { useCallback, useState } from "react";

export default function NewDeployment() {
  const { identity, backendActor, openWs } = useIcContext();
  const { loadOrCreateCertificate } = useDeploymentContext();
  const [isLoading, setIsLoading] = useState(false);

  const onWsOpen: OnWsOpenCallback = useCallback(async () => {
    console.log("ws open");

    setIsLoading(true);
    try {
      await loadOrCreateCertificate(backendActor!, identity!);

      const res = await backendActor!.create_test_deployment();
      console.log("create_test_deployment res:", res);

    } catch (e) {
      console.error(e);
      alert("Failed to create deployment, see console for details");
    }

    setIsLoading(false);
  }, [backendActor, identity, loadOrCreateCertificate]);

  const onWsMessage: OnWsMessageCallback = useCallback((ev) => {
    console.log("ws message", ev.data);
  }, []);

  const handleDeploy = useCallback(async () => {
    openWs(onWsOpen, onWsMessage);
  }, [openWs, onWsOpen, onWsMessage]);

  return (
    <div className="flex-1 space-y-4 p-8 pt-6">
      <div className="flex items-center justify-start">
        <BackButton />
        <h2 className="ml-4 text-3xl font-bold tracking-tight">Create Deployment</h2>
      </div>
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <LoadingButton onClick={handleDeploy} isLoading={isLoading}>Deploy</LoadingButton>
      </div>
    </div>
  )
}
