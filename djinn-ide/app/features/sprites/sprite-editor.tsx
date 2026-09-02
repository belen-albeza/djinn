import { cn } from "~/utils/cn";

import Button from "~/ui/button";
import { PaintBrushIcon } from "@phosphor-icons/react";

export function Toolbar() {
  return (
    <section className="h-min">
      <Button
        icon={PaintBrushIcon}
        iconWeight="bold"
        iconSize={24}
        variant="ghost"
      />
    </section>
  );
}

export function DrawingArea() {
  return <section className="bg-black w-[512px] h-[512px]"></section>;
}

export function Palette() {
  return (
    <section className="bg-black text-white w-[256px] h-[256px]">
      Palette
    </section>
  );
}

export function SpriteEditorStatusBar({ className }: { className?: string }) {
  return (
    <section
      className={cn(
        "bg-sand-100 text-sand-800 border-sand-200 border-t p-2",
        className,
      )}
    >
      Status Bar
    </section>
  );
}

export default function SpriteEditor() {
  return (
    <article
      className={
        "bg-paper grid grid-rows-[auto_auto] h-min gap-2 justify-self-center mt-8 shadow-grotesk-ink rounded-sharp"
      }
    >
      <section className="grid grid-cols-[auto_auto_auto] gap-2 max-w-fit">
        <Toolbar />
        <DrawingArea />
        <Palette />
      </section>
      <SpriteEditorStatusBar className="" />
    </article>
  );
}
