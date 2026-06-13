import { cn } from "~/utils/cn";
import { createPortal } from "react-dom";
import { useEffect, useRef } from "react";
import {
  XIcon,
  SkullIcon,
  WarningIcon,
  ChatDotsIcon,
} from "@phosphor-icons/react";

import Button from "~/ui/button";

type ModalVariant = "neutral" | "destructive" | "error";

const surfaceClasses: Record<ModalVariant, string> = {
  neutral:
    "bg-[linear-gradient(to_bottom,var(--color-ink)_6px,var(--color-sand-100)_6px)]",
  destructive:
    "bg-[linear-gradient(to_bottom,var(--color-error-700)_6px,var(--color-sand-100)_6px)]",
  error:
    "bg-[linear-gradient(to_bottom,var(--color-error-700)_6px,var(--color-sand-100)_6px)]",
};

function LevelIcon({ level }: { level: ModalVariant }) {
  switch (level) {
    case "neutral":
      return <ChatDotsIcon size={32} className="text-ink" />;
    case "destructive":
      return <WarningIcon size={32} className="text-error-700" />;
    case "error":
      return <SkullIcon size={32} className="text-error-700" />;
  }
}

interface ModalProps {
  children: React.ReactNode;
  open: boolean;
  onClose?: () => void;
  variant?: ModalVariant;
  className?: string;
  header?: React.ReactNode;
}

export function Modal({
  open,
  header,
  children,
  onClose,
  variant = "neutral",
  className,
}: ModalProps) {
  const ref = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    if (open) {
      ref.current?.showModal();
    } else {
      ref.current?.close();
    }
  }, [open]);

  // intercept Escape before the browser handles it
  const handleCancel = (e: React.KeyboardEvent<HTMLDialogElement>) => {
    e.preventDefault();
    onClose?.();
  };

  const dialog = (
    <dialog
      onCancel={handleCancel}
      ref={ref}
      className={cn(
        "m-auto max-h-[calc(100dvh-2rem)] max-w-[calc(100vw-2rem)] overflow-hidden",
        "px-6 pt-[22px] pb-6 ",
        "rounded-card border-ui border-ink shadow-grotesk-ink-pop backdrop:bg-ink/55",
        surfaceClasses[variant],
        className,
      )}
    >
      <div className="grid gap-4 grid-auto-rows">
        {header && (
          <header className="grid grid-cols-[auto_1fr] gap-2 items-center">
            <LevelIcon level={variant} />
            <div>{header}</div>
          </header>
        )}
        {children}
      </div>
      <Button
        variant="ghost"
        onClick={() => onClose?.()}
        icon={XIcon}
        iconWeight="bold"
        aria-label="Close"
        className="absolute top-4 right-4 hover:bg-sand-100 p-1 rounded-sharp"
      />
    </dialog>
  );
  return createPortal(dialog, document.body);
}
