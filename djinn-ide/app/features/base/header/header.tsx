import EditableInput from "~/ui/editable-input";
import { useProjectStore } from "~/features/base/project.store";
import MainMenu from "../main-menu";

export default function Header() {
  const title = useProjectStore((state) => state.title);
  const setTitle = useProjectStore((state) => state.setTitle);

  return (
    <header className="p-4 bg-ink text-paper grid grid-cols-[auto_1fr] gap-8 items-center">
      <MainMenu />
      <EditableInput
        value={title}
        onChange={setTitle}
        className="font-mono text-xl font-bold"
        editButtonClassName="text-sand-600 hover:text-burst"
        editIconAriaLabel="Edit title"
      />
    </header>
  );
}
