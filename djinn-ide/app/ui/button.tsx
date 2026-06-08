import { type Icon as IconType } from "@phosphor-icons/react";
import { cn } from "~/utils/cn";

import type { ButtonHTMLAttributes } from "react";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "ghost" | "primary" | "destructive";
  disabled?: boolean;
  onClick?: () => void;
  icon?: IconType;
  iconSize?: number;
  iconClassName?: string;
  iconKey?: string | number;
  "aria-label"?: string;
}

export default function Button({
  children,
  icon,
  iconSize = 16,
  variant = "primary",
  className,
  iconClassName,
  iconKey,
  "aria-label": ariaLabel,
  ...props
}: ButtonProps) {
  const Icon = icon;

  return (
    <button
      title={ariaLabel}
      className={cn(
        variant === "ghost" &&
          "bg-transparent transition-colors duration-250 font-bold text-sand-600 hover:text-ink",
        children && "px-4 py-2",
        variant !== "ghost" && [
          "shadow-grotesk-ink",
          "hover:-translate-x-px hover:-translate-y-px hover:shadow-grotesk-ink-pop",
          "active:translate-x-px active:translate-y-px active:shadow-grotesk-ink-pressed",
          "rounded-sharp border-ui border-ink ",
        ],
        variant === "primary" && "bg-burst text-paper hover:bg-burst-400",
        variant === "destructive" &&
          "bg-error-700 text-paper hover:bg-error-400",
        "transition-[transform,box-shadow,background-color,color] duration-150",
        "disabled:translate-none disabled:shadow-grotesk-ink",
        "font-bold text-base capitalize ",
        className,
      )}
      {...props}
    >
      {Icon && (
        <Icon
          key={iconKey}
          size={iconSize}
          aria-hidden={!ariaLabel}
          aria-label={ariaLabel}
          className={iconClassName}
        />
      )}
      {children}
    </button>
  );
}
