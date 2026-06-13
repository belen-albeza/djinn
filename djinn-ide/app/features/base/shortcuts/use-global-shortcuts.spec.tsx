import { it, expect, describe, mock } from "bun:test";
import { render, fireEvent } from "@testing-library/react";

import { useGlobalShortcuts, type Shortcut } from "./use-global-shortcuts";

function Harness({ shortcuts }: { shortcuts: Shortcut[] }) {
  useGlobalShortcuts(shortcuts);
  return null;
}

describe("useGlobalShortcuts", () => {
  it("Runs a shortcut when its key and modifier match", () => {
    const run = mock(() => {});
    render(<Harness shortcuts={[{ key: "b", mod: true, run }]} />);

    fireEvent.keyDown(window, { key: "b", ctrlKey: true });

    expect(run).toHaveBeenCalledTimes(1);
  });

  it("Ignores the key when the required modifier is absent", () => {
    const run = mock(() => {});
    render(<Harness shortcuts={[{ key: "b", mod: true, run }]} />);

    fireEvent.keyDown(window, { key: "b" });

    expect(run).not.toHaveBeenCalled();
  });

  it("Stops listening after unmount", () => {
    const run = mock(() => {});
    const { unmount } = render(
      <Harness shortcuts={[{ key: "b", mod: true, run }]} />,
    );

    unmount();
    fireEvent.keyDown(window, { key: "b", ctrlKey: true });

    expect(run).not.toHaveBeenCalled();
  });
});
