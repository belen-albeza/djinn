import { useRef, useEffect } from "react";
import { Emulator, type DjinnError } from "djinn-dev-wasm";

import { Modal } from "~/ui/modal";
import { cn } from "~/utils/cn";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";
import { useEmulatorStore } from "../emulator.store";
import { useEditorStore } from "~/features/code/code-editor/editor.store";

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
    if (!canvasRef.current || !emulator || !open) return;

    const canvas = canvasRef.current;
    const imageData = new ImageData(CANVAS_WIDTH, CANVAS_HEIGHT);
    let animationFrameId = 0;
    let stopped = false;

    let error: DjinnError | null = null;

    const stop = () => {
      stopped = true;
      cancelAnimationFrame(animationFrameId);
    };

    const frame = () => {
      if (stopped) return;

      let shallHalt = false;

      try {
        shallHalt = emulator.tick();
      } catch (err: unknown) {
        error = err as DjinnError;
        shallHalt = true;
      }

      for (const message of emulator.stdout) {
        console.log(message);
      }

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
        return;
      }

      // handle errors
      if (error) {
        console.error(`Runtime error at Ln ${error.position.line}, Col ${error.position.column}: ${error.message}`);
        useStatusBarStore.getState().setMessages([
          {
            level: "error",
            position: error.position,
            message: error.message,
          },
        ]);
        useEditorStore.getState().setErrors([
          {
            position: error.position,
            message: error.message,
          },
        ]);
      } else {
        useStatusBarStore.getState().setMessages([]);
      }

      // program halted succesfully -> close the modal
      onClose();
    };

    useEmulatorStore.getState().setStop(stop);
    animationFrameId = requestAnimationFrame(frame);

    return () => {
      stop();
      cancelAnimationFrame(animationFrameId);
    };
  }, [emulator, canvasRef, open, onClose]);

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
