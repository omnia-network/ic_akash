"use client";

import Tier from "@/components/Tier";
import {LoadingButton} from "@/components/loading-button";
import {Spinner} from "@/components/spinner";
import {Button} from "@/components/ui/button";
import {Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,} from "@/components/ui/card";
import {Collapsible, CollapsibleContent, CollapsibleTrigger,} from "@/components/ui/collapsible";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {useToast} from "@/components/ui/use-toast";
import {useDeploymentContext} from "@/contexts/DeploymentContext";
import {useIcContext} from "@/contexts/IcContext";
import {type Deployment} from "@/declarations/backend.did";
import {
  extractDeploymentCreated,
  extractLeaseCreated,
  getDeploymentCreatedDate,
  getDeploymentStateDate,
  getDeploymentStateName,
  getLastDeploymentState,
  isDeploymentActive,
  isDeploymentClosed,
  isDeploymentFailed,
} from "@/helpers/deployment";
import {displayIcp} from "@/helpers/ui";
import {queryLeaseStatus, sendManifestToProviderFlow} from "@/services/deployment";
import {DeploymentTier} from "@/types/deployment";
import {ChevronsUpDown} from "lucide-react";
import {useRouter} from "next/navigation";
import {useCallback, useEffect, useState} from "react";
import {extractOk} from "@/helpers/result";

