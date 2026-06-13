import { NavLink } from "react-router";
import {
  ScrollIcon,
  GhostIcon,
  TextAaIcon,
  MapTrifoldIcon,
  WaveformIcon,
  MusicNoteIcon,
  type Icon as IconType,
} from "@phosphor-icons/react";

import { cn } from "~/utils/cn";

interface NavBarItemProps {
  title: string;
  to: string;
  icon?: IconType;
}

function NavBarItem({ title, to, icon }: NavBarItemProps) {
  const Icon = icon;

  return (
    <li className="h-full ">
      <NavLink
        to={to}
        className={({ isActive }) =>
          cn(
            "relative h-full place-self-center text-base",
            "flex items-center font-sans font-bold gap-1",
            "hover:text-sand-400 transition-colors duration-150",
            isActive &&
              "text-sand-100 after:absolute after:inset-x-0 after:top-full after:h-1 after:bg-burst after:content-['']",
          )
        }
      >
        {Icon && <Icon size={24} />}
        {title}
      </NavLink>
    </li>
  );
}

export default function NavBar({ className }: { className?: string }) {
  return (
    <nav className={cn("h-full", className)}>
      <menu className="flex direction-row gap-4 text-label h-full">
        <NavBarItem title="Code" to="/" icon={ScrollIcon} />
        <NavBarItem title="Sprites" to="/sprites/" icon={GhostIcon} />
        <NavBarItem title="Map" to="/map/" icon={MapTrifoldIcon} />
        <NavBarItem title="Font" to="/font/" icon={TextAaIcon} />
        <NavBarItem title="Sfx" to="/sfx/" icon={WaveformIcon} />
        <NavBarItem title="Music" to="/music/" icon={MusicNoteIcon} />
      </menu>
    </nav>
  );
}
