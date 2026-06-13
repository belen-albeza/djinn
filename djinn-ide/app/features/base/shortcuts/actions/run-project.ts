import type { DjinnErrorList } from "djinn-dev-wasm";

import { buildProject } from "./build-project";
import { useEmulatorStore } from "~/features/runner/emulator.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";

export function runProject() {
  let errors: string[] = [];

  try {
    const emulator = buildProject();
    if (!emulator) return;

    useEmulatorStore.getState().setEmulator(emulator);
    useEmulatorStore.getState().setVisible(true);

    emulator.run();
  } catch (err: unknown) {
    errors = (err as DjinnErrorList).map(
      (e) => `Error at Ln ${e.position[0]}, Col ${e.position[1]}: ${e.message}`,
    );
  }

  useStatusBarStore.getState().setErrors(errors);
}
