import slugify from "slugify";
import {
  toProjectSnapshot,
  useProjectStore,
  projectSchema,
} from "~/features/base/project.store";
import {
  saveFileToDisk,
  pickFileFromDisk,
  type SaveFile,
  type PickFile,
} from "./project-io-ports";

export function downloadProject(save: SaveFile = saveFileToDisk) {
  const project = toProjectSnapshot(useProjectStore.getState());
  // build a JSON blob to download
  const json = JSON.stringify(project);
  const blob = new Blob([json], { type: "application/json" });

  // Build a filename from the project title, discarding special characters.
  // If the title is empty, use "djinn" as the default.
  const filename = `${slugify(project.title, { lower: true, strict: true }) || "djinn"}.json`;

  save(filename, blob);
}

export async function loadProject(
  pick: PickFile = pickFileFromDisk,
): Promise<"success" | "cancelled" | "error"> {
  try {
    const data = await pick();
    if (!data) {
      return "cancelled";
    }
    const rawJson = JSON.parse(data);
    const result = projectSchema.safeParse(rawJson);
    if (!result.success) {
      return "error";
    }

    useProjectStore.getState().setProject(result.data);
    return "success";
  } catch (error) {
    console.error("Invalid file", error);
    return "error";
  }
}
