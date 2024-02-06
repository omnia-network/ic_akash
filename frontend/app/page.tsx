"use client";

import { Button } from "@/components/ui/button";
import { useIcContext } from "@/contexts/IcContext";
import { useRouter } from "next/navigation";
import { useCallback } from "react";

export default function Home() {
  const router = useRouter();
  const { isLoggedIn, login } = useIcContext();

  const handleGoToDashboard = useCallback(async () => {
    if (!isLoggedIn) {
      await login();
    }

    router.push("/dashboard");
  }, [isLoggedIn, login, router]);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <Button onClick={handleGoToDashboard}>Go To Dashboard</Button>
    </main>
  );
}
