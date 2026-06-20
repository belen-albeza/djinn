import type { Location } from "djinn-dev-wasm";
import { create } from "zustand";

export interface Message {
  level: "error" | "success";
  message: string;
  position?: Location;
}

type StatusBarSnapshot = {
  savedTick: number;
  messages: Message[];
};

interface StatusBarStore extends StatusBarSnapshot {
  notifySaved: () => void;
  setMessages: (messages: Message[]) => void;
  reset: () => void;
}

const defaultStatusBar: StatusBarSnapshot = {
  savedTick: 0,
  messages: [],
};

export const useStatusBarStore = create<StatusBarStore>((set) => ({
  ...defaultStatusBar,
  notifySaved: () => set((state) => ({ savedTick: state.savedTick + 1 })),
  setMessages: (messages: Message[]) => set({ messages }),
  reset: () => set(defaultStatusBar),
}));
