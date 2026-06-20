import {
  build,
  Emulator,
  type Project,
  type DjinnErrorList,
} from "djinn-dev-wasm";

import type { Message } from "~/features/base/status-bar/status.store";

import {
  toProjectSnapshot,
  useProjectStore,
} from "~/features/base/project.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";
import { useEditorStore } from "~/features/code/code-editor/editor.store";
import { getCanBuild } from "./execution.rules";

export function buildProject() {
  if (!getCanBuild()) return;

  let errors: DjinnErrorList = [];
  let emulator: Emulator | null = null;

  const code = useEditorStore.getState().readCodeFn?.();
  if (!code) {
    console.error("readCodeFn is not set");
    return;
  }

  useProjectStore.getState().setSourceCode(code);

  try {
    emulator = build(toProjectSnapshot(useProjectStore.getState()) as Project);
  } catch (err: unknown) {
    errors = err as DjinnErrorList;
  }

  let messages: Message[] =
    errors.length > 0
      ? errors.map((e) => ({
          level: "error",
          message: e.message,
          position: e.position,
        }))
      : [{ level: "success", message: "Built without errors." }];

  useStatusBarStore.getState().setMessages(messages);
  useEditorStore.getState().setErrors(
    errors.map((e) => ({
      position: e.position,
      message: e.message,
    })),
  );
  return emulator;
}
