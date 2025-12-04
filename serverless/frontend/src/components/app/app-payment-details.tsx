"use client";

import { ExternalLink } from "lucide-react";
import { cn } from "@/lib/utils";
import { motion } from "framer-motion";

export interface DetailItem {
  label: string;
  value?: string;
  isStatus?: boolean;
  highlight?: boolean;
  src?: string;
}

interface DetailListProps {
  items: DetailItem[];
  className?: string;
}

export function DetailList({ items, className }: DetailListProps) {
  if (!items.length) return null;

  return (
    <div
      className={cn(
        "bg-dark/50 p-6 rounded-lg border border-primary/20",
        className
      )}
    >
      {items.map((item, index) => (
        <DetailListItem
          key={`${item.label}-${index}`}
          item={item}
          index={index}
          isLast={index === items.length - 1}
        />
      ))}
    </div>
  );
}

interface DetailListItemProps {
  item: DetailItem;
  index: number;
  isLast: boolean;
}

function DetailListItem({ item, index, isLast }: DetailListItemProps) {
  return (
    <motion.div
      initial={{ opacity: 0, x: -10 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ delay: index * 0.05, duration: 0.3 }}
      className={cn(
        "flex justify-between items-center transition-all duration-300",
        !isLast && "mb-4",
        item.highlight && [
          "p-3 -mx-3 rounded-lg",
          "bg-primary/10 border border-primary/30",
          "shadow-lg shadow-primary/10",
          "hover:bg-primary/15 hover:border-primary/40 hover:shadow-primary/20",
        ]
      )}
    >
      {/* Label */}
      <span
        className={cn(
          "text-gray-400 transition-colors font-medium",
          item.highlight && "text-primary font-semibold"
        )}
      >
        {item.label}
      </span>

      <div className='flex items-center gap-2'>
        <span
          className={cn("transition-all", {
            "text-xs px-3 py-1.5 bg-primary/20 text-secondary rounded-full font-medium":
              item.isStatus,
            "font-mono text-sm text-secondary":
              !item.isStatus && !item.highlight,
            "font-mono text-base font-bold text-primary":
              !item.isStatus && item.highlight,
          })}
        >
          {item.value}
        </span>

        {item.src && (
          <a
            href={item.src}
            target='_blank'
            rel='noopener noreferrer'
            className='p-1.5 rounded-md hover:bg-primary/20 transition-colors group'
            aria-label={`Open ${item.label}`}
          >
            <ExternalLink className='w-4 h-4 text-primary/70 group-hover:text-primary transition-colors' />
          </a>
        )}
      </div>
    </motion.div>
  );
}
