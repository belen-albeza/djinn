import {
  build,
  Emulator,
  type Project,
  type DjinnErrorList,
} from "djinn-dev-wasm";

import {
  toProjectSnapshot,
  useProjectStore,
} from "~/features/base/project.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";

export function buildProject() {
  let errors: string[] = [];
  let emulator: Emulator | null = null;

  try {
    emulator = build(toProjectSnapshot(useProjectStore.getState()) as Project);
  } catch (err: unknown) {
    errors = (err as DjinnErrorList).map(
      (e) => `Error at Ln ${e.position[0]}, Col ${e.position[1]}: ${e.message}`,
    );
  }

  useStatusBarStore.getState().setErrors(errors);
  return emulator;
}
