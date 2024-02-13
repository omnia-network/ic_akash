"use client";

import { LoadingButton } from "@/components/loading-button";
import { useIcContext } from "@/contexts/IcContext";
import { getOrCreateCurrentUser } from "@/services/user";
import { useRouter } from "next/navigation";
import { useCallback, useState } from "react";

export default function Home() {
  const router = useRouter();
  const { isLoggedIn, backendActor, login } = useIcContext();
  const [isLoading, setIsLoading] = useState(false);

  const handleGoToDashboard = useCallback(async () => {
    let _backendActor = backendActor;

    setIsLoading(true);

    if (!isLoggedIn || !_backendActor) {
      [, _backendActor] = await login();
    }

    await getOrCreateCurrentUser(_backendActor!);

    setIsLoading(false);

    router.push("/dashboard");
  }, [isLoggedIn, login, router, backendActor]);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <LoadingButton
        onClick={handleGoToDashboard}
        isLoading={isLoading}
      >
        Go to Dashboard
      </LoadingButton>
    </main>
  );
}
