// Integration glue for the Lezer-based Djinn ASM language.
import { parser } from "./asm.grammar";
import {
  LRLanguage,
  LanguageSupport,
  HighlightStyle,
  syntaxHighlighting,
} from "@codemirror/language";
import { styleTags, tags as t } from "@lezer/highlight";

// Map grammar node names to highlight tags
const asmParser = parser.configure({
  props: [
    styleTags({
      Opcode: t.keyword,
      Bool: t.bool,
      ProcessType: t.typeName,
      Label: t.labelName,
      Symbol: t.atom,
      Variable: t.variableName,
      Number: t.number,
      String: t.string,
      LineComment: t.lineComment,
      PushSigil: t.operator,
      Punctuation: t.punctuation,
    }),
  ],
});

export const asmLanguage = LRLanguage.define({
  parser: asmParser,
  languageData: {
    commentTokens: { line: ";" },
  },
});

export const asmHighlightStyle = HighlightStyle.define([
  { tag: t.keyword, color: "var(--color-ink)", fontWeight: "bold" },
  { tag: t.typeName, color: "var(--color-burst-600)", fontWeight: "bold" },
  { tag: t.labelName, color: "var(--color-info-700)", fontWeight: "normal" },
  { tag: t.atom, color: "var(--color-error-700)" },
  { tag: t.variableName, color: "var(--color-sand-800)", fontStyle: "italic" },
  { tag: t.number, color: "var(--color-error-700)" },
  { tag: t.bool, color: "var(--color-error-700)" },
  { tag: t.string, color: "var(--color-error-700)" },
  { tag: t.operator, color: "var(--color-sand-600)" },
  { tag: t.punctuation, color: "var(--color-sand-600)" },
  { tag: t.lineComment, color: "var(--color-sand-400)", fontStyle: "italic" },
]);

export function asm() {
  return new LanguageSupport(asmLanguage, [
    syntaxHighlighting(asmHighlightStyle),
  ]);
}
