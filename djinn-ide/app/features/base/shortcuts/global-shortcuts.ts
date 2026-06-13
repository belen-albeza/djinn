import { buildProject } from "./actions/build-project";
import { runProject } from "./actions/run-project";

import type { Shortcut } from "./use-global-shortcuts";

export const globalShortcuts: Shortcut[] = [
  {
    key: "b",
    mod: true,
    preventDefault: true,
    run: buildProject,
  },
  {
    key: "Enter",
    mod: true,
    preventDefault: true,
    run: runProject,
  },
];
