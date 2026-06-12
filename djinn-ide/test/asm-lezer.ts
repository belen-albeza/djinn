import { mock } from "bun:test";

const grammarParser = { configure: () => grammarParser };

type AsmLezer = typeof import("~/features/code/asm-lang/asm-lezer");

let realAsmLezer: AsmLezer | undefined;

// Bun's test runner does not compile Lezer grammars the way Vite does.
mock.module("~/features/code/asm-lang/asm.grammar", () => ({
  parser: grammarParser,
}));

export async function ensureAsmLezerLoadable() {
  if (realAsmLezer) return realAsmLezer;

  realAsmLezer = await import("~/features/code/asm-lang/asm-lezer");
  return realAsmLezer;
}

export type MockAsmLezerOptions = {
  asm?: AsmLezer["asm"];
};

export async function mockAsmLezer(options: MockAsmLezerOptions = {}) {
  await ensureAsmLezerLoadable();

  mock.module("~/features/code/asm-lang/asm-lezer", () => ({
    asm: options.asm ?? (() => []),
  }));
}

export function restoreAsmLezer() {
  if (!realAsmLezer) return;

  mock.module("~/features/code/asm-lang/asm-lezer", () => realAsmLezer!);
}
