import { it, expect, describe, spyOn } from "bun:test";
import {
  fireEvent,
  getByRole,
  getByText,
  render,
} from "@testing-library/react";
import { createMemoryRouter, RouterProvider } from "react-router";
import { renderWithRouter } from "#test/testing-library";
import MainMenu from "./main-menu";
import New from "~/routes/new";

describe("MainMenu", () => {
  it("Shows the main menu", () => {
    const { container } = renderWithRouter(<MainMenu />);
    expect(getByText(container, "New Project")).toBeInTheDocument();
  });

  it("Redirects to the new project route when the new project button is clicked", () => {
    // accept the confirmation dialog
    spyOn(window, "confirm").mockReturnValue(true);

    const router = createMemoryRouter(
      [
        { path: "/", element: <MainMenu /> },
        { path: "/new", element: <New /> },
      ],
      { initialEntries: ["/"] },
    );
    const { container } = render(<RouterProvider router={router} />);

    fireEvent.click(getByRole(container, "button", { name: "New Project" }));

    expect(window.confirm).toHaveBeenCalled();
    expect(router.state.location.pathname).toBe("/new");
  });
});
