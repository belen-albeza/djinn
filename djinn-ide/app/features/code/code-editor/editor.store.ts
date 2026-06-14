import { create } from "zustand";

import type { CodeError } from "./error-markers";

type ReadCodeFn = () => string;
interface EditorStore {
  // Returns the current editor contents, or null when no editor is mounted.
  readCode: ReadCodeFn | null;
  errors: CodeError[];
  setReadCodeFn: (fn: ReadCodeFn | null) => void;
  setErrors: (errors: CodeError[]) => void;
}

export const useEditorStore = create<EditorStore>((set) => ({
  readCode: null,
  errors: [],
  setReadCodeFn: (fn) => set({ readCode: fn }),
  setErrors: (errors) => set({ errors }),
}));
