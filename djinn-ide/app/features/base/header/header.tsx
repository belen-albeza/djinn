import EditableInput from "~/ui/editable-input";
import { useProjectStore } from "~/features/base/project.store";
import MainMenu from "../main-menu";

export default function Header() {
  const title = useProjectStore((state) => state.title);
  const setTitle = useProjectStore((state) => state.setTitle);

  return (
    <header className="p-4 bg-ink text-paper grid grid-cols-[auto_1fr] gap-8 items-center">
      <MainMenu />
      {title ? (
        <EditableInput
          value={title}
          onChange={setTitle}
          className="font-mono text-m uppercase font-semibold text-sand-400"
          editButtonClassName="text-sand-400 hover:text-burst"
          editIconAriaLabel="Edit title"
          required
        />
      ) : (
        <p className="font-mono text-m uppercase font-semibold text-sand-400 before:content-['•'] before:text-burst flex direction-row gap-1">
          New Project
        </p>
      )}
    </header>
  );
}
