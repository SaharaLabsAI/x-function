"use client";

import { motion, type HTMLMotionProps } from "framer-motion";
import { cn } from "@/lib/utils";
import type { ReactNode } from "react";

interface MotionButtonProps
  extends Omit<HTMLMotionProps<"button">, "children"> {
  children: ReactNode;
  variant?: "primary" | "secondary" | "success" | "danger";
  loading?: boolean;
  className?: string;
}

const MotionButton = ({
  children,
  onClick,
  variant = "primary",
  disabled = false,
  loading = false,
  className = "",
  type = "button",
  ...props
}: MotionButtonProps) => {
  const baseClasses =
    "px-8 py-4 rounded-lg font-semibold transition-all duration-300 transform hover:-translate-y-1 focus:outline-none flex items-center justify-center cursor-pointer";

  const variantClasses = {
    primary:
      "bg-gradient-to-r from-primary to-secondary text-dark hover:shadow-lg hover:shadow-primary/30",
    secondary:
      "bg-dark-light text-white hover:bg-dark-lighter focus:ring-amber-500",
    success: "bg-green-500 text-white hover:bg-green-600 focus:ring-green-500",
    danger: "bg-red-500 text-white hover:bg-red-600 focus:ring-red-500",
  };

  return (
    <motion.button
      animate={
        loading
          ? {
              scale: [1, 1.02, 1],
              opacity: [1, 0.8, 1],
            }
          : {}
      }
      transition={
        loading
          ? {
              duration: 2,
              repeat: Infinity,
              ease: "easeInOut",
            }
          : {}
      }
      whileHover={!loading && !disabled ? { scale: 1.02 } : undefined}
      whileTap={!loading && !disabled ? { scale: 0.98 } : undefined}
      type={type}
      onClick={onClick}
      disabled={disabled || loading}
      className={cn(`${baseClasses} ${variantClasses[variant]} ${className}`, {
        "cursor-not-allowed opacity-60": disabled || loading,
      })}
      {...props}
    >
      {loading ? (
        <div className='flex items-center justify-center gap-2'>
          <motion.svg
            animate={{ rotate: 360 }}
            transition={{
              duration: 1,
              repeat: Infinity,
              ease: "linear",
            }}
            className='h-4 w-4 text-dark'
            xmlns='http://www.w3.org/2000/svg'
            fill='none'
            viewBox='0 0 24 24'
          >
            <circle
              className='opacity-25'
              cx='12'
              cy='12'
              r='10'
              stroke='currentColor'
              strokeWidth='4'
            />
            <path
              className='opacity-75'
              fill='currentColor'
              d='M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z'
            />
          </motion.svg>

          <motion.span
            animate={{
              opacity: [1, 0.5, 1],
            }}
            transition={{
              duration: 1.5,
              repeat: Infinity,
              ease: "easeInOut",
            }}
          >
            {children}
          </motion.span>
        </div>
      ) : (
        children
      )}
    </motion.button>
  );
};

export default MotionButton;
