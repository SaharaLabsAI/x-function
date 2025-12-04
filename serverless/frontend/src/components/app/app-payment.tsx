"use client";

import { DetailItem, DetailList } from "./app-payment-details";
import { useMemo, useRef } from "react";

import { ConnectButton } from "@rainbow-me/rainbowkit";
import MotionButton from "@/components/common/motion-button";
import { MotionCard } from "../common/motion-card";
import { cn } from "@/lib/utils";
import { toast } from "sonner";
import { useAccount } from "wagmi";
import { useBaseSepoliaUsdc } from "@/hooks/common/use-base-sepolia-usdc";
import { useFetchWithX402Payment } from "@/hooks/common/use-x402-fetch";

export default function AppConnectModule({
  onPaymentSuccess,
}: {
  onPaymentSuccess?: (res: any) => void;
}) {
  const repoUrl = useRef(
    "https://github.com/SaharaLabsAI/weather-function-sample.git"
  );
  const { address, isConnected, chain } = useAccount();
  const { fetchHandlerAsync, loading: paymentLoading } =
    useFetchWithX402Payment({
      endPath: "/apis/x402/v1/services",
      method: "POST",
      data: {
        url: repoUrl.current,
      },
    });
  const {
    formatted: usdcBalance,
    isLoading: isLoadingUsdc,
    value: usdcValue,
  } = useBaseSepoliaUsdc();

  const detailItems: DetailItem[] = useMemo(() => {
    if (!isConnected) return [];
    return [
      {
        label: "Function",
        value: "SaharalabsAI/WeatherFunction",
        highlight: true,
        src: "https://github.com/SaharaLabsAI/weather-function-sample.git",
      },
      {
        label: "Connected Wallet",
        value: "Active",
        isStatus: true,
      },
      {
        label: "Address",
        value: address,
      },
      {
        label: "Balance",
        value: isLoadingUsdc ? "Loading..." : `${usdcBalance} USDC`,
      },
      {
        label: "Amount",
        value: "0.01 USDC",
      },
      {
        label: "Network",
        value: chain?.name || "",
      },
    ];
  }, [isConnected, address, isLoadingUsdc, usdcBalance, chain?.name]);

  const onPayment = () => {
    if (usdcValue < 0.01) {
      toast.error("Insufficient USDC balance");
      return;
    }

    fetchHandlerAsync()
      .then((res) => {
        toast.success("Payment successful");
        onPaymentSuccess?.(res.data);
      })
      .catch((err) => {
        toast.error(`Payment failed: ${err.message || err}`);
      });
  };
  return (
    <MotionCard className='w-2/3 mx-auto '>
      <div className='w-full'>
        {!isConnected && (
          <>
            <h3 className='text-2xl font-bold mb-4 text-center'>
              Connect Your Wallet
            </h3>
            <p className='text-gray-300 mb-8 text-center'>
              Connect your blockchain wallet to authenticate and pay for
              deployments. We support all major wallets through the X402
              protocol.
            </p>
          </>
        )}
        <div
          className={cn("space-y-4 text-center flex justify-center", {
            "justify-start": isConnected,
          })}
        >
          <ConnectButton />
        </div>
        {isConnected && (
          <div className='mt-8'>
            <DetailList items={detailItems} />

            <MotionButton
              className='mt-6 w-full'
              onClick={onPayment}
              loading={paymentLoading}
            >
              Pay now
            </MotionButton>
          </div>
        )}
      </div>
    </MotionCard>
  );
}
