// Type shims for the Lezer Vite/Rollup plugin (@lezer/generator/rollup).
// The plugin compiles `*.grammar` files at build time and resolves
// `*.grammar.terms` to the generated term-id table.

declare module "*.grammar" {
  import type { LRParser } from "@lezer/lr";
  export const parser: LRParser;
}

declare module "*.grammar.terms" {
  // Term ids produced by the grammar.
  export const Opcode: number;
  export const Bool: number;
}
