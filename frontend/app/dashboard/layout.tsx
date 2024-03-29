"use client";

import { Spinner } from "@/components/spinner";
import { Button } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { useIcContext } from "@/contexts/IcContext";
import { displayE8sAsIcp, shortAccountId, shortPrincipal } from "@/helpers/ui";
import { AccountIdentifier } from "@dfinity/ledger-icp";
import { RefreshCw } from "lucide-react";
import { useRouter } from "next/navigation";
import { useCallback, useEffect, useMemo, useState } from "react";

export default function DashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const router = useRouter();
  const { identity, logout, isLoggedIn, isLoading, ledgerData, refreshLedgerData } = useIcContext();
  const userPrincipal = useMemo(() => identity?.getPrincipal(), [identity]);
  const userAccountId = useMemo(() => userPrincipal && AccountIdentifier.fromPrincipal({ principal: userPrincipal }), [userPrincipal]);

  const goToHome = useCallback(() => {
    router.replace("/");
  }, [router]);

  const handleLogout = useCallback(async () => {
    await logout();
    goToHome();
  }, [logout, goToHome]);

  const fetchBalance = useCallback(async () => {
    await refreshLedgerData();
  }, [refreshLedgerData]);

  useEffect(() => {
    if (!isLoading && !isLoggedIn) {
      goToHome();
    } else {
      fetchBalance();
    }
  }, [isLoading, isLoggedIn, goToHome, fetchBalance]);

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (!isLoggedIn) {
    return null;
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
                    Principal: <pre>{shortPrincipal(userPrincipal!)}</pre>
                  </TooltipTrigger>
                  <TooltipContent
                    side="bottom"
                    sideOffset={10}
                    align="end"
                  >
                    <pre>{userPrincipal?.toText()}</pre>
                  </TooltipContent>
                </Tooltip>
                <Tooltip delayDuration={100}>
                  <TooltipTrigger className="flex flex-row items-center gap-2">
                    Ledger Account ID: <pre>{shortAccountId(userAccountId!)}</pre>
                  </TooltipTrigger>
                  <TooltipContent
                    side="bottom"
                    sideOffset={10}
                    align="end"
                  >
                    <pre>{userAccountId?.toHex()}</pre>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
              <div className="flex flex-row items-center gap-2">
                Ledger balance:
                {(!ledgerData.isLoading && ledgerData.balance !== null) ? <pre>{displayE8sAsIcp(ledgerData.balance)}</pre> : null}
                <Button
                  variant="ghost"
                  size="icon"
                  onClick={fetchBalance}
                  disabled={ledgerData.isLoading}
                >
                  {ledgerData.isLoading ? <Spinner /> : <RefreshCw className="h-4 w-4" />}
                </Button>
              </div>
              <Button variant="outline" onClick={handleLogout}>Logout</Button>
            </div>
          </div>
        </div>
        {children}
      </div>
    </main>
  )
}
