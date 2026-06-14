import { EditorView } from "@codemirror/view";

export const codeEditorTheme = EditorView.theme({
  "&": {
    backgroundColor: "var(--color-sand-50)",
    color: "var(--color-ink)",
    fontFamily: "var(--font-mono)",
    fontSize: "1.25rem",
    height: "100%",
    maxHeight: "100%",
    display: "flex",
    flexDirection: "column",
    overflow: "hidden",
  },
  ".cm-content": {
    caretColor: "var(--color-burst)",
  },
  ".cm-cursor": {
    borderLeftColor: "var(--color-burst)",
  },
  ".cm-activeLine": {
    backgroundColor: "var(--color-sand-200-30)",
  },
  ".cm-gutters": {
    backgroundColor: "var(--color-sand-50)",
    color: "var(--color-sand-200)",
    border: "none",
    borderRight: "1px solid var(--color-sand-100)",
  },
  ".cm-activeLineGutter": {
    backgroundColor: "var(--color-sand-100)",
  },
  ".cm-selectionBackground, ::selection": {
    backgroundColor: "var(--color-sand-200) !important",
  },
  ".cm-scroller": {
    flex: "1 1 auto",
    overflow: "auto",
    minHeight: 0,
    scrollbarWidth: "thin",
    scrollbarColor: "var(--color-sand-200) var(--color-sand-50)",
  },
  ".cm-scroller::-webkit-scrollbar": {
    width: "8px",
    height: "8px",
  },
  ".cm-scroller::-webkit-scrollbar-track": {
    background: "var(--color-sand-50)",
  },
  ".cm-scroller::-webkit-scrollbar-thumb": {
    background: "var(--color-sand-200)",
    borderRadius: "var(--radius-sharp)",
  },
  ".cm-scroller::-webkit-scrollbar-thumb:hover": {
    background: "var(--color-sand-400)",
  },
  // Error markers
  ".cm-errorLine": {
    backgroundColor: "var(--color-error-bg, rgba(220, 38, 38, 0.12))",
  },
  ".cm-errorGutter": {
    paddingLeft: "calc(var(--spacing) * 2)",
  },
  ".cm-lineNumbers .cm-gutterElement": {
    paddingLeft: 0,
  },
  ".cm-errorGutter .cm-gutterElement": {
    display: "flex",
    alignItems: "center",
    paddingRight: 0,
  },
  ".cm-errorGutterMarker": {
    color: "var(--color-error, #dc2626)",
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
  },
});
