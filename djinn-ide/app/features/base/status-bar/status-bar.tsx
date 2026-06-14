import { useEffect, useState } from "react";
import { FloppyDiskIcon, CheckCircleIcon } from "@phosphor-icons/react";

import { useStatusBarStore } from "./status.store";
import ErrorDisplay from "./error-display";
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

function MessageDisplay({ message }: { message: string }) {
  return (
    <span className="flex direction-row gap-1 items-center">
      <CheckCircleIcon size={20} /> {message}
    </span>
  );
}

export default function StatusBar() {
  const savedTick = useStatusBarStore((state) => state.savedTick);
  const [savedFlashing, setSavedFlashing] = useState(false);
  const errors = useStatusBarStore((state) => state.errors);
  const message = useStatusBarStore((state) => state.message);

  useEffect(() => {
    if (savedTick > 0) {
      setSavedFlashing(true);
    }
  }, [savedTick]);

  return (
    <footer className="px-3 py-2 border-t border-sand-200 bg-sand-100 align-center">
      <p className="text-small flex direction-row gap-2 justify-between items-center">
        {(errors.length > 0 || message === "") && (
          <ErrorDisplay errors={errors} />
        )}
        {message && errors.length === 0 && <MessageDisplay message={message} />}
        {savedFlashing && <SavedIndicator key={savedTick} />}
      </p>
    </footer>
  );
}
