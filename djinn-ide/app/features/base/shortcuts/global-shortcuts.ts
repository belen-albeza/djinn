import { buildProject } from "./actions/build-project";
import { runProject } from "./actions/run-project";
import { saveProject } from "./actions/save-project";

import type { Shortcut } from "./use-global-shortcuts";

export const globalShortcuts: Shortcut[] = [
  {
    key: "s",
    mod: true,
    preventDefault: true,
    run: saveProject,
  },
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
