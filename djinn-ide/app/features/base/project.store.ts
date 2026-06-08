import { create } from "zustand";
import { persist } from "zustand/middleware";

export type ProjectSnapshot = {
  title: string;
};

export const defaultProject: ProjectSnapshot = {
  title: "",
};

interface ProjectStore extends ProjectSnapshot {
  setProject: (project: ProjectSnapshot) => void;
  setTitle: (title: string) => void;
  reset: () => void;
}

export function toProjectSnapshot(state: ProjectStore): ProjectSnapshot {
  return { title: state.title };
}

export const useProjectStore = create<ProjectStore>()(
  persist(
    (set) => ({
      ...defaultProject,
      reset: () => set(defaultProject),
      setProject: (project) => set({ ...defaultProject, ...project }),
      setTitle: (title) => set({ title }),
    }),
    {
      name: "project",
      partialize: toProjectSnapshot,
    },
  ),
);
