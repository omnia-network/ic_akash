"use client";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { useIcContext } from "@/contexts/IcContext";
import { useRouter } from "next/navigation";
import { useCallback, useMemo } from "react";

export default function Login() {
  const router = useRouter();
  const { identity, logout, isLoggedIn, isLoading } = useIcContext();
  const userPrincipal = useMemo(() => identity?.getPrincipal().toText(), [identity]);

  const handleLogout = useCallback(async () => {
    await logout();

    router.replace("/");
  }, [logout, router])

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (!isLoggedIn) {
    router.replace("/");
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-between">
      <div className="flex flex-col w-full">
        <div className="border-b">
          <div className="flex h-16 items-center px-4">
            <div className="ml-auto flex items-center space-x-4 justify-end">
              <TooltipProvider>
                <Tooltip delayDuration={100}>
                  <TooltipTrigger className="flex flex-row items-center gap-2">
                    Principal: <pre>{userPrincipal?.slice(0, 5)}...{userPrincipal?.slice(-3)}</pre>
                  </TooltipTrigger>
                  <TooltipContent
                    side="bottom"
                    sideOffset={10}
                    align="end"
                  >
                    <pre>{userPrincipal}</pre>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
              <Button variant="outline" onClick={handleLogout}>Logout</Button>
            </div>
          </div>
        </div>
        <div className="flex-1 space-y-4 p-8 pt-6">
          <div className="flex items-center justify-between space-y-2">
            <h2 className="text-3xl font-bold tracking-tight">Deployments</h2>
            <div className="flex items-center space-x-2">
              <Button>Deploy</Button>
            </div>
          </div>
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
            <p>You don&apos;t have any deployments</p>
            {/* <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Total Revenue
                </CardTitle>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  className="h-4 w-4 text-muted-foreground"
                >
                  <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
                </svg>
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">$45,231.89</div>
                <p className="text-xs text-muted-foreground">
                  +20.1% from last month
                </p>
              </CardContent>
            </Card> */}
          </div>
        </div>
      </div>
    </main>
  )
}
