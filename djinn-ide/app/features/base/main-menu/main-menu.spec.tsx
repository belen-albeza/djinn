import { it, expect, describe } from "bun:test";
import {
  fireEvent,
  getByRole,
  getByText,
  render,
  screen,
} from "@testing-library/react";
import { createMemoryRouter, RouterProvider } from "react-router";
import { renderWithRouter } from "#test/testing-library";
import MainMenu from "./main-menu";

describe("MainMenu", () => {
  it("Shows the main menu", () => {
    const { container } = renderWithRouter(<MainMenu />);
    expect(getByText(container, "New Project")).toBeInTheDocument();
  });

  it("Shows the confirm new project modal on New Project click", () => {
    const { container } = renderWithRouter(<MainMenu />);
    fireEvent.click(getByRole(container, "button", { name: "New Project" }));

    expect(
      screen.getByRole("heading", { name: "Start a new project?" }),
    ).toBeInTheDocument();
  });

  it("Redirects to the new project route on confirmation", () => {
    const router = createMemoryRouter(
      [
        { path: "/", element: <MainMenu /> },
        { path: "/new", element: <>New project</> },
      ],
      { initialEntries: ["/"] },
    );
    const { container } = render(<RouterProvider router={router} />);

    fireEvent.click(getByRole(container, "button", { name: "New Project" }));
    fireEvent.click(screen.getByRole("button", { name: "Discard & create" }));

    expect(router.state.location.pathname).toBe("/new");
  });
});
