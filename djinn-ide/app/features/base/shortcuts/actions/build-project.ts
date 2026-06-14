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
import { getCanBuild } from "./execution.rules";

export function buildProject() {
  if (!getCanBuild()) return;

  let errors: string[] = [];
  let emulator: Emulator | null = null;

  try {
    emulator = build(toProjectSnapshot(useProjectStore.getState()) as Project);
  } catch (err: unknown) {
    errors = (err as DjinnErrorList).map(
      (e) => `Error at Ln ${e.position[0]}, Col ${e.position[1]}: ${e.message}`,
    );
  }

  let messages: Message[] =
    errors.length > 0
      ? errors.map((e) => ({ type: "error", message: e }))
      : [{ type: "success", message: "Built without errors." }];

  useStatusBarStore.getState().setMessages(messages);
  return emulator;
}
