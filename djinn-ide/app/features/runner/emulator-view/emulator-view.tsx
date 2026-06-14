import { useRef, useEffect } from "react";
import { Emulator } from "djinn-dev-wasm";

import { Modal } from "~/ui/modal";
import { cn } from "~/utils/cn";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";
import { useEmulatorStore } from "../emulator.store";

// FIXME: maybe get these from somewhere else
const CANVAS_WIDTH = 160;
const CANVAS_HEIGHT = 144;

export default function EmulatorView({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const emulator = useEmulatorStore((state) => state.emulator);

  useEffect(() => {
    if (!canvasRef.current || !emulator) return;

    const canvas = canvasRef.current;
    const imageData = new ImageData(CANVAS_WIDTH, CANVAS_HEIGHT);
    let animationFrameId: number;

    const frame = () => {
      console.log(animationFrameId);
      const shallHalt = emulator.tick();

      const sharedBuffer = new Uint8Array(
        Emulator.memory.buffer,
        emulator.displayBuffer,
        CANVAS_WIDTH * CANVAS_HEIGHT * 4,
      );

      imageData.data.set(sharedBuffer);
      const ctx = canvas.getContext("2d");
      ctx?.putImageData(imageData, 0, 0);

      if (!shallHalt) {
        animationFrameId = requestAnimationFrame(frame);
      } else {
        useStatusBarStore
          .getState()
          .setMessage("Emulator halted successfully.");
      }
    };

    animationFrameId = requestAnimationFrame(frame);

    return () => {
      cancelAnimationFrame(animationFrameId);
    };
  }, [emulator, canvasRef]);

  return (
    <Modal open={open} onClose={onClose} className="p-12 bg-sand-100">
      <canvas
        ref={canvasRef}
        className={cn("pixelated bg-black")}
        style={{
          width: `${CANVAS_WIDTH * 4}px`,
          height: `${CANVAS_HEIGHT * 4}px`,
        }}
        width={CANVAS_WIDTH}
        height={CANVAS_HEIGHT}
      ></canvas>
    </Modal>
  );
}
