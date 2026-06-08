import { it, expect, describe, mock } from "bun:test";
import {
  fireEvent,
  getByRole,
  getByDisplayValue,
  render,
} from "@testing-library/react";
import EditableInput from "./editable-input";

describe("EditableInput", () => {
  it("Commits changes on blur", () => {
    const onChange = mock(() => {});
    const { container } = render(
      <EditableInput
        value="Rocky Galaxy"
        onChange={onChange}
        editIconAriaLabel="Edit title"
      />,
    );

    fireEvent.click(getByRole(container, "button", { name: "Edit title" }));
    const input = getByDisplayValue(container, "Rocky Galaxy");
    fireEvent.change(input, { target: { value: "Lorem Ipsum" } });
    fireEvent.blur(input);

    expect(onChange).toHaveBeenCalledWith("Lorem Ipsum");
  });

  it("Commits changes on blur after a previous Enter commit", () => {
    const onChange = mock(() => {});
    const { container, rerender } = render(
      <EditableInput
        value="Rocky Galaxy"
        onChange={onChange}
        editIconAriaLabel="Edit title"
      />,
    );

    fireEvent.click(getByRole(container, "button", { name: "Edit title" }));
    let input = getByDisplayValue(container, "Rocky Galaxy");
    fireEvent.change(input, { target: { value: "First Edit" } });
    fireEvent.keyDown(input, { key: "Enter" });

    rerender(
      <EditableInput
        value="First Edit"
        onChange={onChange}
        editIconAriaLabel="Edit title"
      />,
    );

    fireEvent.click(getByRole(container, "button", { name: "Edit title" }));
    input = getByDisplayValue(container, "First Edit");
    fireEvent.change(input, { target: { value: "Second Edit" } });
    fireEvent.blur(input);

    expect(onChange).toHaveBeenLastCalledWith("Second Edit");
  });

  it("Does not commit changes on Escape", () => {
    const onChange = mock(() => {});
    const { container } = render(
      <EditableInput
        value="Rocky Galaxy"
        onChange={onChange}
        editIconAriaLabel="Edit title"
      />,
    );

    fireEvent.click(getByRole(container, "button", { name: "Edit title" }));
    const input = getByDisplayValue(container, "Rocky Galaxy");
    fireEvent.change(input, { target: { value: "Lorem Ipsum" } });
    fireEvent.keyDown(input, { key: "Escape" });

    expect(onChange).not.toHaveBeenCalled();
  });
});
