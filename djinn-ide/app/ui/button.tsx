import clsx from "clsx";
import { type Icon as IconType } from "@phosphor-icons/react";

import type { ButtonHTMLAttributes } from "react";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "ghost" | "primary";
  disabled?: boolean;
  onClick?: () => void;
  icon?: IconType;
  "aria-label"?: string;
}

export default function Button({
  children,
  icon,
  variant = "primary",
  className,
  "aria-label": ariaLabel,
  ...props
}: ButtonProps) {
  const Icon = icon;

  return (
    <button
      className={clsx(
        variant === "ghost" &&
          "bg-transparent text-current transition-colors duration-250",
        variant === "primary" && [
          "bg-burst text-paper font-bold text-lg capitalize px-4 py-2 rounded-sharp border-ui border-ink shadow-grotesk-ink",
          "hover:-translate-x-px hover:-translate-y-px hover:bg-burst-400 hover:text-paper hover:shadow-grotesk-ink-pop",
          "active:translate-x-px active:translate-y-px active:shadow-grotesk-ink-pressed",
        ],
        "transition-[transform,box-shadow,background-color,color] duration-150",
        "disabled:translate-none disabled:shadow-grotesk-ink",
        className,
      )}
      {...props}
    >
      {Icon && (
        <Icon size={24} aria-hidden={!ariaLabel} aria-label={ariaLabel} />
      )}
      {children}
    </button>
  );
}
