import { DEPLOYMENT_TIERS } from "@/lib/constants";
import { DeploymentTier } from "@/types/deployment";
import { Badge } from "@/components/ui/badge";

type Props = {
  tier: DeploymentTier
};

const Tier: React.FC<Props> = ({ tier }) => {
  return (
    <div className="flex flex-col items-start gap-1">
      <span>
        {DEPLOYMENT_TIERS[tier].titleText} {!DEPLOYMENT_TIERS[tier].isEnabled && <Badge variant="secondary">Coming Soon</Badge>}
      </span>
      <p className="text-xs text-muted-foreground">{DEPLOYMENT_TIERS[tier].subtitleText}</p>
    </div>
  );
};

export default Tier;
