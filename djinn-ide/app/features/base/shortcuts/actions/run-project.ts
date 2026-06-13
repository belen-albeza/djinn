import type { DjinnError, DjinnErrorList } from "djinn-dev-wasm";

import { buildProject } from "./build-project";
import { useEmulatorStore } from "~/features/runner/emulator.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";
import { getCanRun } from "./execution.rules";

export function runProject() {
  if (!getCanRun()) return;

  const emulator = buildProject();
  if (!emulator) return;

  useEmulatorStore.getState().setEmulator(emulator);
  useEmulatorStore.getState().setVisible(true);
}
