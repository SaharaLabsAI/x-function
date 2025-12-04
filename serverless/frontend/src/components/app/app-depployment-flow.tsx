"use client";

import { AnimatePresence, motion } from "framer-motion";
import { useCallback, useMemo, useState } from "react";

import { AppDeployment } from "@/components/app/app-deployment";
import AppPayment from "@/components/app/app-payment";
import { ICreatedServiceInfo } from "@/types/app";

type DeploymentStep = "stepPayment" | "stepDeployment";

export function AppDeploymentFlow() {
  const [step, setStep] = useState<DeploymentStep>("stepPayment");
  const [serviceInfo, setServiceInfo] = useState<
    ICreatedServiceInfo | undefined
  >(undefined);

  const handlePaymentSuccess = useCallback((info: ICreatedServiceInfo) => {
    console.log("Payment successful, service info:", info);
    setServiceInfo(info);
    setStep("stepDeployment");
  }, []);

  const handleReset = useCallback(() => {
    setStep("stepPayment");
    setServiceInfo(undefined);
  }, []);

  const stepComponentMap = useMemo(
    () => ({
      stepPayment: {
        component: AppPayment,
        props: { onPaymentSuccess: handlePaymentSuccess },
        key: "payment",
      },
      stepDeployment: {
        component: AppDeployment,
        props: { serviceInfo, onReset: handleReset },
        key: "deployment",
      },
    }),
    [handlePaymentSuccess, handleReset, serviceInfo]
  );

  const currentStepConfig = stepComponentMap[step];
  const CurrentComponent = currentStepConfig.component;

  const pageVariants = {
    initial: { opacity: 0, y: 20 },
    animate: { opacity: 1, y: 0 },
    exit: { opacity: 0, y: -20 },
  };

  return (
    <div className='flex-1 container mx-auto pb-8'>
      <AnimatePresence mode='wait'>
        <motion.div
          key={currentStepConfig.key}
          variants={pageVariants}
          initial='initial'
          animate='animate'
          exit='exit'
          transition={{ duration: 0.3 }}
          className='w-full'
        >
          <CurrentComponent {...currentStepConfig.props} />
        </motion.div>
      </AnimatePresence>
    </div>
  );
}
