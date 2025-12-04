"use client";

import { AppDeploymentFlow } from "@/components/app/app-depployment-flow";
import AppHeader from "@/components/app/app-header";
import AppHero from "@/components/app/app-hero";
import { ParticlesBackground } from "@/components/common/particles-bg";

export default function App() {
  return (
    <div className='h-full bg-dark-light overflow-auto'>
      <ParticlesBackground />
      <main className='z-10 h-full flex flex-col'>
        <AppHeader />
        <AppHero className='mt-10' />
        <AppDeploymentFlow />
      </main>
    </div>
  );
}
