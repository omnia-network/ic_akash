"use client";

import { BackButton } from "@/components/back-button";
import { Button } from "@/components/ui/button";
import { useCallback } from "react";

export default function Dashboard() {
  const handleDeploy = useCallback(async () => {
    // TODO
  }, []);

  return (
    <div className="flex-1 space-y-4 p-8 pt-6">
      <div className="flex items-center justify-start">
        <BackButton />
        <h2 className="ml-4 text-3xl font-bold tracking-tight">Create Deployment</h2>
      </div>
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Button onClick={handleDeploy}>Deploy</Button>
      </div>
    </div>
  )
}
