"use client";

import { useAccount, useReadContract } from "wagmi";

import { formatUnits } from "viem";
import { useMemo } from "react";

const BASE_SEPOLIA_USDC = "0x036CbD53842c5426634e7929541eC2318f3dCF7e" as const;

const ERC20_ABI = [
  {
    constant: true,
    inputs: [{ name: "_owner", type: "address" }],
    name: "balanceOf",
    outputs: [{ name: "balance", type: "uint256" }],
    type: "function",
  },
  {
    constant: true,
    inputs: [],
    name: "decimals",
    outputs: [{ name: "", type: "uint8" }],
    type: "function",
  },
  {
    constant: true,
    inputs: [],
    name: "symbol",
    outputs: [{ name: "", type: "string" }],
    type: "function",
  },
] as const;

export function useBaseSepoliaUsdc() {
  const { address, chain } = useAccount();

  const isBaseSepolia = chain?.id === 84532;

  const { data: balance, isLoading, refetch, error } = useReadContract({
    address: BASE_SEPOLIA_USDC,
    abi: ERC20_ABI,
    functionName: "balanceOf",
    args: address ? [address] : undefined,
    query: {
      enabled: Boolean(address && isBaseSepolia),
    },
  });

  const { data: decimals } = useReadContract({
    address: BASE_SEPOLIA_USDC,
    abi: ERC20_ABI,
    functionName: "decimals",
    query: {
      enabled: isBaseSepolia,
    },
  });

  const { data: symbol } = useReadContract({
    address: BASE_SEPOLIA_USDC,
    abi: ERC20_ABI,
    functionName: "symbol",
    query: {
      enabled: isBaseSepolia,
    },
  });

  const formatted = useMemo(() => {
    if (!balance || !decimals) return "0.00";
    const value = formatUnits(balance as bigint, decimals as number);
    return Number(value).toFixed(2);
  }, [balance, decimals]);

  const value = useMemo(() => {
    if (!balance || !decimals) return 0;
    return Number(formatUnits(balance as bigint, decimals as number));
  }, [balance, decimals]);

  return {
    balance: balance as bigint | undefined,
    decimals: decimals as number | undefined,
    symbol: (symbol as string) || "USDC",
    formatted,
    value,
    isLoading,
    isBaseSepolia,
    contractAddress: BASE_SEPOLIA_USDC,
    refetch,
    error,
  };
}