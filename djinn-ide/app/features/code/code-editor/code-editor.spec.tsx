import { it, expect, describe, afterAll, beforeEach } from "bun:test";
import { render, getByTestId } from "@testing-library/react";
import { mockAsmLezer, restoreAsmLezer } from "#test/asm-lezer";
import { restoreDjinnDevWasm } from "#test/djinn-dev-wasm";
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

  it("Renders the source code", async () => {
    useProjectStore.setState(anyProjectWithSourceCode("; Hello, world!"));
    const { container } = render(<CodeEditor />);

    const editor = getByTestId(container, "code-editor-content");
    expect(editor.textContent).toContain("; Hello, world!");
  });
});
