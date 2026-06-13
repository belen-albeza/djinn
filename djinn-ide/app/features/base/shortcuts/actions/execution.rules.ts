import { useLocation } from "react-router";

import { useProjectStore } from "~/features/base/project.store";
import { useEmulatorStore } from "~/features/runner/emulator.store";

/** Route on which a project cannot be built or run. */
const NEW_ROUTE = "/new";

export interface ExecutionContext {
  emulatorVisible: boolean;
  projectTitle: string;
  pathname: string;
}

// Single source of truth for whether the project is in a state that allows
// building/executing.
const isReadyToBuild = (ctx: ExecutionContext): boolean =>
  !ctx.emulatorVisible &&
  ctx.projectTitle.trim() !== "" &&
  ctx.pathname !== NEW_ROUTE;

// TODO: eventually `canRun` will be more complex, so let's keep these separate
export const canBuild = isReadyToBuild;
export const canRun = isReadyToBuild;
export const canSave = isReadyToBuild;

// Reads the current ExecutionContext outside of React (action guards, shortcuts)
const readContext = (): ExecutionContext => ({
  emulatorVisible: useEmulatorStore.getState().visible,
  projectTitle: useProjectStore.getState().title,
  // react-router uses the History API, so this stays current on SPA navigation.
  pathname: window.location.pathname,
});

export const getCanBuild = (): boolean => canBuild(readContext());
export const getCanRun = (): boolean => canRun(readContext());
export const getCanSave = (): boolean => canSave(readContext());

// Reactive context (for React components)
const useExecutionContext = (): ExecutionContext => ({
  emulatorVisible: useEmulatorStore((s) => s.visible),
  projectTitle: useProjectStore((s) => s.title),
  pathname: useLocation().pathname,
});

export const useCanBuild = (): boolean => canBuild(useExecutionContext());
export const useCanRun = (): boolean => canRun(useExecutionContext());
export const useCanSave = (): boolean => canSave(useExecutionContext());
