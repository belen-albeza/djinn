import slugify from "slugify";
import {
  toProjectSnapshot,
  useProjectStore,
} from "~/features/base/project.store";

export function downloadProject() {
  const project = toProjectSnapshot(useProjectStore.getState());
  // build a JSON blob to download
  const json = JSON.stringify(project);
  const blob = new Blob([json], { type: "application/json" });
  const url = URL.createObjectURL(blob);

  // build a filename from the project title, discarding special characters
  const { title } = project;
  const filename = `${slugify(title, { lower: true })}.json`;

  // trigger download by clicking an orphan, temporary link
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();

  // clean up the temporary URL
  URL.revokeObjectURL(url);
}

function loadFile(): Promise<string | null> {
  return new Promise((resolve) => {
    const fileInput = document.createElement("input");
    fileInput.type = "file";
    fileInput.accept = "application/json";
    fileInput.onchange = (event) => {
      const file = (event.target as HTMLInputElement).files?.[0];
      if (!file) {
        resolve(null);
        return;
      }

      const reader = new FileReader();
      reader.onload = (loadEvent) => {
        const result = loadEvent.target?.result;
        resolve(typeof result === "string" ? result : null);
      };
      reader.onerror = () => resolve(null);
      reader.readAsText(file);
    };
    fileInput.click();
  });
}

export async function loadProject(): Promise<
  "success" | "cancelled" | "error"
> {
  try {
    const data = await loadFile();
    if (!data) {
      return "cancelled";
    }
    const project = JSON.parse(data);
    // TODO: validate the project structure
    useProjectStore.getState().setProject(project);
    return "success";
  } catch (error) {
    console.error("Invalid file", error);
    return "error";
  }
}
