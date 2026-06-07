import { afterEach, expect, mock } from "bun:test";
import {
  cleanup,
  getAllByText,
  getByText,
  render,
  type RenderOptions,
} from "@testing-library/react";
import { getElementError, isInaccessible } from "@testing-library/dom";
import * as matchers from "@testing-library/jest-dom/matchers";
import { createElement, type ReactElement, type ReactNode } from "react";
import { MemoryRouter, type MemoryRouterProps } from "react-router";

type RenderWithRouterOptions = RenderOptions & {
  router?: MemoryRouterProps;
};

export function renderWithRouter(
  ui: ReactElement,
  { router, ...options }: RenderWithRouterOptions = {},
) {
  function Wrapper({ children }: { children: ReactNode }) {
    return createElement(MemoryRouter, router, children);
  }

  return render(ui, { wrapper: Wrapper, ...options });
}

expect.extend(matchers);

afterEach(() => {
  cleanup();
  mock.restore();
});

type GetAllByTextOptions = Parameters<typeof getAllByText>[2];

export function getByAccessibleText(
  container: HTMLElement,
  text: Parameters<typeof getAllByText>[1],
  options?: GetAllByTextOptions,
): HTMLElement {
  const matches = getAllByText(container, text, options);
  const accessible = matches.filter((element) => !isInaccessible(element));

  if (accessible.length === 1) {
    return accessible[0]!;
  }

  if (accessible.length > 1) {
    throw getElementError(
      `Found multiple accessible elements with the text: ${String(text)}`,
      container,
    );
  }

  if (matches.length > 0) {
    throw getElementError(
      `Found matching text but only in inaccessible elements (e.g. aria-hidden): ${String(text)}`,
      container,
    );
  }

  getByText(container, text, options);
  throw new Error("unreachable");
}
