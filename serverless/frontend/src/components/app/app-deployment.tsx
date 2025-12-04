"use client";

import { AnimatePresence, Variants, motion } from "framer-motion";
import { CircleEllipsis, CircleX, SquareCheck } from "lucide-react";
import { useEffect, useState } from "react";

import { AppResultContainer } from "./app-result-container";
import { ICreatedServiceInfo } from "@/types/app";
import MotionButton from "@/components/common/motion-button";
import { MotionCard } from "@/components/common/motion-card";
import { toast } from "sonner";
import { useServiceStatus } from "@/hooks/app/useGetServiceStatus";

interface AppDeploymentProps {
  serviceInfo?: ICreatedServiceInfo;
  onReset?: () => void;
}

const titleMap = {
  deploying: "Waiting for application ready",
  deployed: "Application Deployed Successfully",
};
const buttonTextMap = {
  deploying: "Deploying...",
  deployed: "Back",
};
const statusIconMap = {
  True: <SquareCheck className='text-success ml-2' />,
  False: <CircleX className='text-error ml-2' />,
  Unknown: <CircleEllipsis className='text-accent ml-2 animate-pulse' />,
};

const headerVariants: Variants = {
  initial: {
    opacity: 0,
    y: -20,
    scale: 0.95,
  },
  animate: {
    opacity: 1,
    y: 0,
    scale: 1,
    transition: {
      duration: 0.5,
      ease: [0.22, 1, 0.36, 1],
    },
  },
  exit: {
    opacity: 0,
    y: 20,
    scale: 0.95,
    transition: {
      duration: 0.3,
    },
  },
};

const mainVariants: Variants = {
  initial: {
    opacity: 0,
    y: 30,
  },
  animate: {
    opacity: 1,
    y: 0,
    transition: {
      delay: 0.2,
      duration: 0.5,
      ease: [0.22, 1, 0.36, 1],
    },
  },
  exit: {
    opacity: 0,
    y: -30,
    transition: {
      duration: 0.3,
    },
  },
};
type deployStatusType = keyof typeof titleMap;
export function AppDeployment({ serviceInfo, onReset }: AppDeploymentProps) {
  const [deployStatus, setDeployStatus] =
    useState<deployStatusType>("deploying");

  const { data: serviceDetail } = useServiceStatus({
    serviceId: serviceInfo?.id,
    enabled: deployStatus === "deploying",
    pollingInterval: 3000,
    onError: (error) => {
      toast.error(`Deployment error: ${error.message}`);
      onReset?.();
    },
    onSuccess: (data) => {
      if (data.ready) {
        setDeployStatus("deployed");
        toast.success("Deployment successful!");
      }
    },
  });

  if (!serviceInfo) {
    return (
      <MotionCard className='max-w-2xl mx-auto'>
        <div className='text-center text-gray-400'>
          No service information available
        </div>
      </MotionCard>
    );
  }

  const actionHandler = () => {
    onReset?.();
  };

  const detailsLogs = () => {
    const logRender = (logInfo: { phase: string; status: string }) => {
      return (
        <div className='flex items-start justify-between' key={logInfo.phase}>
          <div className='flex items-center'>
            <span
              className={`flex-shrink-0 w-6 h-6 rounded-full bg-primary flex items-center justify-center mr-3 mt-0.5`}
            />
            <p className='text-sm'>{logInfo.phase}</p>
          </div>

          <span>
            {statusIconMap[logInfo.status as keyof typeof statusIconMap]}
          </span>
        </div>
      );
    };
    return (
      <div className='bg-dark/50 p-4 rounded-lg border border-primary/20 h-48 overflow-y-auto'>
        <div className='space-y-3' id='activity-log'>
          {serviceDetail?.extra?.details?.map((item) => logRender(item))}
        </div>
      </div>
    );
  };

  return (
    <MotionCard className='w-3/4 mx-auto min-h-[100px] '>
      <motion.header
        key={deployStatus}
        variants={headerVariants}
        initial='initial'
        animate='animate'
        exit='exit'
        className='space-y-6 text-center text-2xl font-bold text-gradient-primary mb-2'
      >
        {titleMap[deployStatus]}
      </motion.header>
      <motion.main
        key={`main-${deployStatus}`}
        variants={mainVariants}
        initial='initial'
        animate='animate'
        exit='exit'
        className='mt-6'
      >
        {deployStatus === "deploying" && detailsLogs()}
        {deployStatus === "deployed" && (
          <AppResultContainer result={serviceDetail?.url} />
        )}
      </motion.main>
      <motion.footer
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{
          delay: 0.4,
          duration: 0.5,
          ease: [0.22, 1, 0.36, 1],
        }}
        className='mt-6 flex justify-center'
      >
        <MotionButton
          className='w-[200px]'
          loading={deployStatus === "deploying"}
          onClick={actionHandler}
        >
          <AnimatePresence mode='wait'>
            <motion.span
              key={`button-text-${deployStatus}`}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              transition={{ duration: 0.2 }}
            >
              {buttonTextMap[deployStatus]}
            </motion.span>
          </AnimatePresence>
        </MotionButton>
      </motion.footer>
    </MotionCard>
  );
}
