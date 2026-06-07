import { create } from "zustand";

export type ProjectSnapshot = {
  title: string;
};

export const defaultProject: ProjectSnapshot = {
  title: "",
};

interface ProjectStore extends ProjectSnapshot {
  setProject: (project: ProjectSnapshot) => void;
  setTitle: (title: string) => void;
}

export const useProjectStore = create<ProjectStore>((set) => ({
  ...defaultProject,
  setProject: (project) => set({ ...defaultProject, ...project }),
  setTitle: (title) => set({ title }),
}));
