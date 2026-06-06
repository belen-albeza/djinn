import { it, expect, describe } from "bun:test";
import {
  fireEvent,
  getByRole,
  getByDisplayValue,
  render,
} from "@testing-library/react";
import { getByAccessibleText } from "#test/testing-library";
import Header from "./header";

describe("Header", () => {
  it("Shows the project title", () => {
    const { container } = render(<Header />);
    expect(getByAccessibleText(container, "Rocky Galaxy")).toBeVisible();
  });

  it("Edits the project title", () => {
    const { container } = render(<Header />);
    const editButton = getByRole(container, "button", { name: "Edit title" });

    fireEvent.click(editButton);
    const input = getByDisplayValue(container, "Rocky Galaxy");
    fireEvent.change(input, { target: { value: "Lorem Ipsum" } });
    fireEvent.keyDown(input, { key: "Enter" });

    expect(getByAccessibleText(container, "Lorem Ipsum")).toBeVisible();
  });

  it("Cancels editing the project title", () => {
    const { container } = render(<Header />);
    const editButton = getByRole(container, "button", { name: "Edit title" });

    fireEvent.click(editButton);
    const input = getByDisplayValue(container, "Rocky Galaxy");
    fireEvent.change(input, { target: { value: "Lorem Ipsum" } });
    fireEvent.keyDown(input, { key: "Escape" });

    expect(getByAccessibleText(container, "Rocky Galaxy")).toBeVisible();
  });
});
