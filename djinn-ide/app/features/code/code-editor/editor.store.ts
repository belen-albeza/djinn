import { create } from "zustand";

import type { CodeError } from "./error-markers";

type ReadCodeFn = () => string;
interface EditorStore {
  // Returns the current editor contents, or null when no editor is mounted.
  readCodeFn: ReadCodeFn | null;
  errors: CodeError[];
  setReadCodeFn: (fn: ReadCodeFn | null) => void;
  setErrors: (errors: CodeError[]) => void;
}

export const useEditorStore = create<EditorStore>((set) => ({
  readCodeFn: null,
  errors: [],
  setReadCodeFn: (fn) => set({ readCodeFn: fn }),
  setErrors: (errors) => set({ errors }),
}));
