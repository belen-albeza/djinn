import EditableInput from "~/ui/editable-input";
import { useProjectStore } from "~/features/base/project.store";

import MainMenu from "./main-menu";
import NavBar from "./nav-bar";
import ActionsBar from "./actions-bar";

export default function Header() {
  const title = useProjectStore((state) => state.title);
  const setTitle = useProjectStore((state) => state.setTitle);

  return (
    <header className="px-2 py-2 bg-ink text-paper grid grid-cols-[auto_auto_1fr_auto] gap-4 items-center">
      <MainMenu />
      <NavBar className="px-4 border-l-2 border-r-2 border-sand-200-30" />
      {title ? (
        <EditableInput
          value={title}
          onChange={setTitle}
          className="text-m uppercase font-semibold text-sand-400"
          editButtonClassName="text-sand-400 hover:text-burst"
          editIconAriaLabel="Edit title"
          required
        />
      ) : (
        <p className="font-mono text-m uppercase font-semibold text-sand-400 before:content-['•'] before:text-burst flex direction-row gap-1">
          New Project
        </p>
      )}
      <ActionsBar />
    </header>
  );
}
