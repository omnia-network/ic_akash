import { Button } from "@/components/ui/button";
import { CheckIcon, ClipboardIcon } from "lucide-react";
import { useCallback, useState } from "react";
import { useToast } from "@/components/ui/use-toast";

type Props = {
  text: string;
};

const ClipboardCopyButton: React.FC<Props> = ({ text }) => {
  const [isCopied, setIsCopied] = useState(false);
  const { toast } = useToast();

  const handleClick = useCallback(async () => {
    await navigator.clipboard.writeText(text);
    setIsCopied(true);

    toast({
      description: (
        <div>
          Copied
          <pre
            className="w-fit px-2 py-1 rounded bg-secondary whitespace-pre-wrap [overflow-wrap:anywhere]"
          >
            {text}
          </pre> to clipboard
        </div>
      ),
    });

    setTimeout(() => setIsCopied(false), 2000);
  }, [text, toast]);

  return (
    <Button
      variant="outline"
      size="icon"
      onClick={handleClick}
      className="w-7 h-7"
    >
      {isCopied ? (
        <CheckIcon className="h-4 w-4" />
      ) : (
        <ClipboardIcon className="h-4 w-4" />
      )}
    </Button>
  );
};

export default ClipboardCopyButton;
