import type { NextConfig } from "next";

const isDev = process.env.NODE_ENV === "development";
console.log("isDev:", isDev);
const nextConfig: NextConfig = {
  /* config options here */
  basePath: "",
  assetPrefix: "",
  output: "export",
  trailingSlash: false,
  images: {
    unoptimized: true,
  },
  async rewrites() {
    if (!isDev) return [];

    return [
      {
        source: "/apis/x402/:path*",
        destination: process.env.NEXT_PUBLIC_SERVER_URL
          ? `${process.env.NEXT_PUBLIC_SERVER_URL}/:path*`
          : "http://localhost:3000/apis/x402/:path*",
      }
    ];
  },
};

export default nextConfig;
