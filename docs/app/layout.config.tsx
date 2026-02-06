import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";
import { glyphs } from "@/components/nf-icon";
import { RailgunLogo } from "@/components/railgun-logo";

export const baseOptions: BaseLayoutProps = {
  nav: {
    title: (
      <span className="font-bold inline-flex items-center gap-2.5">
        <RailgunLogo size={22} />
        <span>
          Rail<span className="text-rust-500">gun</span>
        </span>
      </span>
    ),
  },
  links: [
    {
      text: "Documentation",
      url: "/docs",
      active: "nested-url",
      icon: <span className="font-mono text-sm">{glyphs.book}</span>,
    },
    {
      text: "GitHub",
      url: "https://github.com/LatitudeVentures/railgun",
      icon: <span className="font-mono text-sm">{glyphs.github}</span>,
    },
  ],
  githubUrl: "https://github.com/LatitudeVentures/railgun",
};
