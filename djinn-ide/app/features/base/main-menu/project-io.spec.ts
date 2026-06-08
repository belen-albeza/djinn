import { it, expect, describe, beforeEach, spyOn } from "bun:test";
import { downloadProject, loadProject } from "./project-io";
import { useProjectStore } from "~/features/base/project.store";

beforeEach(() => {
  useProjectStore.getState().reset();
});

describe("downloadProject", () => {
  it("Downloads the current project as JSON", async () => {
    useProjectStore.getState().setProject({ title: "Piñata: 🪅!" });

    let saved: { filename: string; blob: Blob } | undefined;
    downloadProject((filename, blob) => {
      saved = { filename, blob };
    });

    expect(saved?.filename).toBe("pinata.json");
    expect(saved?.blob.type).toBe("application/json");
    expect(JSON.parse(await saved!.blob.text())).toEqual({
      title: "Piñata: 🪅!",
    });
  });
});

describe("loadProject", () => {
  it("Loads a chosen project into the store", async () => {
    const result = await loadProject(async () =>
      JSON.stringify({ title: "Loaded" }),
    );

    expect(result).toBe("success");
    expect(useProjectStore.getState().title).toBe("Loaded");
  });

  it("Reports an error for malformed project data", async () => {
    const result = await loadProject(async () =>
      JSON.stringify({ nope: true }),
    );
    expect(result).toBe("error");
  });

  it("Reports an error for invalid JSON", async () => {
    // silence console.error so test output is clean
    spyOn(console, "error").mockImplementation(() => {});
    const result = await loadProject(async () => "not json");
    expect(result).toBe("error");
  });

  it("Is cancelled when the user picks nothing", async () => {
    const result = await loadProject(async () => null);
    expect(result).toBe("cancelled");
  });
});
