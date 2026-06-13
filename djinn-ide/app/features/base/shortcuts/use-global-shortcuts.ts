import { useEffect } from "react";

export interface Shortcut {
  key: string;
  mod?: boolean; // Cmd on macOS, Ctrl elsewhere
  shift?: boolean;
  alt?: boolean;
  preventDefault?: boolean;
  /** When provided and it returns false, the shortcut is skipped (key passes through). */
  enabled?: () => boolean;
  run: () => void;
}

function matches(event: KeyboardEvent, shortcut: Shortcut): boolean {
  const mod = event.metaKey || event.ctrlKey;

  return (
    event.key.toLowerCase() === shortcut.key.toLowerCase() &&
    mod === Boolean(shortcut.mod) &&
    event.shiftKey === Boolean(shortcut.shift) &&
    event.altKey === Boolean(shortcut.alt)
  );
}

export function useGlobalShortcuts(shortcuts: Shortcut[]) {
  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      for (const shortcut of shortcuts) {
        if (matches(event, shortcut)) {
          if (shortcut.enabled && !shortcut.enabled()) continue;
          if (shortcut.preventDefault) event.preventDefault();
          shortcut.run();
          return;
        }
      }
    }

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [shortcuts]);
}
