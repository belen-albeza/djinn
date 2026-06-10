import { cn } from "~/utils/cn";
import type { Icon as IconType } from "@phosphor-icons/react";

interface MenuItemProps {
  children: React.ReactNode;
  icon?: IconType;
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
}

export function MenuItem({
  children,
  icon,
  onClick,
  disabled,
  className,
  ...other
}: MenuItemProps) {
  const Icon = icon;

  return (
    <li
      className={cn(!disabled && "hover:bg-burst hover:text-ink", className)}
      {...other}
    >
      <button
        disabled={disabled}
        onClick={onClick}
        popoverTarget="main-menu"
        popoverTargetAction="hide"
        className="py-2 px-4 w-full text-left text-sm font-semibold flex direction-row gap-1 items-center disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {Icon && <Icon size={12} weight="bold" />}
        {children}
      </button>
    </li>
  );
}
