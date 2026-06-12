import { mock, type Mock } from "bun:test";

type DjinnDevWasm = typeof import("djinn-dev-wasm");

let realDjinnDevWasm: DjinnDevWasm | undefined;

// Bun's test runner does not initialize WASM the way Vite does.
mock.module("djinn-dev-wasm/djinn_dev_wasm_bg.wasm", () => ({
  __wbindgen_start: () => {},
}));

export async function ensureDjinnDevWasmLoadable() {
  if (realDjinnDevWasm) return realDjinnDevWasm;

  realDjinnDevWasm = await import("djinn-dev-wasm");
  return realDjinnDevWasm;
}

export type BuildMock = Mock<(title: string) => { title: string }>;

export type MockDjinnDevWasmOptions = {
  build?: BuildMock;
  init?: Mock<() => void>;
};

export async function mockDjinnDevWasm(options: MockDjinnDevWasmOptions = {}) {
  const real = await ensureDjinnDevWasmLoadable();
  const build =
    options.build ?? mock((_title: string) => ({ title: "Lorem Ipsum" }));

  mock.module("djinn-dev-wasm", () => ({
    ...real,
    build,
    init: options.init ?? real.init,
  }));

  return { build };
}

export function restoreDjinnDevWasm() {
  if (!realDjinnDevWasm) return;

  mock.module("djinn-dev-wasm", () => realDjinnDevWasm!);
}
