import { it, expect, describe } from "bun:test";
import {
  render,
  getByText,
  queryByText,
  getByRole,
} from "@testing-library/react";
import MessageDisplay from "./message-display";

describe("MessageDisplay", () => {
  it("Renders 'Ready.' message when there are no messages", () => {
    const { container } = render(<MessageDisplay messages={[]} />);
    expect(getByText(container, "Ready.")).toBeVisible();
  });

  it("Renders message when there is one", () => {
    const { container } = render(
      <MessageDisplay
        messages={[{ type: "error", message: "Lorem ipsum dolors" }]}
      />,
    );
    expect(getByRole(container, "img", { name: "Error" })).toBeVisible();
    expect(getByText(container, "Lorem ipsum dolors")).toBeVisible();
  });

  it("Renders only the first message", () => {
    const { container } = render(
      <MessageDisplay
        messages={[
          { type: "error", message: "Lorem ipsum dolors" },
          { type: "error", message: "Sit amet" },
        ]}
      />,
    );
    expect(getByText(container, "2")).toBeVisible();
    expect(getByText(container, "Lorem ipsum dolors")).toBeVisible();
    expect(queryByText(container, "Sit amet")).not.toBeInTheDocument();
  });

  it("Renders success messages with the correct icon", () => {
    const { container } = render(
      <MessageDisplay
        messages={[{ type: "success", message: "Lorem ipsum dolors" }]}
      />,
    );
    expect(getByRole(container, "img", { name: "Success" })).toBeVisible();
    expect(getByText(container, "Lorem ipsum dolors")).toBeVisible();
  });

  it("Renders error messages with the correct icon", () => {
    const { container } = render(
      <MessageDisplay
        messages={[{ type: "error", message: "Lorem ipsum dolors" }]}
      />,
    );
    expect(getByRole(container, "img", { name: "Error" })).toBeVisible();
    expect(getByText(container, "Lorem ipsum dolors")).toBeVisible();
  });
});
