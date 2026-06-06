import { it, expect, describe } from "bun:test";
import { render, screen } from "@testing-library/react";
import Home from "./home";

describe("Home", () => {
  it("Renders Djinn heading", () => {
    render(<Home />);

    expect(
      screen.getByRole("heading", { level: 1, name: /djinn/i }),
    ).toBeInTheDocument();
  });
});
