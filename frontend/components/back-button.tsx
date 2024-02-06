import { useRouter } from "next/navigation";
import { Button } from "./ui/button"
import { useCallback } from "react";
import { ArrowLeft } from "lucide-react"

export const BackButton = () => {
  const router = useRouter();

  const handleBack = useCallback(() => {
    router.back();
  }, [router]);

  return (
    <Button
      variant="outline"
      onClick={handleBack}
    >
      <ArrowLeft className="mr-2 h-4 w-4" />
      Back
    </Button>
  )
}