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
    template: "%s | Railgun",
    default: "Railgun - The Security Hook for Claude Code",
  },
  description:
    "Protect your AI coding sessions. Railgun intercepts Claude Code tool calls to block secrets leakage, dangerous commands, and data exfiltration with sub-millisecond, fail-closed security.",
  openGraph: {
    title: "Railgun",
    description: "The Security Hook for Claude Code — block secrets, dangerous commands, and exfiltration",
    siteName: "Railgun",
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
