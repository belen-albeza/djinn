import { it, expect, describe, mock, afterAll, beforeEach } from "bun:test";
import {
  render,
  waitFor,
  getByTestId,
  fireEvent,
} from "@testing-library/react";
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

describe("CodeEditor", () => {
  beforeEach(() => {
    useStatusBarStore.getState().reset();
  });

  it("Builds a project", async () => {
    const build = mock((_title: string) => ({ title: "Lorem" }));
    await mockDjinnDevWasm({ build });

    useProjectStore.setState(anyProjectWithSourceCode("; Hello, world!"));
    const { container } = render(<CodeEditor />);

    const editor = getByTestId(container, "code-editor-content");
    fireEvent.keyDown(editor, { key: "b", ctrlKey: true });

    await waitFor(() => {
      expect(build).toHaveBeenCalledWith("Lorem");
    });
    expect(useStatusBarStore.getState().errors).toEqual([]);
  });
});
