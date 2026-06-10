import { create } from "zustand";

interface StatusBarStore {
  savedTick: number; // increments every time the user saves
  notifySaved: () => void;
}

export const useStatusBarStore = create<StatusBarStore>((set) => ({
  savedTick: 0,
  notifySaved: () => set((state) => ({ savedTick: state.savedTick + 1 })),
}));
