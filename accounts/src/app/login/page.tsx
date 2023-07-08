'use client';

import { LoginFlow, UpdateLoginFlowBody } from "@ory/client"
import { AxiosError } from "axios"
import type { NextPage } from "next"
import Head from "next/head"
import Link from "next/link"
import { useRouter } from "next/navigation"
import { useEffect, useState, useTransition } from "react"
import ory from '@/ory';
import { handleGetFlowError, handleFlowError } from "@/errors"
import { LogoutLink } from "@/logout";
import { FlowProvider } from "@/flow";
import useLogin from "@/hooks/use-login";
import { Flow } from "./flow";

type Props = {
  searchParams: {
    return_to?: string;
    flow?: string;
    refresh?: string;
    aal?: string;
  };
};

export default function Login({ searchParams }: Props) {
  const [flow, setFlow] = useState<LoginFlow>();
  const router = useRouter();
  const {
    return_to: returnTo,
    flow: flowId,
    refresh,
    aal,
  } = searchParams;

  useLogin(flow, {
    flowId: flowId ? String(flowId) : undefined,
    returnTo: returnTo ? String(returnTo) : undefined,
    aal: aal ? String(aal) : undefined,
    refresh: Boolean(refresh),
  }, setFlow);

  const onSubmit = (values: UpdateLoginFlowBody) => {
    history.pushState(null, '', `/login?flow=${flow?.id}`);
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
      <Flow onSubmit={onSubmit} flow={flow} />
    </FlowProvider>
  )
}