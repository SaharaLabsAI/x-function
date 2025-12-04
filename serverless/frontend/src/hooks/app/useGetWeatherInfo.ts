"use client";

import { useRequest } from "ahooks";

export function useGetWeatherInfo({
  host
}: {
  host: string;
}) {
  const { data, error, loading, run, cancel, refresh, mutate } = useRequest(
    async () => {
      const normalizedHost = host.replace(/^http:/, 'https:');
      const response = await fetch(`${normalizedHost}/weather?latitude=52.52&longitude=13.41`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
        mode: 'cors',
      });
      const res = await response.json();
      return res;
    },
    {
      manual: false,
      ready: !!host,
    }
  );

  return { data, error, loading, run, cancel, refresh, mutate };
}