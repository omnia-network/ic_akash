"use client";

import { ThemeProvider } from "@/components/theme-provider";
import { DeploymentProvider } from "@/contexts/DeploymentContext";
import { IcProvider } from "@/contexts/IcContext";

export function Providers({ children }: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <ThemeProvider
      attribute="class"
      defaultTheme="dark"
      enableSystem={false}
      disableTransitionOnChange
    >
      <IcProvider>
        <DeploymentProvider>
          {children}
        </DeploymentProvider>
      </IcProvider>
    </ThemeProvider>
  );
}
