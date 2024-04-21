"use client"

import { zodResolver } from "@hookform/resolvers/zod";
import { SubmitHandler, useFieldArray, useForm } from "react-hook-form";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { cn } from "@/lib/utils";
import { ChevronDown, Plus, X } from "lucide-react";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { LoadingButton } from "@/components/loading-button";
import type { DeploymentParams } from "@/declarations/backend.did";
import { DeploymentTier } from "@/types/deployment";
import { DEPLOYMENT_TIERS } from "@/lib/constants";
import Tier from "@/components/Tier";

const formSchema = z.object({
  deploymentName: z.string().min(2).max(100),
  dockerImage: z.string().min(2).max(100),
  tier: z.nativeEnum(DeploymentTier),
  command: z.array(z.string().max(50)).max(30),
  envVariables: z.array(z.object({
    name: z.string().max(50),
    value: z.string().max(50),
  })).max(20),
  volumeMount: z.string().max(50).optional(),
  expose: z.object({
    httpPort: z.coerce.number().min(1).max(65535).optional(),
    domain: z.string().max(100).optional(),
  }).optional(),
});

export interface NewDeploymentFormProps {
  isLoading?: boolean;
  isSubmitDisabled?: boolean;
  onSubmit: (values: DeploymentParams) => Promise<void> | void;
};

