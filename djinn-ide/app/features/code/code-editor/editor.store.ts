import { create } from "zustand";

type ReadCodeFn = () => string;

interface EditorStore {
  /** Returns the current editor contents, or null when no editor is mounted. */
  readCode: ReadCodeFn | null;
  setReadCodeFn: (fn: ReadCodeFn | null) => void;
}

export const useEditorStore = create<EditorStore>((set) => ({
  readCode: null,
  setReadCodeFn: (fn) => set({ readCode: fn }),
}));
