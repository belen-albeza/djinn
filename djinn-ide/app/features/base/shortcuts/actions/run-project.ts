import type { DjinnError } from "djinn-dev-wasm";

import { buildProject } from "./build-project";
import { useEmulatorStore } from "~/features/runner/emulator.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";
import { getCanRun } from "./execution.rules";

export function runProject() {
  if (!getCanRun()) return;

  let error: string | undefined = undefined;

  try {
    const emulator = buildProject();
    if (!emulator) return;

    useEmulatorStore.getState().setEmulator(emulator);
    useEmulatorStore.getState().setVisible(true);

    // TODO: handle halting logic
    const shallHalt = emulator.step();
    if (shallHalt) {
      console.log("Halted.");
    }
  } catch (err: unknown) {
    let e = err as DjinnError;
    error = `Error at Ln ${e.position[0]}, Col ${e.position[1]}: ${e.message}`;
  }

  if (error) {
    useStatusBarStore.getState().setErrors([error]);
  }
}
