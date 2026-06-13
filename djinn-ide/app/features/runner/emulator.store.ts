import { create } from "zustand";
import type { Emulator } from "djinn-dev-wasm";

interface EmulatorStore {
  emulator: Emulator | null;
  visible: boolean;
  setEmulator: (emulator: Emulator) => void;
  setVisible: (value: boolean) => void;
}

export const useEmulatorStore = create<EmulatorStore>()((set) => ({
  emulator: null,
  visible: false,
  setEmulator: (emulator) => set({ emulator }),
  setVisible: (value: boolean) => set({ visible: value }),
}));
