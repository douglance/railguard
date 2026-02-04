import type { Config } from "tailwindcss";
import { createPreset } from "fumadocs-ui/tailwind-plugin";

const config: Config = {
  content: [
    "./components/**/*.{ts,tsx}",
    "./app/**/*.{ts,tsx}",
    "./content/**/*.{md,mdx}",
    "./mdx-components.tsx",
    "./node_modules/fumadocs-ui/dist/**/*.js",
  ],
  presets: [createPreset()],
  theme: {
    extend: {
      fontFamily: {
        sans: ["var(--font-fira-sans)", "system-ui", "sans-serif"],
        mono: ["var(--font-fira-code)", "ui-monospace", "monospace"],
      },
      colors: {
        // International Orange color palette
        // Base: #FF4F00 (International Orange) at 500
        rust: {
          50: "#fff8f5",
          100: "#ffefe6",
          200: "#ffdacc",
          300: "#ffb899",
          400: "#ff8c4d",
          500: "#FF4F00",  // International Orange - primary brand color
          600: "#e64700",
          700: "#cc3d00",
          800: "#a63200",
          900: "#802600",
          950: "#4d1700",
        },
      },
    },
  },
};

export default config;
