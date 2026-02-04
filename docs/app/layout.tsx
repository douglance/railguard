import "./globals.css";
import { RootProvider } from "fumadocs-ui/provider";
import type { ReactNode } from "react";
import type { Metadata } from "next";
import localFont from "next/font/local";

const firaSans = localFont({
  src: [
    {
      path: "../public/fonts/FiraSans-Regular.ttf",
      weight: "400",
      style: "normal",
    },
    {
      path: "../public/fonts/FiraSans-Italic.ttf",
      weight: "400",
      style: "italic",
    },
    {
      path: "../public/fonts/FiraSans-Medium.ttf",
      weight: "500",
      style: "normal",
    },
    {
      path: "../public/fonts/FiraSans-MediumItalic.ttf",
      weight: "500",
      style: "italic",
    },
    {
      path: "../public/fonts/FiraSans-SemiBold.ttf",
      weight: "600",
      style: "normal",
    },
    {
      path: "../public/fonts/FiraSans-SemiBoldItalic.ttf",
      weight: "600",
      style: "italic",
    },
    {
      path: "../public/fonts/FiraSans-Bold.ttf",
      weight: "700",
      style: "normal",
    },
    {
      path: "../public/fonts/FiraSans-BoldItalic.ttf",
      weight: "700",
      style: "italic",
    },
  ],
  variable: "--font-fira-sans",
  display: "swap",
});

const firaCode = localFont({
  src: [
    {
      path: "../public/fonts/FiraCodeNerdFont-Light.ttf",
      weight: "300",
      style: "normal",
    },
    {
      path: "../public/fonts/FiraCodeNerdFont-Regular.ttf",
      weight: "400",
      style: "normal",
    },
    {
      path: "../public/fonts/FiraCodeNerdFont-Medium.ttf",
      weight: "500",
      style: "normal",
    },
    {
      path: "../public/fonts/FiraCodeNerdFont-SemiBold.ttf",
      weight: "600",
      style: "normal",
    },
    {
      path: "../public/fonts/FiraCodeNerdFont-Bold.ttf",
      weight: "700",
      style: "normal",
    },
  ],
  variable: "--font-fira-code",
  display: "swap",
});

export const metadata: Metadata = {
  title: {
    template: "%s | Railguard",
    default: "Railguard - The Firewall for Your Wallet",
  },
  description:
    "Local-first RPC proxy that secures EVM transactions. Deep Packet Inspection, policy enforcement, and fail-closed security for DeFi operators and protocol engineers.",
  openGraph: {
    title: "Railguard",
    description: "The Firewall for Your Wallet - Secure EVM transaction proxy",
    siteName: "Railguard",
  },
};

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <html
      lang="en"
      className={`${firaSans.variable} ${firaCode.variable}`}
      suppressHydrationWarning
    >
      <body className="flex min-h-screen flex-col font-sans">
        <RootProvider>{children}</RootProvider>
      </body>
    </html>
  );
}
