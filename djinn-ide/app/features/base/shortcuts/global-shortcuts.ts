import { buildProject } from "./actions/build-project";
import { runProject } from "./actions/run-project";
import { saveProject } from "./actions/save-project";
import { getCanBuild, getCanRun, getCanSave } from "./actions/execution.rules";

import type { Shortcut } from "./use-global-shortcuts";

export const globalShortcuts: Shortcut[] = [
  {
    key: "s",
    mod: true,
    preventDefault: true,
    run: saveProject,
    enabled: getCanSave,
  },
  {
    key: "b",
    mod: true,
    preventDefault: true,
    enabled: getCanBuild,
    run: buildProject,
  },
  {
    key: "Enter",
    mod: true,
    preventDefault: true,
    enabled: getCanRun,
    run: runProject,
  },
];
