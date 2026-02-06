"use client";

import Link from "next/link";
import { useState, useEffect } from "react";
import { motion, useReducedMotion, type Variants } from "framer-motion";
import { glyphs } from "@/components/nf-icon";
import { RailgunLogo } from "@/components/railgun-logo";

// Animation variants for reusability
const fadeInUp: Variants = {
  hidden: { opacity: 0, y: 20 },
  visible: { opacity: 1, y: 0 },
};

const staggerContainer: Variants = {
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: {
      staggerChildren: 0.15,
      delayChildren: 0.1,
    },
  },
};

const cardHover: Variants = {
  rest: { y: 0, boxShadow: "0 0 0 rgba(255, 79, 0, 0)" },
  hover: {
    y: -4,
    boxShadow: "0 8px 30px rgba(255, 79, 0, 0.15)",
    transition: { duration: 0.25, ease: "easeOut" },
  },
};

// "guard" entrance animation - scale up with slight overshoot
const gunEntrance: Variants = {
  hidden: { opacity: 0, scale: 0.8 },
  visible: {
    opacity: 1,
    scale: 1,
    textShadow: "0 0 12px rgba(255, 79, 0, 0.5)",
    transition: {
      duration: 0.5,
      ease: [0.34, 1.56, 0.64, 1],
    },
  },
};

const featureCards = [
  {
    title: "Fail Closed",
    icon: glyphs.shield,
    description: "If it crashes, nothing gets through. Panic becomes deny — zero secrets leak past the gate.",
  },
  {
    title: "Sub-Millisecond",
    icon: glyphs.bolt,
    description: "Pre-compiled regex and zero-copy inspection. < 1ms p99 latency — invisible to your workflow.",
  },
  {
    title: "Deep Inspection",
    icon: glyphs.telescope,
    description: "Scans for AWS keys, private keys, dangerous commands, protected paths, and exfiltration domains.",
  },
];

// Generate random gear config - randomized on each load
function generateGearConfig() {
  return Array.from({ length: 38 }).map((_, i) => {
    const sizeRand = Math.random();
    const size = Math.floor(Math.pow(sizeRand, 0.6) * 1990) + 10;

    const top = Math.random() * 140 - 20;
    const left = Math.random() * 140 - 20;

    const normalizedSize = (size - 10) / 1990;
    const baseOpacity = 0.03 + Math.pow(normalizedSize, 2.2) * 0.38;
    const opacityVariation = (Math.random() - 0.5) * 0.08;
    const opacity = Math.max(0.02, Math.min(0.45, baseOpacity + opacityVariation));

    const duration = 25 + Math.random() * 95;
    const direction = (Math.random() > 0.5 ? 1 : -1) as 1 | -1;

    return {
      size,
      opacity,
      top: `${top}%`,
      left: `${left}%`,
      duration,
      direction,
      delay: 0,
    };
  });
}

