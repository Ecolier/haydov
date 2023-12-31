'use client';

import { LoginFlow, UpdateLoginFlowBody } from "@ory/client"
import { AxiosError } from "axios"
import { useRouter, useSearchParams } from "next/navigation"
import { useState }from "react"
import ory from '@/ory';
import { FlowProvider } from "@/FlowProvider";
import useLogin from "@/hooks/use-login";
import Flow from "@/Flow";

type Props = {
  returnTo?: string;
  flowId?: string;
  refresh?: string;
  aal?: string;
};

export default function LoginFlow({returnTo, flowId, refresh, aal}: Props) {
  const [flow, setFlow] = useState<LoginFlow>();
  const router = useRouter();
  useLogin(flow, {
    flowId: flowId ? String(flowId) : undefined,
    returnTo: returnTo ? String(returnTo) : undefined,
    aal: aal ? String(aal) : undefined,
    refresh: Boolean(refresh),
  }, setFlow);

  const onSubmit = (values: UpdateLoginFlowBody) => {
    router.push(`/login?flow=${flow?.id}`);
    return ory.updateLoginFlow({
        flow: String(flow?.id),
        updateLoginFlowBody: values,
      })
      .then(() => {
        if (flow?.return_to) {
          window.location.href = flow?.return_to;
          return
        }
        router.push("/")
      })
      .catch((err: AxiosError) => {
        // If the previous handler did not catch the error it's most likely a form validation error
        if (err.response?.status === 400) {
          // Yup, it is!
          setFlow(err.response?.data)
          return
        }
        return Promise.reject(err)
      })
  }

  return (
    <FlowProvider type="login" reset={() => setFlow(undefined)}>
      {flow && <Flow flow={flow}></Flow>}
    </FlowProvider>
  )
}