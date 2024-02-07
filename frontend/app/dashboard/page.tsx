"use client";

import { LoadingButton } from "@/components/loading-button";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Collapsible } from "@/components/ui/collapsible";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { useDeploymentContext } from "@/contexts/DeploymentContext";
import { useIcContext } from "@/contexts/IcContext";
import { getDeploymentCreatedDate, getDeploymentUpdateDate, getDeploymentUpdateName, getLastDeploymentUpdate, isDeploymentClosed, isDeploymentFailed } from "@/helpers/deployment";
import { CollapsibleContent, CollapsibleTrigger } from "@radix-ui/react-collapsible";
import { ChevronsUpDown } from "lucide-react";
import { useRouter } from "next/navigation";
import { useCallback, useEffect, useState } from "react";

export default function Dashboard() {
  const router = useRouter();
  const { isLoggedIn, backendActor } = useIcContext();
  const { deployments, fetchDeployments } = useDeploymentContext();
  const [isClosingDeployment, setIsClosingDeployment] = useState(false);
  const [dialogDeploymentId, setDialogDeploymentId] = useState<string>();

  const handleNewDeployment = useCallback(async () => {
    router.push("/dashboard/new-deployment");
  }, [router]);

  const handleCloseDeployment = useCallback(async (deploymentId: string) => {
    setIsClosingDeployment(true);

    const res = await backendActor!.close_deployment(deploymentId);

    setIsClosingDeployment(false);
    setDialogDeploymentId(undefined);

    if ("Err" in res) {
      console.error("Failed to close deployment:", res.Err);
      alert("Failed to close deployment, see console for details");
    }

    await fetchDeployments(backendActor!);
  }, [backendActor, fetchDeployments]);

  useEffect(() => {
    if (isLoggedIn && backendActor) {
      fetchDeployments(backendActor);
    }
  }, [isLoggedIn, fetchDeployments, backendActor]);

  return (
    <div className="flex-1 space-y-4 p-8 pt-6">
      <div className="flex items-center justify-between space-y-2">
        <h2 className="text-3xl font-bold tracking-tight">Deployments</h2>
        <div className="flex items-center space-x-2">
          <Button onClick={handleNewDeployment}>New Deployment</Button>
        </div>
      </div>
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        {deployments.length === 0 && (
          <p>You don&apos;t have any deployments</p>
        )}
        {deployments.length > 0 && deployments.map((el) => (
          <Card key={el.id}>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">
                {el.id}
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div>
                Created at: {getDeploymentCreatedDate(el.deployment).toISOString()}
              </div>
              <div>
                Status: <span className="text-xl font-bold">{getDeploymentUpdateName(getLastDeploymentUpdate(el.deployment))}</span>
              </div>
              <Collapsible>
                <CollapsibleTrigger className="flex items-center gap-4 w-full mt-4">
                  Status history
                  <ChevronsUpDown className="h-4 w-4" />
                </CollapsibleTrigger>
                <CollapsibleContent className="flex flex-col gap-1">
                  {el.deployment.state_history.map((item, i) => (
                    <div
                      key={i}
                      className="flex flex-col gap-2 rounded-md border px-4 py-3 font-mono text-sm"
                    >
                      {getDeploymentUpdateName(item[1])}
                      <p className="text-xs">{getDeploymentUpdateDate(item).toISOString()}</p>
                    </div>
                  ))}
                </CollapsibleContent>
              </Collapsible>
            </CardContent>
            {!isDeploymentClosed(el.deployment) && (
              <CardFooter>
                <Dialog
                  open={dialogDeploymentId === el.id}
                  onOpenChange={(open) => setDialogDeploymentId(open ? el.id : undefined)}
                >
                  <DialogTrigger asChild>
                    <Button variant="outline">Close Deployment</Button>
                  </DialogTrigger>
                  <DialogContent>
                    <DialogHeader>
                      <DialogTitle>Are you sure?</DialogTitle>
                      <DialogDescription>
                        This action cannot be undone.

                        <p>Deployment id to close: <pre>{el.id}</pre></p>
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
  )
}
