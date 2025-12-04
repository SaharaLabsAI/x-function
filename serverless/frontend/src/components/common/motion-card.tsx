"use client";

import { cubicBezier, motion } from "framer-motion";

import type { HTMLMotionProps } from "framer-motion";
import { ReactNode } from "react";
import { cn } from "@/lib/utils";

interface MotionCardProps extends HTMLMotionProps<"div"> {
  children: ReactNode;
  className?: string;
}

export function MotionCard({
  children,
  className = "",
  ...props
}: MotionCardProps) {
  const customEase = cubicBezier(0.22, 1, 0.36, 1); // easeOutExpo

  return (
    <motion.div
      initial={{ opacity: 0, y: 20, scale: 1 }}
      animate={{
        opacity: 1,
        y: 0,
        scale: 1.01,
      }}
      exit={{ opacity: 0, y: -20 }}
      transition={{
        duration: 0.5,
        ease: customEase,
      }}
      className={cn(
        `
        glass-effect
        rounded-xl
        p-10
        shadow-[0_20px_50px_rgba(248,255,173,0.15)]
        relative
        overflow-hidden
        transition-all duration-300
      `,
        className
      )}
      style={{
        boxShadow: "0 20px 50px rgba(248, 255, 173, 0.15)",
      }}
      {...props}
    >
      <div className='absolute inset-0 -z-10 opacity-20 pointer-events-none'>
        <div className='absolute top-0 -left-4 w-72 h-72 bg-primary/30 rounded-full blur-3xl' />
        <div className='absolute bottom-0 -right-4 w-72 h-72 bg-accent/20 rounded-full blur-3xl' />
      </div>
      <div className='absolute top-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-primary/50 to-transparent' />
      <div className='absolute bottom-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-primary/50 to-transparent' />
      <div className='absolute top-0 left-0 bottom-0 w-px bg-gradient-to-b from-transparent via-primary/50 to-transparent' />
      <div className='absolute top-0 right-0 bottom-0 w-px bg-gradient-to-b from-transparent via-primary/50 to-transparent' />
      <div className='relative z-10'>{children}</div>
    </motion.div>
  );
}
