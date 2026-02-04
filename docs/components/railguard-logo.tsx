/**
 * Railguard Logo - Shield with rail/track design
 * Works on both light and dark backgrounds
 */

interface RailguardLogoProps {
  size?: number;
  className?: string;
}

/**
 * Shield logo with stylized rail/track lines
 */
export function RailguardLogo({ size = 20, className = "" }: RailguardLogoProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      className={className}
      aria-hidden="true"
      fill="none"
    >
      {/* Shield outline */}
      <path
        d="M12 2L3 6v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V6l-9-4z"
        fill="#FF4F00"
      />
      {/* Inner shield cutout for depth */}
      <path
        d="M12 4L5 7.2v4.8c0 4.44 3.07 8.59 7 9.6 3.93-1.01 7-5.16 7-9.6V7.2L12 4z"
        fill="#0a0a0a"
      />
      {/* Rail tracks - horizontal lines */}
      <path
        d="M7 9h10M7 12h10M7 15h10"
        stroke="#FF4F00"
        strokeWidth="1.5"
        strokeLinecap="round"
      />
      {/* Vertical rails/ties */}
      <path
        d="M9 8v8M12 8v8M15 8v8"
        stroke="#FF4F00"
        strokeWidth="0.75"
        strokeLinecap="round"
        opacity="0.6"
      />
    </svg>
  );
}

/**
 * Alternative: Simplified shield with checkmark for "guarded"
 */
export function RailguardLogoCheck({ size = 20, className = "" }: RailguardLogoProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      className={className}
      aria-hidden="true"
      fill="none"
    >
      {/* Shield */}
      <path
        d="M12 2L3 6v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V6l-9-4z"
        fill="#FF4F00"
      />
      {/* Checkmark knocked out */}
      <path
        d="M8 12l3 3 5-6"
        stroke="#0a0a0a"
        strokeWidth="2.5"
        strokeLinecap="round"
        strokeLinejoin="round"
        fill="none"
      />
    </svg>
  );
}
