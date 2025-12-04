"use client";

import { AnimatePresence, motion } from "framer-motion";
import { Check, Copy, Loader2 } from "lucide-react";

import dynamic from "next/dynamic";
import { useGetWeatherInfo } from "@/hooks/app/useGetWeatherInfo";
import { useState } from "react";

const ReactJson = dynamic(() => import("react-json-view"), { ssr: false });

export const AppResultContainer = ({ result = "" }: { result?: string }) => {
  const [copied, setCopied] = useState(false);
  const { data, loading } = useGetWeatherInfo({ host: result });

  const handleCopy = async () => {
    await navigator.clipboard.writeText(JSON.stringify(data, null, 2));
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.4, ease: [0.22, 1, 0.36, 1] }}
      className='mt-6'
    >
      <div className='glass-effect border border-primary/30 rounded-xl p-6 relative overflow-hidden'>
        <div className='absolute inset-0 -z-10 opacity-10 pointer-events-none'>
          <div className='absolute top-0 right-0 w-32 h-32 bg-primary/40 rounded-full blur-2xl' />
          <div className='absolute bottom-0 left-0 w-32 h-32 bg-accent/30 rounded-full blur-2xl' />
        </div>

        <div className='flex items-center justify-between mb-4'>
          <h3 className='text-lg font-semibold text-primary'>
            Function Result
          </h3>
          <button
            onClick={handleCopy}
            disabled={loading || !data}
            className='flex items-center gap-2 px-3 py-1.5 rounded-lg bg-primary/10 hover:bg-primary/20 transition-colors text-sm disabled:opacity-50 disabled:cursor-not-allowed'
          >
            {copied ? (
              <>
                <Check className='w-4 h-4 text-green-500' />
                <span>Copied!</span>
              </>
            ) : (
              <>
                <Copy className='w-4 h-4' />
                <span>Copy</span>
              </>
            )}
          </button>
        </div>

        <AnimatePresence mode='wait'>
          {loading ? (
            <motion.div
              key='loading'
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className='flex flex-col items-center justify-center py-12'
            >
              <Loader2 className='w-12 h-12 text-primary animate-spin mb-4' />
              <p className='text-sm text-muted-foreground'>Fetching data...</p>
            </motion.div>
          ) : data ? (
            <motion.div
              key='data'
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className='max-h-[400px] overflow-auto'
            >
              <ReactJson
                src={data}
                theme='monokai'
                collapsed={2}
                displayDataTypes={false}
                displayObjectSize={true}
                enableClipboard={true}
                indentWidth={2}
                name={false}
                style={{
                  background: "rgba(0, 0, 0, 0.2)",
                  padding: "1rem",
                  borderRadius: "0.5rem",
                  fontSize: "0.875rem",
                }}
              />
            </motion.div>
          ) : (
            <motion.div
              key='empty'
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className='flex flex-col items-center justify-center py-12 text-muted-foreground'
            >
              <p className='text-sm'>No data available</p>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  );
};
