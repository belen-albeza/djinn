import { Modal } from "~/ui/modal";
import { cn } from "~/utils/cn";

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
  return (
    <Modal open={open} onClose={onClose} className="p-12 bg-sand-100">
      <canvas
        className={cn(
          "pixelated bg-black",
          `w-[${CANVAS_WIDTH * 4}px] h-[${CANVAS_HEIGHT * 4}px]`,
        )}
        width={CANVAS_WIDTH}
        height={CANVAS_HEIGHT}
      ></canvas>
    </Modal>
  );
}
