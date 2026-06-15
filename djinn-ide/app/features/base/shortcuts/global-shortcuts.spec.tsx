import { it, expect, describe, mock, afterAll, beforeEach } from "bun:test";
import { render, waitFor, fireEvent } from "@testing-library/react";
import { mockDjinnDevWasm, restoreDjinnDevWasm } from "#test/djinn-dev-wasm";
import { useProjectStore } from "~/features/base/project.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";
import { useEditorStore } from "~/features/code/code-editor/editor.store";

afterAll(() => {
  restoreDjinnDevWasm();
});

const { useGlobalShortcuts } = await import("./use-global-shortcuts");
const { globalShortcuts } = await import("./global-shortcuts");

function Harness() {
  useGlobalShortcuts(globalShortcuts);
  return null;
}

describe("Global shortcuts", () => {
  beforeEach(() => {
    useStatusBarStore.getState().reset();
  });

  afterAll(() => {
    useEditorStore.getState().setReadCodeFn(null);
  });

  it("Saves the editor contents with Mod-s", () => {
    useEditorStore.getState().setReadCodeFn(() => "; edited source");
    useProjectStore.setState({ title: "Lorem", sourceCode: "" });

    render(<Harness />);

    fireEvent.keyDown(window, { key: "s", ctrlKey: true });

    expect(useProjectStore.getState().sourceCode).toBe("; edited source");
    expect(useStatusBarStore.getState().savedTick).toBeGreaterThan(0);
  });

  it("Builds a project with Mod-b", async () => {
    const build = mock();
    await mockDjinnDevWasm({ build });

    useProjectStore.setState({ title: "Lorem", sourceCode: "" });
    useEditorStore.getState().setReadCodeFn(() => "; Hello, world!");
    render(<Harness />);

    fireEvent.keyDown(window, { key: "b", ctrlKey: true });

    await waitFor(() => {
      expect(build).toHaveBeenCalledWith({
        title: "Lorem",
        sourceCode: "; Hello, world!",
      });
    });
    expect(useStatusBarStore.getState().messages).toEqual([
      { type: "success", message: "Built without errors." },
    ]);
  });
});
