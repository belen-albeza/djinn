import { afterEach, expect } from "bun:test";
import { cleanup, getAllByText, getByText } from "@testing-library/react";
import { getElementError, isInaccessible } from "@testing-library/dom";
import * as matchers from "@testing-library/jest-dom/matchers";

expect.extend(matchers);

afterEach(() => {
  cleanup();
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
