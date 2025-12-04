import "./globals.css";

import { Geist, Geist_Mono } from "next/font/google";

import type { Metadata } from "next";
import { Providers } from "@/providers/wagmiProvider";
import { Toaster } from "@/components/ui/sonner";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "x/function extension demo",
  description: "x/function extension demo.",
  icons: [{ url: "/favicon.svg", type: "image/svg+xml" }],
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang='zh-CN' suppressHydrationWarning>
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        <Providers>
          <main className='h-screen overflow-auto'>{children}</main>
        </Providers>
        <Toaster
          position='top-center'
          richColors
          closeButton
          toastOptions={{
            classNames: {
              toast: "bg-dark-light border-dark-lighter",
              title: "text-secondary",
              description: "text-gray-400",
              error: "bg-error/10 border-error text-error",
              success: "bg-success/10 border-success text-success",
              warning: "bg-warning/10 border-warning text-warning",
              info: "bg-accent/10 border-accent text-accent",
            },
          }}
        />
      </body>
    </html>
  );
}
