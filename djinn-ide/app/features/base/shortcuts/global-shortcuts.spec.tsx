import { it, expect, describe, mock, afterAll, beforeEach } from "bun:test";
import { render, waitFor, fireEvent } from "@testing-library/react";
import { mockDjinnDevWasm, restoreDjinnDevWasm } from "#test/djinn-dev-wasm";
import { useProjectStore } from "~/features/base/project.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";

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

  it("Builds a project with Mod-b", async () => {
    const build = mock((_title: string) => ({ title: "Lorem" }));
    await mockDjinnDevWasm({ build });

    useProjectStore.setState({ title: "Lorem", sourceCode: "; Hello, world!" });
    render(<Harness />);

    fireEvent.keyDown(window, { key: "b", ctrlKey: true });

    await waitFor(() => {
      expect(build).toHaveBeenCalledWith("Lorem");
    });
    expect(useStatusBarStore.getState().errors).toEqual([]);
  });
});
