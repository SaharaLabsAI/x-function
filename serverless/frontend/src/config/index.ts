'use client';

import {
  binanceWallet,
  bitgetWallet,
  imTokenWallet,
  metaMaskWallet,
  okxWallet,
  rabbyWallet,
  tokenPocketWallet,
  trustWallet,
  walletConnectWallet,
} from '@rainbow-me/rainbowkit/wallets';
import { createStorage, http } from 'wagmi';

import { baseSepolia } from 'wagmi/chains';
import { getDefaultConfig } from '@rainbow-me/rainbowkit';

export const config = getDefaultConfig({
  appName: 'Git Repo Deploy Portal',
  projectId: process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID || '',
  chains: [baseSepolia],
  wallets: [
    {
      groupName: 'Recommended',
      wallets: [
        metaMaskWallet,
        okxWallet,
        rabbyWallet,
      ],
    },
    {
      groupName: 'Exchange Wallets',
      wallets: [
        binanceWallet,
        bitgetWallet,
      ],
    },
    {
      groupName: 'Mobile Wallets',
      wallets: [
        trustWallet,
        imTokenWallet,
        tokenPocketWallet,
      ],
    },
    {
      groupName: 'Others',
      wallets: [
        walletConnectWallet,
      ],
    },
  ],
  appDescription: 'download pre',
  ssr: true,
  storage: createStorage({
    storage: typeof window !== "undefined" ? window.localStorage : undefined,
  }),
  transports: {
    [baseSepolia.id]: http(),
  },
});