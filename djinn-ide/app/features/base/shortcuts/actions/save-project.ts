import { useProjectStore } from "~/features/base/project.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";
import { useEditorStore } from "~/features/code/code-editor/editor.store";
import { getCanSave } from "./execution.rules";

export function saveProject() {
  if (!getCanSave()) return;

  const readCode = useEditorStore.getState().readCodeFn;
  if (!readCode) return;

  useProjectStore.getState().setSourceCode(readCode());
  useStatusBarStore.getState().notifySaved();
}
