import { LoginFlow } from "@ory/client";
import { useEffect } from "react";
import ory from '@/ory';

interface UseLoginOptions {
  aal?: string;
  refresh?: boolean;
  returnTo?: string;
  flowId?: string;
}

function useLogin(flow: LoginFlow | undefined, {
  flowId, refresh, returnTo, aal,
}: UseLoginOptions, onFlow?: (flow: LoginFlow) => void) {
  useEffect(() => {
    if (flow) {
      return;
    }
    if (flowId) {
      ory.getLoginFlow({id: flowId}).then(response => onFlow && onFlow(response?.data));
      return;
    }
    ory.createBrowserLoginFlow({
      refresh, aal, returnTo,
    }).then(({data}) => onFlow && onFlow(data));
  }, [flow, flowId, refresh, returnTo, aal, onFlow])
}

export default useLogin;