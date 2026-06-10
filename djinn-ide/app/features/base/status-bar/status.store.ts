import { create } from "zustand";

interface StatusBarStore {
  savedTick: number; // increments every time the user saves
  errors: string[];
  notifySaved: () => void;
  setErrors: (errors: string[]) => void;
}

export const useStatusBarStore = create<StatusBarStore>((set) => ({
  savedTick: 0,
  errors: [],
  notifySaved: () => set((state) => ({ savedTick: state.savedTick + 1 })),
  setErrors: (errors: string[]) => set({ errors }),
}));