// Rust-style gear with thin ring and sharp triangular teeth
function RotatingGear({
  size,
  top,
  left,
  right,
  bottom,
  duration,
  direction = 1,
  delay = 0,
  opacity = 0.15,
}: {
  size: number;
  top?: string;
  left?: string;
  right?: string;
  bottom?: string;
  duration: number;
  direction?: 1 | -1;
  delay?: number;
  opacity?: number;
}) {
  const teeth = 32;
  const cx = size / 2;
  const cy = size / 2;
  const outerRadius = size * 0.45;
  const innerRadius = size * 0.38;
  const toothHeight = size * 0.08;
  const ringThickness = size * 0.04;

  const teethPath = Array.from({ length: teeth })
    .map((_, i) => {
      const angle1 = (i / teeth) * Math.PI * 2;
      const angle2 = ((i + 0.5) / teeth) * Math.PI * 2;
      const angle3 = ((i + 1) / teeth) * Math.PI * 2;

      const x1 = cx + Math.cos(angle1) * outerRadius;
      const y1 = cy + Math.sin(angle1) * outerRadius;

      const x2 = cx + Math.cos(angle2) * (outerRadius + toothHeight);
      const y2 = cy + Math.sin(angle2) * (outerRadius + toothHeight);

      const x3 = cx + Math.cos(angle3) * outerRadius;
      const y3 = cy + Math.sin(angle3) * outerRadius;

      return `L ${x1} ${y1} L ${x2} ${y2} L ${x3} ${y3}`;
    })
    .join(" ");

  const firstAngle = 0;
  const startX = cx + Math.cos(firstAngle) * outerRadius;
  const startY = cy + Math.sin(firstAngle) * outerRadius;

  return (
    <motion.div
      className="absolute"
      style={{ top, left, right, bottom }}
      initial={{ rotate: 0 }}
      animate={{ rotate: 360 * direction }}
      transition={{
        duration,
        repeat: Infinity,
        ease: "linear",
        delay,
      }}
    >
      <svg
        width={size}
        height={size}
        viewBox={`0 0 ${size} ${size}`}
        style={{ opacity }}
      >
        <path
          d={`M ${startX} ${startY} ${teethPath} Z`}
          fill="#FF4F00"
        />
        <circle
          cx={cx}
          cy={cy}
          r={innerRadius}
          fill="#0a0a0a"
        />
        <circle
          cx={cx}
          cy={cy}
          r={innerRadius}
          fill="none"
          stroke="#CC3D00"
          strokeWidth={ringThickness}
        />
        <circle
          cx={cx}
          cy={cy}
          r={size * 0.25}
          fill="#0a0a0a"
        />
      </svg>
    </motion.div>
  );
}

function FeatureCard({
  title,
  icon,
  description,
  index,
}: {
  title: string;
  icon: string;
  description: string;
  index: number;
}) {
  const shouldReduceMotion = useReducedMotion();

  return (
    <motion.div
      variants={fadeInUp}
      initial="hidden"
      whileInView="visible"
      viewport={{ once: true, margin: "-50px" }}
      transition={{
        duration: 0.5,
        delay: shouldReduceMotion ? 0 : index * 0.1,
        ease: [0.25, 0.1, 0.25, 1],
      }}
    >
      <motion.div
        className="rounded-lg border border-zinc-800 bg-zinc-900/50 p-6 cursor-default"
        variants={cardHover}
        initial="rest"
        whileHover="hover"
      >
        <motion.div
          className="mb-3 text-2xl text-rust-400 font-mono"
          whileHover={{ scale: 1.2, rotate: 5 }}
          transition={{ type: "spring", stiffness: 400, damping: 10 }}
        >
          {icon}
        </motion.div>
        <h3 className="mb-2 text-lg font-semibold text-rust-400">{title}</h3>
        <p className="text-sm text-zinc-400">{description}</p>
      </motion.div>
    </motion.div>
  );
}

function TypewriterCode() {
  return (
    <motion.div
      className="mt-12"
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: 0.8, duration: 0.5 }}
    >
      <motion.pre
        className="inline-flex items-center gap-3 rounded-lg bg-zinc-800/50 px-6 py-3 text-left text-sm border border-zinc-700/50"
        whileHover={{
          borderColor: "rgba(255, 79, 0, 0.3)",
          transition: { duration: 0.2 },
        }}
      >
        <span className="font-mono text-rust-500">{glyphs.terminal}</span>
        <code className="text-zinc-300">
          <motion.span
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 1.0 }}
          >
            cargo install railgun &&{" "}
          </motion.span>
          <motion.span
            className="text-rust-400"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 1.2 }}
          >
            railgun install
          </motion.span>
          <motion.span
            className="inline-block w-2 h-4 ml-1 bg-rust-400 align-middle"
            animate={{ opacity: [1, 0] }}
            transition={{ duration: 0.8, repeat: Infinity, repeatType: "reverse" }}
          />
        </code>
      </motion.pre>
    </motion.div>
  );
}

