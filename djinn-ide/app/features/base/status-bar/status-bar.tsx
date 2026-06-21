import { useEffect, useState } from "react";
import { useLocation } from "react-router";
import { FloppyDiskIcon } from "@phosphor-icons/react";
import type { Location } from "djinn-dev-wasm";

import { useStatusBarStore } from "./status.store";
import { useEditorStore } from "~/features/code/code-editor/editor.store";
import MessageDisplay from "./message-display";
import { cn } from "~/utils/cn";

function SavedIndicator({ ...other }: React.SVGProps<SVGSVGElement>) {
  return (
    <FloppyDiskIcon
      size={20}
      weight="duotone"
      className={cn("opacity-0 animate-saved-flash")}
      {...other}
    />
  );
}

function Cursor({ position }: { position: Location }) {
  return (
    <span className="text-small text-ink tabular-nums">
      Ln {position.line}, Col {position.column}
    </span>
  );
}

export default function StatusBar() {
  const savedTick = useStatusBarStore((state) => state.savedTick);
  const [savedFlashing, setSavedFlashing] = useState(false);
  const messages = useStatusBarStore((state) => state.messages);

  const pathname = useLocation().pathname;
  const shallShowCursor = pathname === "/" || pathname === "/code"; // TODO:
  const cursor = useEditorStore((state) => state.cursor);

  useEffect(() => {
    if (savedTick > 0) {
      setSavedFlashing(true);
    }
  }, [savedTick]);

  return (
    <footer className="px-3 py-2 border-t border-sand-200 bg-sand-100 align-center">
      <p className="text-small flex direction-row gap-2 justify-between items-center">
        {<MessageDisplay messages={messages} />}
        <span className="flex direction-row gap-2">
          {savedFlashing && (
            <SavedIndicator
              key={savedTick}
              onAnimationEnd={(e: React.AnimationEvent<SVGSVGElement>) => {
                if (e.animationName === "saved-flash") {
                  setSavedFlashing(false);
                }
              }}
            />
          )}
          {shallShowCursor && <Cursor position={cursor} />}
        </span>
      </p>
    </footer>
  );
}
