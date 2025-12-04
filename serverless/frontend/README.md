# x-function-demo-app

[![Next.js](https://img.shields.io/badge/Next.js-15.x-black?style=flat-square&logo=next.js)](https://nextjs.org/)
[![React](https://img.shields.io/badge/React-19.x-blue?style=flat-square&logo=react)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue?style=flat-square&logo=typescript)](https://www.typescriptlang.org/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-4.x-38B2AC?style=flat-square&logo=tailwind-css)](https://tailwindcss.com/)

**x-function-demo-app** is a modern web application that enables users to deploy applications from Git repositories using blockchain payments. The platform integrates with the X402 protocol to facilitate seamless on-chain transactions for accessing and deploying functions.

If you're using this demo, please **★Star** this repository to show your interest!

## ✨ Features

- 🚀 **Cutting-edge Tech Stack**: Next.js 15 + React 19 + TypeScript + Tailwind CSS 4
- 🎨 **Rich UI Components**: 30+ exquisite components based on shadcn/ui
- 🌙 **Theme Switching**: Built-in dark/light themes with automatic system theme detection
- 📱 **Responsive Design**: Perfectly adapts to all device sizes
- ⚡ **High Performance**: Supports Turbopack for excellent build and development experience
- 🎭 **Animation Effects**: Integrated with Motion animation library for rich interactive experiences
- 🛠️ **Best Practices**: Code standards, type safety, and component-driven development
- 🌍 **Internationalization Support**: Integrated with i18next for easy multi-language implementation
- 📦 **State Management**: Integrated with Zustand lightweight state management
- 🎯 **Development Experience**: Complete development toolchain and ESLint configuration
- 🔗 **Blockchain Integration**: Connect with wallets and process payments using the X402 protocol
- 🚢 **Deployment Flow**: Seamless application deployment from Git repositories with status tracking

### Key Components

| Component                                                           | Purpose                                          |
| ------------------------------------------------------------------- | ------------------------------------------------ |
| [app-deployment-flow](/src/components/app/app-depployment-flow.tsx) | Manages the multi-step deployment process        |
| [app-payment](/src/components/app/app-payment.tsx)                  | Handles wallet connection and payment processing |
| [app-deployment](/src/components/app/app-deployment.tsx)            | Tracks and displays deployment status            |
| [useGetServiceStatus](/src/hooks/app/useGetServiceStatus.ts)        | Custom hook for polling deployment status        |
| [use-x402-fetch](/src/hooks/common/use-x402-fetch.ts)               | Custom hook for pay with x402-fetch              |

## Screenshots

| Payment Screen                                                                                | Deployment Status Screen                                                                               |
| --------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| [![Screenshot of payment screen](/docs/img/payment-screen.png)](/docs/img/payment-screen.png) | [![Screenshot of deployment status](/docs/img/deployment-status.png)](/docs/img/deployment-status.png) |

## 🚀 Quick Start

### Prerequisites

- Node.js >= 20.0 (recommended, required by shadcn/ui), minimum >= 18.17 for development
- yarn >= 3.0 (recommended) or yarn 4.1.1

### Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/SaharaLabsAI/x-function-demo-app.git
   cd x-function-demo-app
   ```

2. **Install Dependencies**

   ```bash
   yarn install
   ```

3. **Start the Development Server**

   ```bash
   yarn dev
   ```

4. **Access the Application**

   Open [http://localhost:3000](http://localhost:3000) to view the application

## 📝 Available Scripts

```bash
# Development mode (with Turbopack)
yarn dev

# Build for production
yarn build

# Start production server
yarn start

# Static file serving (for static export)
yarn serve

# Lint code
yarn lint
```

### Static Export Mode

If `output: 'export'` is configured in `next.config.ts`, `yarn start` cannot be used. Follow these steps instead:

```bash
# 1. Build static files
yarn build

# 2. Start static file server
yarn serve
```

> **Note**: Static export mode builds the application into pure static files, output to the `out` directory. Suitable for static hosting services like GitHub Pages, Netlify, etc.

## 🛠️ Tech Stack

### Core Frameworks

- **Next.js 15** - React full-stack framework
- **React 19** - UI construction library
- **TypeScript 5** - Type-safe JavaScript

### Styling and UI

- **Tailwind CSS 4** - Utility-first CSS framework
- **shadcn/ui** - High-quality React component library
- **Radix UI** - Accessible low-level UI primitives
- **Lucide React** - Elegant icon library

### Animation and Interaction

- **Motion** - Powerful animation library
- **React Spring** - Spring physics-based animations

### State Management and Tools

- **Zustand** - Lightweight state management
- **ahooks** - Practical React Hooks library
- **React Use** - Collection of commonly used React Hooks

### Development Tools

- **ESLint** - Code linting
- **Prettier** - Code formatting
- **Turbopack** - Blazing-fast build tool

### Blockchain Integration

- **RainbowKit** - Wallet connection UI
- **Wagmi** - Ethereum tools for React
- **X402 Protocol** - Payment processing

## 📁 Project Structure

```text
x-function-demo-app/
├── app/                    # Next.js app directory
│   ├── favicon.ico
│   ├── globals.css        # Global styles
│   ├── layout.tsx         # Root layout
│   └── page.tsx           # Home page
├── components/            # React components
├── hooks/                # Custom Hooks
├── lib/                  # Utility functions
├── public/               # Static assets
├── components.json       # shadcn/ui configuration
├── next.config.ts        # Next.js configuration
├── tailwind.config.js    # Tailwind CSS configuration
└── tsconfig.json         # TypeScript configuration
```

## 🎨 UI Components

- **Layout Components**: Card, Sheet, Sidebar, Resizable
- **Navigation Components**: Navigation Menu, Breadcrumb, Pagination
- **Form Components**: Button, Input, Select, Textarea, Switch
- **Feedback Components**: Alert Dialog, Toast, Progress, Skeleton
- **Data Display**: Table, Avatar, Badge, Tooltip
- **Others**: Tabs, Dropdown Menu, Popover, Slider, etc.

All components support theme switching, with complete type definitions and accessibility support.

## 🌙 Theme System

Built-in complete theme system:

- 🌞 Light theme
- 🌙 Dark theme
- 🔄 Automatic system theme detection
- 🎨 Customizable theme colors

Implemented with `next-themes`, supporting SSR flicker-free switching.

## 📦 Deployment

### Vercel Deployment (Recommended)

The easiest way to deploy is using the [Vercel platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme):

### Static Export Deployment

For deployment to static hosting services (such as GitHub Pages, Netlify Static Hosting, etc.), use static export mode:

1. **Configure Static Export**

   Add to `next.config.ts`:

   ```typescript
   const nextConfig: NextConfig = {
     output: "export",
     trailingSlash: true,
     images: {
       unoptimized: true,
     },
   };
   ```

2. **Build and Deploy**

   ```bash
   # Build static files
   yarn build

   # Local preview (optional)
   yarn serve

   # Deploy the out directory to your static hosting service
   ```

### Other Deployment Options

- **Netlify**: Supports both static export and server-side rendering
- **Railway**: Simple full-stack deployment
- **Docker**: Containerized deployment
- **Self-hosting**: Use `yarn build` and `yarn start`

For detailed deployment guides, refer to the [Next.js Deployment Documentation](https://nextjs.org/docs/app/building-your-application/deploying).

## 📄 License

This project is licensed under the [MIT License](LICENSE).

This means you are free to:

- ✅ Use commercially
- ✅ Modify the code
- ✅ Distribute the code
- ✅ Use privately

The only requirement is to include the original license and copyright notice.

---
