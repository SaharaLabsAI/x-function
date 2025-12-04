"use client";

import React from "react";
import { RocketIcon } from "lucide-react";

const Header: React.FC = () => {
  return (
    <header className='relative z-10 py-6 px-4 md:px-8 flex justify-between items-center border-b border-primary/20'>
      <div className='flex items-center space-x-2'>
        <div className='w-10 h-10 rounded-lg bg-gradient-to-r from-primary to-secondary flex items-center justify-center animate-glow'>
          <RocketIcon className=' h-5 w-5 text-dark' />
        </div>
        <div>
          <div className='text-xs text-primary font-semibold tracking-widest'>
            x/function
          </div>
          <h1 className='text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-primary to-secondary text-gradient-primary'>
            Pay for function call
          </h1>
        </div>
      </div>
    </header>
  );
};

export default Header;
