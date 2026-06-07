import { it, expect, describe } from "bun:test";
import {
  getByText,
  getByRole,
  fireEvent,
  getByLabelText,
  render,
} from "@testing-library/react";
import { createMemoryRouter, RouterProvider } from "react-router";
import { renderWithRouter } from "#test/testing-library";

import New from "./new";

describe("New project route", () => {
  it("Shows the new project form", () => {
    const { container } = renderWithRouter(<New />, {
      router: { initialEntries: ["/new"] },
    });
    expect(getByText(container, "Name your game")).toBeInTheDocument();
  });

  it("Redirects to the home route when the form is submitted", () => {
    const router = createMemoryRouter(
      [
        { path: "/new", element: <New /> },
        { path: "/", element: <div>Home</div> },
      ],
      { initialEntries: ["/new"] },
    );
    const { container } = render(<RouterProvider router={router} />);

    const input = getByLabelText(container, "Project title");
    fireEvent.change(input, { target: { value: "Rocky Galaxy" } });
    fireEvent.click(getByRole(container, "button", { name: "Create project" }));

    expect(router.state.location.pathname).toBe("/");
  });
});
