import { it, expect, describe, mock, afterAll, beforeEach } from "bun:test";
import { render, waitFor } from "@testing-library/react";
import { mockAsmLezer, restoreAsmLezer } from "#test/asm-lezer";
import { mockDjinnDevWasm, restoreDjinnDevWasm } from "#test/djinn-dev-wasm";
import {
  useProjectStore,
  type ProjectSnapshot,
} from "~/features/base/project.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";

await mockAsmLezer();

afterAll(() => {
  restoreDjinnDevWasm();
  restoreAsmLezer();
});

const { default: CodeEditor } = await import("./code-editor");

function anyProjectWithSourceCode(sourceCode: string): ProjectSnapshot {
  return { title: "Lorem", sourceCode };
}

function pressBuildShortcut(element: HTMLElement) {
  // Happy DOM reports an X11 platform, so CodeMirror maps Mod-b to Ctrl-b.
  const event = new KeyboardEvent("keydown", {
    key: "b",
    code: "b",
    keyCode: 66,
    which: 66,
    ctrlKey: true,
    bubbles: true,
    cancelable: true,
  });
  Object.defineProperty(event, "synthetic", { value: true });
  element.dispatchEvent(event);
}

describe("CodeEditor", () => {
  beforeEach(() => {
    useStatusBarStore.getState().reset();
  });

  it("Builds a project", async () => {
    const build = mock((_title: string) => ({ title: "Lorem" }));
    await mockDjinnDevWasm({ build });

    useProjectStore.setState(anyProjectWithSourceCode("; Hello, world!"));
    const { container } = render(<CodeEditor />);

    const editor = await waitFor(() => {
      const content = container.querySelector(".cm-content");
      if (!content) throw new Error("CodeMirror editor not mounted");
      return content as HTMLElement;
    });

    pressBuildShortcut(editor);

    await waitFor(() => {
      expect(build).toHaveBeenCalledWith("Lorem");
    });
    expect(useStatusBarStore.getState().errors).toEqual([]);
  });
});
