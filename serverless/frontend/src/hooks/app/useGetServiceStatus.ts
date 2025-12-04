"use client";

import { IDeployDetailsResDto } from "@/types/app";
import { formatServiceDetails } from "@/lib/app";
import { getServiceStatus } from "@/services/app/getServiceDetail";
import { useRequest } from "ahooks";

interface UseServiceStatusOptions {
  serviceId?: string;
  pollingInterval?: number;
  enabled?: boolean;
  retryCount?: number;
  retryInterval?: number;
  onSuccess?: (data: IDeployDetailsResDto) => void;
  onError?: (error: Error) => void;
  onPolling?: (data: IDeployDetailsResDto) => void;
}

export function useServiceStatus(options: UseServiceStatusOptions = {}) {
  const {
    serviceId,
    pollingInterval = 10 * 1000,
    enabled = false,
    retryCount = 3,
    retryInterval = 2000,
    onSuccess,
    onError,
    onPolling,
  } = options;

  const {
    data,
    error,
    loading,
    run,
    cancel,
    refresh,
    mutate,
  } = useRequest(
    async () => {
      const response = await getServiceStatus(serviceId!);
      const formattedDetails = formatServiceDetails(response.extra?.details || []);
      response.extra = {
        ...response.extra,
        details: formattedDetails,
      };
      return response;
    },
    {
      manual: false,
      ready: !!serviceId && enabled,
      pollingInterval: enabled && serviceId ? pollingInterval : undefined,
      pollingWhenHidden: false,
      pollingErrorRetryCount: retryCount,
      retryCount,
      retryInterval,
      onSuccess: (result) => {
        onSuccess?.(result);
        onPolling?.(result);
        if (result.ready) {
          cancel();
        } else if (result.extra?.details?.some(item => item.status === 'False')) {
          cancel();
          onError?.(new Error("Service deployment failed"));
        }
      },

      onError: (err) => {
        console.error("Service status polling error:", err);
        // onError?.(err as Error);
      },

      refreshDeps: [serviceId, enabled],
    }
  );

  return {
    data,
    loading,
    error,
    run,
    cancel,
    refresh,
    mutate,
  };
}