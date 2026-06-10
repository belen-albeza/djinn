import { it, expect, describe } from "bun:test";
import { render, getByText, queryByText } from "@testing-library/react";
import ErrorDisplay from "./error-display";

describe("ErrorDisplay", () => {
  it("Renders 'no errors' message when there are no errors", () => {
    const { container } = render(<ErrorDisplay errors={[]} />);
    expect(getByText(container, "No errors")).toBeVisible();
  });

  it("Renders error message when there are errors", () => {
    const { container } = render(
      <ErrorDisplay errors={["Lorem ipsum dolors"]} />,
    );
    expect(getByText(container, "1")).toBeVisible();
    expect(getByText(container, "Lorem ipsum dolors")).toBeVisible();
  });

  it("Renders only the first error message", () => {
    const { container } = render(
      <ErrorDisplay errors={["Lorem ipsum dolors", "Sit amet"]} />,
    );
    expect(getByText(container, "2")).toBeVisible();
    expect(getByText(container, "Lorem ipsum dolors")).toBeVisible();
    expect(queryByText(container, "Sit amet")).not.toBeInTheDocument();
  });
});
