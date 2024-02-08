"use client";

import { Button } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { useIcContext } from "@/contexts/IcContext";
import { useRouter } from "next/navigation";
import { useCallback, useEffect, useMemo } from "react";

export default function DashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const router = useRouter();
  const { identity, logout, isLoggedIn, isLoading } = useIcContext();
  const userPrincipal = useMemo(() => identity?.getPrincipal().toText(), [identity]);

  const goToHome = useCallback(() => {
    router.replace("/");
  }, [router]);

  const handleLogout = useCallback(async () => {
    await logout();

    goToHome();
  }, [logout, goToHome]);

  useEffect(() => {
    if (!isLoading && !isLoggedIn) {
      goToHome();
    }
  }, [isLoading, isLoggedIn, goToHome]);

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
        {children}
      </div>
    </main>
  )
}