export default function Dashboard() {
  const router = useRouter();
  const {isLoggedIn, backendActor} = useIcContext();
  const {deployments, fetchDeployments, loadOrCreateCertificate} =
    useDeploymentContext();
  const [isClosingDeployment, setIsClosingDeployment] = useState(false);
  const [dialogDeploymentId, setDialogDeploymentId] = useState<string>();
  const [isStatusDialogOpen, setIsStatusDialogOpen] = useState(false);
  const [isFetchingStatus, setIsFetchingStatus] = useState(false);
  const [leaseStatusData, setLeaseStatusData] =
    useState<Record<string, unknown>>();
  const {toast} = useToast();

  const handleNewDeployment = useCallback(async () => {
    router.push("/dashboard/new-deployment");
  }, [router]);

  const handleCloseDeployment = useCallback(
    async (deploymentId: string) => {
      setIsClosingDeployment(true);

      const res = await backendActor!.close_deployment(deploymentId);

      setIsClosingDeployment(false);
      setDialogDeploymentId(undefined);

      if ("Err" in res) {
        console.error("Failed to close deployment:", res.Err);
        alert("Failed to close deployment, see console for details");
      }

      await fetchDeployments(backendActor!);
    },
    [backendActor, fetchDeployments]
  );

  const handleFetchStatus = useCallback(
    async (deployment: Deployment) => {
      setIsStatusDialogOpen(true);
      setIsFetchingStatus(true);

      try {
        const certData = await loadOrCreateCertificate(backendActor!);
        const updates = deployment.state_history.map(([_, d]) => d);
        const {dseq} = extractDeploymentCreated(
          updates.find((el) => el.hasOwnProperty("DeploymentCreated"))!
        );
        const {provider_url} = extractLeaseCreated(
          updates.find((el) => el.hasOwnProperty("LeaseCreated"))!
        );

        const queryLeaseUrl = new URL(
          `/lease/${dseq}/1/1/status`,
          provider_url
        );

        const statusData = await queryLeaseStatus(
          queryLeaseUrl.toString(),
          certData!
        );
        setLeaseStatusData(statusData);
      } catch (e) {
        console.error("Failed to query lease status", e);
        toast({
          variant: "destructive",
          title: "Something went wrong.",
          description: "Failed to query lease status, see console for details.",
        });
      }

      setIsFetchingStatus(false);
    },
    [toast, loadOrCreateCertificate, backendActor]
  );

  useEffect(() => {
    if (isLoggedIn && backendActor) {
      fetchDeployments(backendActor);
    }
  }, [isLoggedIn, fetchDeployments, backendActor]);

  useEffect(() => {
    checkDeploymentState();
  }, [deployments]);

  const checkDeploymentState = useCallback(async () => {
    for (const deployment of deployments) {
      const lastState = deployment.deployment.state_history[deployment.deployment.state_history.length - 1][1];
      const deploymentCreatedState = deployment.deployment.state_history.find(([_, state]) => "DeploymentCreated" in state)![1];
      if ("LeaseCreated" in lastState) {
        try {
          const cert = await loadOrCreateCertificate(backendActor!);

          await sendManifestToProviderFlow(
            lastState,
            deploymentCreatedState,
            cert!
          );

          const stepActive = {
            Active: null,
          };

          extractOk(
            await backendActor!.update_deployment_state(
              deployment.id,
              stepActive
            )
          );
        } catch (e) {
          console.error(e);
        }
      }
    }
  }, [backendActor, deployments, loadOrCreateCertificate]);

  return (
    <div className="flex-1 space-y-4 p-8 pt-6">
      <div className="flex items-center justify-between space-y-2">
        <h2 className="text-3xl font-bold tracking-tight">Deployments</h2>
        <div className="flex items-center space-x-2">
          <Button onClick={handleNewDeployment}>New Deployment</Button>
        </div>
      </div>
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {deployments.length === 0 && <p>You don&apos;t have any deployments</p>}
        {deployments.length > 0 &&
          deployments.map((el) => (
            <Card key={el.id}>
              <CardHeader>
                <CardTitle className="text-sm font-medium">{el.deployment.params.name}</CardTitle>
                <CardDescription>
                  <pre className="font-xs">{el.id}</pre>
                </CardDescription>
              </CardHeader>
              <CardContent className="flex flex-col gap-1">
                <div>
                  Created at:{" "}
                  {getDeploymentCreatedDate(el.deployment).toISOString()}
                </div>
                <div>
                  Status:{" "}
                  <span className="text-xl font-bold">
                    {getDeploymentStateName(
                      getLastDeploymentState(el.deployment)
                    )}
                  </span>
                </div>
                <div className="flex flex-row gap-1">
                  Tier:
                  {/* TODO: display the actual deployment tier */}
                  <div className="border rounded-md px-3 py-2">
                    <Tier tier={DeploymentTier.SMALL}/>
                  </div>
                </div>
                <div className="flex flex-row gap-1">
                  Price:
                  <pre>{displayIcp(el.deployment.icp_price, {maximumFractionDigits: 6})}</pre>
                </div>
                <Collapsible>
                  <CollapsibleTrigger className="flex items-center gap-4 w-full mt-4">
                    Status history
                    <ChevronsUpDown className="h-4 w-4"/>
                  </CollapsibleTrigger>
                  <CollapsibleContent className="flex flex-col gap-1">
                    {el.deployment.state_history.map((item, i) => (
                      <div
                        key={i}
                        className="flex flex-col gap-2 rounded-md border px-4 py-3 font-mono text-sm"
                      >
                        {getDeploymentStateName(item[1])}
                        <p className="text-xs">
                          {getDeploymentStateDate(item).toISOString()}
                        </p>
                      </div>
                    ))}
                  </CollapsibleContent>
                </Collapsible>
              </CardContent>
              {!isDeploymentClosed(el.deployment) && (
                <CardFooter className="gap-2">
                  {(isDeploymentActive(el.deployment) && !isDeploymentFailed(el.deployment)) && (
                    <>
                      <Button
                        variant="secondary"
                        onClick={() => handleFetchStatus(el.deployment)}
                      >
                        Fetch status
                      </Button>
                      <Dialog
                        open={isStatusDialogOpen}
                        onOpenChange={setIsStatusDialogOpen}
                      >
                        <DialogContent>
                          <DialogHeader>
                            <DialogTitle>Deployment status</DialogTitle>
                            <DialogDescription>
                              {isFetchingStatus ? (
                                <Spinner/>
                              ) : (
                                Boolean(leaseStatusData) && (
                                  <span className="font-mono">
                                    {JSON.stringify(leaseStatusData, null, 2)}
                                  </span>
                                )
                              )}
                            </DialogDescription>
                          </DialogHeader>
                        </DialogContent>
                      </Dialog>
                    </>
                  )}
                  <Dialog
                    open={dialogDeploymentId === el.id}
                    onOpenChange={(open) =>
                      setDialogDeploymentId(open ? el.id : undefined)
                    }
                  >
                    <DialogTrigger asChild>
                      <Button variant="outline">Close Deployment</Button>
                    </DialogTrigger>
                    <DialogContent>
                      <DialogHeader>
                        <DialogTitle>Are you sure?</DialogTitle>
                        <DialogDescription>
                          Deployment id to close:
                          <br/>
                          <span className="font-mono text-nowrap">{el.id}</span>
                          <br/>
                          <b>This action cannot be undone.</b>
                        </DialogDescription>
                      </DialogHeader>
                      <DialogFooter>
                        <LoadingButton
                          variant="destructive"
                          onClick={() => handleCloseDeployment(el.id)}
                          isLoading={isClosingDeployment}
                        >
                          Close Deployment
                        </LoadingButton>
                      </DialogFooter>
                    </DialogContent>
                  </Dialog>
                </CardFooter>
              )}
            </Card>
          ))}
      </div>
    </div>
  );
}
