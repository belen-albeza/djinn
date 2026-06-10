import { CheckCircleIcon } from "@phosphor-icons/react";
import { WarningIcon } from "@phosphor-icons/react";

function ErrorBar({ errors }: { errors: string[] }) {
  return (
    <span className="flex direction-row gap-1 items-center">
      <span className="text-error-700 font-semibold"> {errors.length} </span>
      <WarningIcon size={20} weight="duotone" className="text-error-700" />{" "}
      <span className="text-ink">{errors[0]}</span>
    </span>
  );
}

export default function ErrorDisplay({ errors }: { errors: string[] }) {
  return errors.length > 0 ? (
    <ErrorBar errors={errors} />
  ) : (
    <span className="flex direction-row gap-1 items-center">
      <CheckCircleIcon size={20} /> No errors
    </span>
  );
}
