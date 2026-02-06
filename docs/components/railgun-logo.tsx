/**
 * Railgun Logo - Electromagnetic railgun
 * Works on both light and dark backgrounds
 */

interface RailgunLogoProps {
  size?: number;
  className?: string;
}

/**
 * Railgun icon: barrel with electromagnetic coils and muzzle flash
 */
export function RailgunLogo({ size = 20, className = "" }: RailgunLogoProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      className={className}
      aria-hidden="true"
      fill="none"
    >
      {/* Barrel outline */}
      <rect
        x="2"
        y="8"
        width="15"
        height="8"
        rx="1.5"
        fill="none"
        stroke="#FF4F00"
        strokeWidth="1.5"
      />
      {/* Inner rails */}
      <line x1="2" y1="10.5" x2="17" y2="10.5" stroke="#FF4F00" strokeWidth="0.75" />
      <line x1="2" y1="13.5" x2="17" y2="13.5" stroke="#FF4F00" strokeWidth="0.75" />
      {/* Electromagnetic coils (extend beyond barrel, increasing intensity) */}
      <line x1="6" y1="6.5" x2="6" y2="17.5" stroke="#FF4F00" strokeWidth="1.5" strokeLinecap="round" opacity="0.4" />
      <line x1="10" y1="6.5" x2="10" y2="17.5" stroke="#FF4F00" strokeWidth="1.5" strokeLinecap="round" opacity="0.55" />
      <line x1="14" y1="6.5" x2="14" y2="17.5" stroke="#FF4F00" strokeWidth="1.5" strokeLinecap="round" opacity="0.7" />
      {/* Muzzle flash */}
      <path
        d="M17 12h4.5M19.5 9.5L22 12l-2.5 2.5"
        stroke="#FF4F00"
        strokeWidth="1.5"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

/**
 * Alternative: Railgun barrel with checkmark for "verified/safe"
 */
export function RailgunLogoCheck({ size = 20, className = "" }: RailgunLogoProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      className={className}
      aria-hidden="true"
      fill="none"
    >
      {/* Barrel body */}
      <rect
        x="2"
        y="7"
        width="16"
        height="10"
        rx="2"
        fill="#FF4F00"
      />
      {/* Checkmark knocked out */}
      <path
        d="M7 12l3 3 5-6"
        stroke="#0a0a0a"
        strokeWidth="2.5"
        strokeLinecap="round"
        strokeLinejoin="round"
        fill="none"
      />
      {/* Muzzle flash */}
      <path
        d="M18 12h3.5M20 9.5L22 12l-2 2.5"
        stroke="#FF4F00"
        strokeWidth="1.5"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}
