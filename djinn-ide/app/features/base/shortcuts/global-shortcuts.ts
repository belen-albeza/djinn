import { buildProject } from "./actions/build-project";

import type { Shortcut } from "./use-global-shortcuts";

export const globalShortcuts: Shortcut[] = [
  {
    key: "b",
    mod: true,
    preventDefault: true,
    run: buildProject,
  },
];
