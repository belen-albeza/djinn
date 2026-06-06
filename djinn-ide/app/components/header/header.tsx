import { useState } from "react";
import { LegoIcon } from "@phosphor-icons/react";
import EditableInput from "~/ui/editable-input";
import { randomGameTitle } from "~/utils/random";

interface HeaderProps {
  defaultTitle: string;
}

export default function Header({ defaultTitle }: HeaderProps) {
  const [title, setTitle] = useState(defaultTitle);

  return (
    <header className="p-4 bg-ink text-paper grid grid-cols-[auto_1fr] gap-8 items-center">
      <h1 className="text-2xl font-bold uppercase text-burst flex items-center gap-2">
        <LegoIcon size={32} /> Djinn
      </h1>
      <EditableInput
        value={title}
        onChange={setTitle}
        className="font-mono text-xl font-bold"
        editButtonClassName="text-sand-600 hover:text-sand-100 border-sand-200"
        editIconAriaLabel="Edit title"
      />
    </header>
  );
}
