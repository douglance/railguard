/**
 * Nerd Font Icon Component
 * Glyphs for the Railguard documentation
 *
 * These glyphs require a Nerd Font to render properly.
 * See: https://www.nerdfonts.com/cheat-sheet
 */

import type { HTMLAttributes } from "react";

/**
 * Space-themed Nerd Font glyphs
 * Unicode values from Nerd Fonts v3.x
 */
export const glyphs = {
  // Primary - Space/NASA theme
  rocket: "\uf135",           // nf-fa-rocket - launches, speed, performance
  terminal: "\uf489",         // nf-oct-terminal - code, CLI
  star: "\uf005",             // nf-fa-star - features, highlights
  starFull: "\ueb59",         // nf-cod-star_full - filled star
  bolt: "\uf0e7",             // nf-fa-bolt - speed, performance
  satellite: "\udb80\udd2f",  // nf-md-satellite_variant - streaming, data
  telescope: "\udb81\ude9a",  // nf-md-telescope - inspection, viewing
  orbit: "\udb82\udc42",      // nf-md-orbit - cycles, flow

  // Status indicators
  check: "\uf42e",            // nf-oct-check - success, valid
  checkCircle: "\udb80\udd50",// nf-md-check_circle - success filled
  x: "\uf467",                // nf-oct-x - error, invalid
  xCircle: "\udb80\udd41",    // nf-md-close_circle - error filled
  warning: "\uea6c",          // nf-cod-warning - caution
  info: "\uea74",             // nf-cod-info - information

  // Development
  gear: "\ueb51",             // nf-cod-gear - settings, config
  code: "\udb81\udc00",       // nf-md-code_tags - code blocks
  package: "\udb81\ude95",    // nf-md-package_variant - packages, npm
  file: "\udb80\udd14",       // nf-md-file_document - documentation
  folder: "\uf413",           // nf-oct-file_directory - directories
  git: "\uf1d3",              // nf-fa-git - version control

  // Navigation
  arrowRight: "\uf432",       // nf-oct-arrow_right
  arrowLeft: "\uf430",        // nf-oct-arrow_left
  arrowUp: "\uf431",          // nf-oct-arrow_up
  arrowDown: "\uf433",        // nf-oct-arrow_down
  chevronRight: "\ueab6",     // nf-cod-chevron_right
  chevronDown: "\ueab4",      // nf-cod-chevron_down
  link: "\uf44c",             // nf-oct-link
  externalLink: "\uf465",     // nf-oct-link_external

  // Actions
  play: "\uf40a",             // nf-oct-play
  copy: "\uea8a",             // nf-cod-copy
  search: "\uf422",           // nf-oct-search
  download: "\uf409",         // nf-oct-download
  upload: "\uf40b",           // nf-oct-upload

  // Misc
  lightbulb: "\uf400",        // nf-oct-light_bulb - tips, ideas
  book: "\uf405",             // nf-oct-book - docs, guides
  shield: "\udb80\udce4",     // nf-md-shield_check - security
  flame: "\uf490",            // nf-oct-flame - hot, trending
  zap: "\uf0e7",              // nf-fa-bolt - speed (alias)
  beaker: "\uf499",           // nf-oct-beaker - experimental

  // Branding
  rust: "\ue7a8",             // nf-dev-rust - Rust language
  comet: "\ue26d",            // nf-fae-comet - fast, fiery, unique
  typescript: "\ue628",       // nf-seti-typescript - TypeScript
  javascript: "\ue781",       // nf-dev-javascript - JavaScript
  github: "\uf408",           // nf-oct-mark_github - GitHub
} as const;

export type GlyphName = keyof typeof glyphs;

interface NFIconProps extends HTMLAttributes<HTMLSpanElement> {
  name: GlyphName;
  size?: "xs" | "sm" | "md" | "lg" | "xl" | "2xl";
  className?: string;
}

const sizeClasses = {
  xs: "text-xs",
  sm: "text-sm",
  md: "text-base",
  lg: "text-lg",
  xl: "text-xl",
  "2xl": "text-2xl",
} as const;

/**
 * Nerd Font Icon component
 *
 * @example
 * <NFIcon name="rocket" size="lg" className="text-rust-500" />
 * <NFIcon name="check" className="text-green-500" />
 */
export function NFIcon({
  name,
  size = "md",
  className = "",
  ...props
}: NFIconProps) {
  return (
    <span
      className={`inline-block font-mono leading-none ${sizeClasses[size]} ${className}`}
      aria-hidden="true"
      {...props}
    >
      {glyphs[name]}
    </span>
  );
}

// Note: All glyph spans MUST use font-mono class to render correctly
// Only FiraCode Nerd Font (mono) has the glyphs, not Fira Sans

/**
 * Inline icon for use in text
 * Slightly smaller with proper vertical alignment
 */
export function NFInline({
  name,
  className = "",
  ...props
}: Omit<NFIconProps, "size">) {
  return (
    <span
      className={`inline font-mono align-middle ${className}`}
      aria-hidden="true"
      {...props}
    >
      {glyphs[name]}
    </span>
  );
}

// Direct glyph usage example:
// <span className="font-mono text-rust-500">{glyphs.checkCircle}</span>

/**
 * Icon with label for navigation items
 */
interface NFLabelProps extends NFIconProps {
  label: string;
  labelClassName?: string;
}

export function NFLabel({
  name,
  label,
  size = "md",
  className = "",
  labelClassName = "",
  ...props
}: NFLabelProps) {
  return (
    <span className={`inline-flex items-center gap-2 ${className}`} {...props}>
      <NFIcon name={name} size={size} />
      <span className={labelClassName}>{label}</span>
    </span>
  );
}
