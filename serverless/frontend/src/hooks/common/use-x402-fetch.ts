"use client";

import { useState } from "react";
import { useWalletClient } from "wagmi";
import { wrapFetchWithPayment } from "x402-fetch";

export function useFetchWithX402Payment({
  host = process.env.NEXT_PUBLIC_SERVER_URL,
  endPath,
  method = "GET",
  data
}: {
  host?: string;
  endPath: string;
  method?: "GET" | "POST" | "PUT" | "DELETE";
  data?: any;
}) {
  const { data: walletClient } = useWalletClient();
  const [loading, setLoading] = useState(false);
  const fetchWithPay = wrapFetchWithPayment(fetch, walletClient as any);

  const fetchHandlerAsync = async () => {
    setLoading(true);
    try {
      const response = await fetchWithPay(`${host || ''}${endPath}`, {
        method,
        headers: {
          "Content-Type": "application/json",
        },
        body: data ? JSON.stringify(data) : undefined,
      });
      const res = await response.json();
      setLoading(false);

      if (res.success === false) {
        throw new Error(res.message || 'Request failed');
      }
      return res
    } catch (error) {
      setLoading(false);
      throw error;
    }
  }
  return {
    fetchHandlerAsync,
    loading
  }
}
