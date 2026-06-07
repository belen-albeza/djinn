import clsx from "clsx";
import { type Icon as IconType } from "@phosphor-icons/react";

import type { ButtonHTMLAttributes } from "react";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "ghost";
  disabled?: boolean;
  onClick?: () => void;
  icon?: IconType;
  "aria-label"?: string;
}

export default function Button({
  children,
  icon,
  variant = "ghost",
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