export default function HomePage() {
  const shouldReduceMotion = useReducedMotion();
  const [gears, setGears] = useState<ReturnType<typeof generateGearConfig>>([]);

  useEffect(() => {
    setGears(generateGearConfig());
  }, []);

  return (
    <main className="relative flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-zinc-900 to-black text-white overflow-hidden">
      {/* Giant rotating gears in background */}
      <div className="absolute inset-0 overflow-hidden">
        {gears.map((gear, i) => (
          <RotatingGear key={i} {...gear} />
        ))}

        {/* Heavy blur overlay to obscure the gears */}
        <div className="absolute inset-0 backdrop-blur-lg bg-gradient-to-b from-zinc-900/60 via-zinc-900/40 to-black/60" />
      </div>

      <div className="container relative z-10 mx-auto px-4 text-center">
        {/* Hero Section with staggered animations */}
        <motion.div
          variants={staggerContainer}
          initial="hidden"
          animate="visible"
        >
          {/* Logo */}
          <motion.div
            className="mb-4 sm:mb-6 mt-8 sm:mt-0 flex justify-center"
            variants={fadeInUp}
            transition={{ duration: 0.6, ease: [0.25, 0.1, 0.25, 1] }}
          >
            <RailgunLogo size={80} />
          </motion.div>
          <motion.h1
            className="mb-4 text-6xl font-bold tracking-tight"
            variants={fadeInUp}
            transition={{ duration: 0.6, ease: [0.25, 0.1, 0.25, 1] }}
          >
            Rail
            <motion.span
              className="text-rust-400 inline-block"
              variants={shouldReduceMotion ? fadeInUp : gunEntrance}
            >
              gun
            </motion.span>
          </motion.h1>

          {/* Subtitle */}
          <motion.p
            className="mb-8 text-xl text-zinc-400"
            variants={fadeInUp}
            transition={{ duration: 0.6, ease: [0.25, 0.1, 0.25, 1] }}
          >
            The Security Hook for Claude Code — Stop secrets leakage, dangerous commands, and data exfiltration
          </motion.p>

          {/* Buttons */}
          <motion.div
            className="flex flex-col items-center gap-4 sm:flex-row sm:justify-center"
            variants={fadeInUp}
            transition={{ duration: 0.6, ease: [0.25, 0.1, 0.25, 1] }}
          >
            <motion.div
              whileHover={{ scale: 1.03 }}
              whileTap={{ scale: 0.98 }}
              transition={{ type: "spring", stiffness: 400, damping: 17 }}
            >
              <Link
                href="/docs"
                className="inline-flex items-center gap-2 rounded-lg bg-rust-500 px-8 py-3 font-semibold text-white transition hover:bg-rust-600"
              >
                <span className="font-mono">{glyphs.rocket}</span>
                Get Started
              </Link>
            </motion.div>
            <motion.div
              whileHover={{ scale: 1.03 }}
              whileTap={{ scale: 0.98 }}
              transition={{ type: "spring", stiffness: 400, damping: 17 }}
            >
              <Link
                href="/docs/configuration"
                className="inline-flex items-center gap-2 rounded-lg border border-zinc-700 px-8 py-3 font-semibold text-white transition hover:border-zinc-500 hover:bg-zinc-800"
              >
                <span className="font-mono">{glyphs.book}</span>
                Configuration
              </Link>
            </motion.div>
          </motion.div>
        </motion.div>

        {/* Feature Cards with scroll-triggered reveal */}
        <div className="mt-16 grid gap-8 sm:grid-cols-3">
          {featureCards.map((card, index) => (
            <FeatureCard
              key={card.title}
              title={card.title}
              icon={card.icon}
              description={card.description}
              index={index}
            />
          ))}
        </div>

        {/* Code snippet with typewriter effect */}
        <TypewriterCode />
      </div>
    </main>
  );
}
