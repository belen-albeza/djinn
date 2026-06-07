import { it, expect, describe, beforeEach } from "bun:test";
import {
  fireEvent,
  getByRole,
  getByDisplayValue,
  queryByRole,
} from "@testing-library/react";
import { getByAccessibleText, renderWithRouter } from "#test/testing-library";
import {
  defaultProject,
  useProjectStore,
  type ProjectSnapshot,
} from "~/features/base/project.store";
import Header from "./header";

function anyProjectWithTitle(title: string): ProjectSnapshot {
  return { title };
}

beforeEach(() => {
  useProjectStore.setState(defaultProject);
});

describe("Header", () => {
  it("Shows the project title if available", () => {
    useProjectStore.setState(anyProjectWithTitle("Rocky Galaxy"));
    const { container } = renderWithRouter(<Header />);
    expect(getByAccessibleText(container, "Rocky Galaxy")).toBeVisible();
  });

  it("Shows no edit button if no title is available", () => {
    const { container } = renderWithRouter(<Header />);
    expect(
      queryByRole(container, "button", { name: "Edit title" }),
    ).not.toBeInTheDocument();
  });

  it("Edits the project title", () => {
    useProjectStore.setState(anyProjectWithTitle("Rocky Galaxy"));
    const { container } = renderWithRouter(<Header />);
    const editButton = getByRole(container, "button", { name: "Edit title" });

    fireEvent.click(editButton);
    const input = getByDisplayValue(container, "Rocky Galaxy");
    fireEvent.change(input, { target: { value: "Lorem Ipsum" } });
    fireEvent.keyDown(input, { key: "Enter" });

    expect(getByAccessibleText(container, "Lorem Ipsum")).toBeVisible();
  });

  it("Cancels editing the project title", () => {
    useProjectStore.setState(anyProjectWithTitle("Rocky Galaxy"));
    const { container } = renderWithRouter(<Header />);
    const editButton = getByRole(container, "button", { name: "Edit title" });

    fireEvent.click(editButton);
    const input = getByDisplayValue(container, "Rocky Galaxy");
    fireEvent.change(input, { target: { value: "Lorem Ipsum" } });
    fireEvent.keyDown(input, { key: "Escape" });

    expect(getByAccessibleText(container, "Rocky Galaxy")).toBeVisible();
  });
});