export const NewDeploymentForm: React.FC<NewDeploymentFormProps> = ({ isLoading, isSubmitDisabled, onSubmit }) => {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      command: [],
      envVariables: [{ name: "", value: "" }],
      tier: DeploymentTier.SMALL,
    },
    reValidateMode: "onChange",
  });

  const {
    fields: envVariablesFields,
    append: appendEnvVariable,
    remove: removeEnvVariable,
    update: updateEnvVariable,
  } = useFieldArray({
    name: "envVariables",
    control: form.control,
  });
  const watchEnvVariables = form.watch("envVariables");
  const controlledEnvVariablesFields = envVariablesFields.map((field, index) => ({
    ...field,
    ...watchEnvVariables[index]
  }));

  const onFormSubmit: SubmitHandler<z.infer<typeof formSchema>> = async (values) => {
    const tierParams = DEPLOYMENT_TIERS[values.tier];

    const deploymentParams: DeploymentParams = {
      name: values.deploymentName.trim(),
      image: values.dockerImage.trim(),
      env_vars: [],
      ports: [],
      cpu: tierParams.cpuSize,
      memory: tierParams.memorySize,
      storage: tierParams.storageSize,
      volume_mount: [],
      command: values.command.filter(Boolean),
    };

    for (const { name, value } of values.envVariables) {
      if (name && value) {
        deploymentParams.env_vars.push([name, value]);
      }
    }

    if (values.expose?.httpPort) {
      deploymentParams.ports.push({
        container_port: values.expose.httpPort,
        host_port: 80,
        domain: values.expose.domain ? [values.expose.domain] : [],
      });
    }

    if (values.volumeMount) {
      deploymentParams.volume_mount = [values.volumeMount];
    }

    await onSubmit(deploymentParams);
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onFormSubmit)} className="space-y-6">
        <FormField
          control={form.control}
          name="deploymentName"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Deployment Name*</FormLabel>
              <FormControl>
                <Input placeholder="My Deployment" {...field} />
              </FormControl>
              <FormDescription>
                A custom name for your deployment
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="dockerImage"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Docker Image Name*</FormLabel>
              <FormControl>
                <Input placeholder="postgres:16" {...field} />
              </FormControl>
              <FormDescription>
                The Docker Image name (with tag). Must be publicly available on Docker Hub
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="tier"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Tier*</FormLabel>
              <FormControl>
                <Select onValueChange={field.onChange} defaultValue={field.value}>
                  <SelectTrigger className="w-fit min-w-72 h-14 gap-2">
                    <SelectValue placeholder="Select a Tier..." />
                  </SelectTrigger>
                  <SelectContent>
                    {Object.keys(DEPLOYMENT_TIERS).map((tier) => (
                      <SelectItem
                        key={tier}
                        value={tier}
                        disabled={!DEPLOYMENT_TIERS[tier as DeploymentTier].isEnabled}
                      >
                        <Tier tier={tier as DeploymentTier} />
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </FormControl>
              <FormDescription>
                Choose between one of the available tiers for your deployment
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <Collapsible>
          <CollapsibleTrigger asChild>
            <Button variant="outline" size="sm" className="mb-4">
              Advanced Options
              <ChevronDown className="ml-2 h-4 w-4" />
            </Button>
          </CollapsibleTrigger>
          <CollapsibleContent className="flex flex-col gap-4">
            <FormField
              control={form.control}
              name="command"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Custom command</FormLabel>
                  <FormControl>
                    <Input
                      placeholder="/bin/to/run --argument1 value1 --argument2 value2 ..."
                      {...field}
                      value={field.value?.join(" ")}
                      onChange={(e) => {
                        const valueArray = e.target.value?.split(" ") || [];
                        field.onChange(valueArray);
                      }}
                    />
                  </FormControl>
                  <FormDescription>
                    Override the Docker Entrypoint if needed
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div>
              {controlledEnvVariablesFields.map((field, index) => (
                <div
                  key={field.id}
                  className="flex flex-row gap-1 items-end"
                >
                  <FormField
                    control={form.control}
                    name={`envVariables.${index}.name`}
                    render={({ field: innerField }) => (
                      <FormItem className="flex-1">
                        <FormLabel className={cn(index !== 0 && "sr-only")}>
                          Environment Variables
                        </FormLabel>
                        <FormDescription className={cn(index !== 0 && "sr-only")}>
                          Add environment variables (if needed)
                        </FormDescription>
                        <FormControl>
                          <Input placeholder="ENV_VARIABLE_NAME" {...innerField} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name={`envVariables.${index}.value`}
                    render={({ field: innerField }) => (
                      <FormItem className="flex-1">
                        <FormControl>
                          <Input placeholder="Env. Variable Value" {...innerField} />
                        </FormControl>
                      </FormItem>
                    )}
                  />
                  {(controlledEnvVariablesFields.length > 1 || field.name !== "") && (
                    <Button
                      type="button"
                      variant="outline"
                      size="icon"
                      onClick={() => {
                        if (controlledEnvVariablesFields.length > 1) {
                          removeEnvVariable(index);
                        } else {
                          updateEnvVariable(index, { name: "", value: "" });
                        }
                      }}
                    >
                      <X className="h-4 w-4" />
                    </Button>
                  )}
                </div>
              ))}
              <Button
                type="button"
                variant="outline"
                size="sm"
                className="mt-2"
                onClick={() => appendEnvVariable({ name: "", value: "" })}
              >
                <Plus className="mr-2 h-4 w-4" />
                Add Environment Variable
              </Button>
            </div>
            <FormField
              control={form.control}
              name="volumeMount"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Volume Mount</FormLabel>
                  <FormControl>
                    <Input
                      placeholder="/path/to/internal/directory"
                      {...field}
                    />
                  </FormControl>
                  <FormDescription>
                    Specify where to mount the storage volume (if needed)
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div
              className="flex flex-row gap-1 items-end"
            >
              <FormField
                control={form.control}
                name="expose.httpPort"
                render={({ field: innerField }) => (
                  <FormItem className="flex-1">
                    <FormLabel>
                      Ports mapping
                    </FormLabel>
                    <FormDescription>
                      Specify the container port that you want to expose publicly via HTTPS,
                      and eventually the custom domain you want to use.
                      In future versions, we&apos;ll support other protocols.
                    </FormDescription>
                    <FormControl>
                      <Input placeholder="Container Port (e.g. 8080)" type="number" {...innerField} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="expose.domain"
                render={({ field: innerField }) => (
                  <FormItem className="flex-1">
                    <FormControl>
                      <Input placeholder="Custom domain (optional)" {...innerField} />
                    </FormControl>
                  </FormItem>
                )}
              />
            </div>
          </CollapsibleContent>
        </Collapsible>
        <LoadingButton
          type="submit"
          isLoading={isLoading || form.formState.isSubmitting}
          disabled={isSubmitDisabled || !form.formState.isValid}
        >
          Deploy service
        </LoadingButton>
      </form>
    </Form>
  );
};
