import { create } from "zustand";
import type { Location } from "djinn-dev-wasm";

import type { CodeError } from "./error-markers";

type ReadCodeFn = () => string;
interface EditorStore {
  // Returns the current editor contents, or null when no editor is mounted.
  readCodeFn: ReadCodeFn | null;
  errors: CodeError[];
  cursor: Location; // 1-based, as compilers report
  setReadCodeFn: (fn: ReadCodeFn | null) => void;
  setErrors: (errors: CodeError[]) => void;
  setCursor: (cursor: Location) => void;
}

export const useEditorStore = create<EditorStore>((set) => ({
  readCodeFn: null,
  errors: [],
  cursor: { line: 1, column: 1 },
  setReadCodeFn: (fn) => set({ readCodeFn: fn }),
  setErrors: (errors) => set({ errors }),
  setCursor: (cursor) => set({ cursor }),
}));
