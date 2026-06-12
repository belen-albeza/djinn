import { build, type BuildErrorList } from "djinn-dev-wasm";

import { useProjectStore } from "~/features/base/project.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";

export function buildProject() {
  let errors: string[] = [];

  try {
    build(useProjectStore.getState().title);
  } catch (err: unknown) {
    errors = (err as BuildErrorList).map(
      (e) => `Error at Ln ${e.position[0]}, Col ${e.position[1]}: ${e.message}`,
    );
  }

  useStatusBarStore.getState().setErrors(errors);
}
