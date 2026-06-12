import { useEffect, useRef, useState } from "react";
import { PencilSimpleIcon } from "@phosphor-icons/react";
import clsx from "clsx";
import Button from "~/ui/button";

interface EditableInputProps {
  value: string;
  onChange: (value: string) => void;
  className?: string;
  editButtonClassName?: string;
  editIconAriaLabel?: string;
  type?: "text" | "number" | "email" | "password" | "tel" | "url";
  required?: boolean;
}

export default function EditableInput({
  value,
  onChange,
  required,
  className,
  editButtonClassName,
  editIconAriaLabel = "Edit",
  type = "text",
}: EditableInputProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [editValue, setEditValue] = useState(value);
  const inputRef = useRef<HTMLInputElement>(null);
  const skipBlurCommitRef = useRef(false);

  useEffect(() => {
    if (isEditing) {
      inputRef.current?.focus();
      inputRef.current?.select();
    }
  }, [isEditing]);

  function startEditing() {
    skipBlurCommitRef.current = false;
    setEditValue(value);
    setIsEditing(true);
  }

  function commitEdit() {
    if (required && !editValue) {
      cancelEdit();
      return;
    }

    onChange(editValue);
    setIsEditing(false);
  }

  function cancelEdit() {
    setEditValue(value);
    setIsEditing(false);
  }

  function handleBlur() {
    if (skipBlurCommitRef.current) {
      skipBlurCommitRef.current = false;
      return;
    }

    commitEdit();
  }

  function handleKeyDown(event: React.KeyboardEvent<HTMLInputElement>) {
    if (event.key === "Enter") {
      event.preventDefault();
      skipBlurCommitRef.current = true;
      commitEdit();
    } else if (event.key === "Escape") {
      event.preventDefault();
      cancelEdit();
    }
  }

  const sizerText = (isEditing ? editValue : value) || "\u00A0";

  return (
    <div className="flex items-center gap-2">
      <div
        className={clsx(
          "inline-grid items-center",
          // TODO: customize this border color
          isEditing && "border-b border-sand-200",
        )}
      >
        <span
          className={clsx(
            className,
            "col-start-1 row-start-1 whitespace-pre invisible pointer-events-none select-none",
          )}
          aria-hidden
        >
          {sizerText}
        </span>
        {isEditing ? (
          <input
            ref={inputRef}
            type={type}
            size={1}
            value={editValue}
            onChange={(event) => setEditValue(event.target.value)}
            onKeyDown={handleKeyDown}
            onBlur={handleBlur}
            required={required}
            className={clsx(
              className,
              "col-start-1 row-start-1 m-0 w-0 min-w-full max-w-full appearance-none border-0 bg-transparent p-0 shadow-none outline-none ring-0",
            )}
          />
        ) : (
          <span
            className={clsx(
              className,
              "col-start-1 row-start-1 whitespace-pre text-base",
            )}
          >
            {value}
          </span>
        )}
      </div>
      <Button
        variant="ghost"
        icon={PencilSimpleIcon}
        iconSize={16}
        aria-label={editIconAriaLabel}
        className={editButtonClassName}
        onClick={startEditing}
        disabled={isEditing}
      />
    </div>
  );
}
