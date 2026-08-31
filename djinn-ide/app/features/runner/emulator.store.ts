import { create } from "zustand";
import type { Emulator } from "djinn-dev-wasm";

interface EmulatorStore {
  emulator: Emulator | null;
  setEmulator: (emulator: Emulator) => void;
  visible: boolean;
  setVisible: (value: boolean) => void;
  stop: (() => void) | null;
  setStop: (stop: (() => void) | null) => void;
  dispose: () => void;
}

export const useEmulatorStore = create<EmulatorStore>()((set, get) => ({
  emulator: null,
  setEmulator: (emulator) => {
    get().emulator?.free(); // make sure any previous emulator is freed
    set({ emulator });
  },

  visible: false,
  setVisible: (value: boolean) => set({ visible: value }),

  stop: null,
  setStop: (stop) => set({ stop }),

  dispose: () => {
    const { emulator, stop } = get();
    if (!emulator) return;

    stop?.();
    emulator.free();
    set({ emulator: null, stop: null, visible: false });
  },
}));
