import { CheckCircleIcon } from "@phosphor-icons/react";
import { WarningIcon } from "@phosphor-icons/react";
import type { Message } from "../status.store";

function LevelIcon({ level }: { level: "error" | "success" }) {
  switch (level) {
    case "error":
      return (
        <WarningIcon
          size={20}
          weight="duotone"
          className="text-error-700"
          aria-label="Error"
          role="img"
        />
      );
    case "success":
      return (
        <CheckCircleIcon
          weight="duotone"
          className="text-success-700"
          size={20}
          aria-label="Success"
          role="img"
        />
      );
  }
}

function MessageBar({ messages }: { messages: Message[] }) {
  return (
    <span className="flex direction-row gap-1 items-center">
      {messages.length > 1 && (
        <span className="text-error-700 font-semibold">
          {" "}
          {messages.length}{" "}
        </span>
      )}
      <LevelIcon level={messages[0].type} />{" "}
      <span className="text-ink">{messages[0].message}</span>
    </span>
  );
}

export default function MessageDisplay({ messages }: { messages: Message[] }) {
  return messages.length > 0 ? (
    <MessageBar messages={messages} />
  ) : (
    <MessageBar messages={[{ type: "success", message: "Ready." }]} />
  );
}
