import { create } from "zustand";
import { persist } from "zustand/middleware";
import { z } from "zod";

export const projectSchema = z.object({
  title: z.string(),
});

export type ProjectSnapshot = z.infer<typeof projectSchema>;

export const defaultProject: ProjectSnapshot = {
  title: "",
};

interface ProjectStore extends ProjectSnapshot {
  setProject: (project: ProjectSnapshot) => void;
  setTitle: (title: string) => void;
  reset: () => void;
}

export function toProjectSnapshot(state: ProjectStore): ProjectSnapshot {
  return projectSchema.parse(state);
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
