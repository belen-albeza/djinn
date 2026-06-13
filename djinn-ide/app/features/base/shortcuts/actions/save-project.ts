import { useProjectStore } from "~/features/base/project.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";
import { useEditorStore } from "~/features/code/code-editor/editor.store";

export function saveProject() {
  const readCode = useEditorStore.getState().readCode;
  if (!readCode) return;

  useProjectStore.getState().setSourceCode(readCode());
  useStatusBarStore.getState().notifySaved();
}
