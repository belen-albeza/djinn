import { useEffect, useState } from "react";
import { FloppyDiskIcon } from "@phosphor-icons/react";

import { useStatusBarStore } from "./status.store";
import { cn } from "~/utils/cn";

function SavedIndicator() {
  return (
    <FloppyDiskIcon
      size={20}
      weight="duotone"
      className={cn("opacity-0 animate-saved-flash")}
    />
  );
}

export default function StatusBar() {
  const savedTick = useStatusBarStore((state) => state.savedTick);
  const [savedFlashing, setSavedFlashing] = useState(false);

  useEffect(() => {
    if (savedTick > 0) {
      setSavedFlashing(true);
    }
  }, [savedTick]);

  return (
    <footer className="px-3 py-2 border-t border-sand-200 bg-sand-100 align-center">
      <p className="text-small flex direction-row gap-2 justify-between items-center">
        <span>No errors</span>
        {savedFlashing && <SavedIndicator key={savedTick} />}
      </p>
    </footer>
  );
}
