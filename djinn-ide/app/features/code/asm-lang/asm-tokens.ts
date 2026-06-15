// Case-insensitive specialization for bare words (lezer tokens are case-sensitive).

import { Opcode, Bool } from "./asm.grammar.terms";

const OPCODES = new Set([
  "noop",
  "push",
  "pop",
  "dup",
  "spwn",
  "yld",
  "sig",
  "stg",
  "ldg",
  "stl",
  "ldl",
  "dev",
  "lth",
  "leq",
  "gth",
  "geq",
  "eq",
  "neq",
  "mod",
  "add",
  "sub",
  "mul",
  "div",
  "inc",
  "dec",
  "jnz",
  "jmp",
]);

export function specializeWord(value: string): number {
  const v = value.toLowerCase();
  if (v === "true" || v === "false") return Bool;
  if (OPCODES.has(v)) return Opcode;
  return -1; // not specialized: keep it as a generic Word
}
