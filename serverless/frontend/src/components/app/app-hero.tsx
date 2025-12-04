"use client";

import { cn } from "@/lib/utils";

export default function AppHero({ className }: { className?: string }) {
  return (
    <section className={cn("mb-12 text-center relative z-10", className)}>
      <div className='absolute top-0 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-64 h-64 bg-primary/10 rounded-full filter blur-3xl'></div>
      <h2 className='text-3xl md:text-4xl font-bold mb-2 relative z-10'>
      DApp demo with x/function enabled
      </h2>
      <h1 className='text-4xl md:text-6xl font-bold mb-6 bg-clip-text text-transparent bg-gradient-to-r from-primary to-secondary relative z-10'>
      Run functions on-chain and pay for them
      </h1>
      <p className='text-gray-300 max-w-2xl mx-auto relative z-10'>
      Access to weather x/function by making a payment. To access the weather function, please pay $0.01 Base Sepolia USDC.
      </p>
      {/* font-style: italic */}
      <p className="text-gray-400 relative z-10 mt-2 italic ">
      Need Base Sepolia USDC?
      <a href="https://faucet.circle.com/" target="_blank" rel="noopener noreferrer"> Get some <u>here</u>.</a>
      </p>
    </section>
  );
}
