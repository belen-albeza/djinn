import { create } from "zustand";

type StatusBarSnapshot = {
  savedTick: number;
  errors: string[];
};

interface StatusBarStore extends StatusBarSnapshot {
  notifySaved: () => void;
  setErrors: (errors: string[]) => void;
  reset: () => void;
}

const defaultStatusBar: StatusBarSnapshot = {
  savedTick: 0,
  errors: [],
};

export const useStatusBarStore = create<StatusBarStore>((set) => ({
  ...defaultStatusBar,
  notifySaved: () => set((state) => ({ savedTick: state.savedTick + 1 })),
  setErrors: (errors: string[]) => set({ errors }),
  reset: () => set(defaultStatusBar),
}));
