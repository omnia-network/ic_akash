import * as React from "react"
import { Button, ButtonProps } from "./ui/button"
import { Loader } from "lucide-react"

export interface LoadingButtonProps extends ButtonProps {
  isLoading?: boolean
};

const LoadingButton = React.forwardRef<HTMLButtonElement, LoadingButtonProps>(
  ({ isLoading, disabled, children, ...props }, ref) => {
    return (
      <Button
        disabled={disabled || isLoading}
        {...props}
      >
        {isLoading ? <Loader className="w-4 h-4 animate-spin" /> : children}
      </Button>
    )
  }
)
LoadingButton.displayName = "LoadingButton"

export { LoadingButton };
