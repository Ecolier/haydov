'use client';

import { PropsWithChildren, createContext, useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import axios from '@/axios';
import { AxiosError } from 'axios';

type FlowError = AxiosError<{
  error: {
    id: string;
    code: number;
    status: string;
    reason: string;
    details: {
        docs: string;
        hint: string;
        reject_reason: string;
    },
    message: string;
  }
  redirect_browser_to: string;
}>;

interface FlowProviderProps extends PropsWithChildren {
  type: string;
  reset: () => void;
}

export function FlowProvider({children, type, reset}: FlowProviderProps) {
  const router = useRouter();
  axios.interceptors.response.use(response => response, (error: FlowError) => {
    switch (error.response?.data.error?.id) {
      case 'session_inactive':
        router.push('/login?return_to=' + window.location.href);
        return;
      case 'session_aal2_required':
        if (error.response?.data.redirect_browser_to) {
          const redirectTo = new URL(error.response?.data.redirect_browser_to);
          if (type === 'settings') {
            redirectTo.searchParams.set('return_to', window.location.href)
          }
          // 2FA is enabled and enforced, but user did not perform 2fa yet!
          window.location.href = redirectTo.toString()
          return;
        }
        router.push('/login?aal=aal2&return_to=' + window.location.href)
        return
      case 'session_already_available':
        // User is already signed in, let's redirect them home!
        router.push('/')
        return;
      case 'session_refresh_required':
        // We need to re-authenticate to perform this action
        window.location.href = error.response?.data.redirect_browser_to
        return
      case 'self_service_flow_return_to_forbidden':
        // The flow expired, let's request a new one.
        console.log('The return_to address is not allowed.')
        reset();
        router.push('/' + type);
        return
      case 'self_service_flow_expired':
        // The flow expired, let's request a new one.
        console.log('Your interaction expired, please fill out the form again.')
        reset();
        router.push('/' + type);
        return
      case 'security_csrf_violation':
        // A CSRF violation occurred. Best to just refresh the flow!
        console.log(
          'A security violation was detected, please fill out the form again.',
        )
        reset();
        router.push('/' + type)
        return
      case 'security_identity_mismatch':
        // The requested item was intended for someone else. Let's request a new flow...
        reset();
        router.push('/' + type)
        return
      case 'browser_location_change_required':
        // Ory Kratos asked us to point the user to this URL.
        window.location.href = error.response.data.redirect_browser_to
        return
    }

    switch (error.response?.status) {
      case 410:
        // The flow expired, let's request a new one.
        reset();
        router.push('/' + type)
        return
    }

    // We are not able to handle the error? Return it.
    return Promise.reject(error)
  });
  return (
    <>{children}</>
  )
}