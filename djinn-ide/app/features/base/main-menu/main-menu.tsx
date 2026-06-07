import clsx from "clsx";

import { ListIcon, PlusIcon } from "@phosphor-icons/react";
import type { Icon as IconType } from "@phosphor-icons/react";

interface MenuItemProps {
  children: React.ReactNode;
  icon?: IconType;
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
}

function MenuItem({
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
      className={clsx("py-2 px-4 hover:bg-burst hover:text-ink", className)}
      {...other}
    >
      <button
        disabled={disabled}
        onClick={onClick}
        className="w-full text-left text-sm font-semibold flex direction-row gap-1 items-center disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {Icon && <Icon size={12} weight="bold" />}
        {children}
      </button>
    </li>
  );
}

export default function MainMenu() {
  return (
    <>
      <button
        popoverTarget="main-menu"
        className="anchor-[--main-menu-button] text-2xl font-bold uppercase text-burst flex items-center gap-2"
      >
        <ListIcon size={24} /> Djinn
      </button>

      <menu
        id="main-menu"
        popover="auto"
        aria-label="Main menu"
        className="anchor-below-[--main-menu-button] z-999 bg-ink text-sand-200 min-w-48 rounded-sharp"
      >
        <MenuItem icon={PlusIcon}>New Project</MenuItem>
        <MenuItem disabled>Open Project</MenuItem>
        <MenuItem disabled>Download</MenuItem>
      </menu>
    </>
  );
}
