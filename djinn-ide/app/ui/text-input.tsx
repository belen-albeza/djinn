import clsx from "clsx";
import { useId, type InputHTMLAttributes } from "react";

interface TextInputProps
  extends Omit<InputHTMLAttributes<HTMLInputElement>, "id"> {
  label: string;
  id?: string;
  error?: string;
  className?: string;
  inputClassName?: string;
}

export default function TextInput({
  label,
  id: idProp,
  error,
  className,
  inputClassName,
  name,
  type = "text",
  ...inputProps
}: TextInputProps) {
  const generatedId = useId();
  const id = idProp ?? generatedId;
  const errorId = error ? `${id}-error` : undefined;

  return (
    <div className={className}>
      <label
        htmlFor={id}
        className="mb-2 block text-xs font-medium tracking-wide uppercase text-sand-600"
      >
        {label}
      </label>
      <input
        id={id}
        name={name}
        type={type}
        aria-invalid={error ? true : undefined}
        aria-describedby={errorId}
        className={clsx(
          "w-full rounded-sharp border-ui border-ink bg-paper px-4 py-4",
          "text-2xl font-medium text-ink outline-none",
          "placeholder:font-normal placeholder:text-sand-400",
          "transition-shadow duration-150",
          "focus:shadow-grotesk",
          error && "border-error-700 focus:shadow-grotesk-error",
          inputClassName,
        )}
        {...inputProps}
      />
      {error ? (
        <p id={errorId} role="alert" className="mt-2 text-sm text-error-700">
          {error}
        </p>
      ) : null}
    </div>
  );
}
