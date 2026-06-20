import { useRef, useEffect } from "react";

import { basicSetup } from "codemirror";
import { EditorView, keymap } from "@codemirror/view";
import { EditorState, Prec } from "@codemirror/state";
import {
  defaultKeymap,
  indentWithTab,
  insertNewlineKeepIndent,
} from "@codemirror/commands";
import { indentUnit } from "@codemirror/language";

import { useProjectStore } from "~/features/base/project.store";
import { asm } from "../asm-lang/asm-lezer";
import { codeEditorTheme } from "./code-editor.theme";
import { useEditorStore } from "./editor.store";
import { errorMarkers, setErrors } from "./error-markers";

// Custom keymaps that override the default setup
const overridenShortcuts = Prec.highest(
  keymap.of([
    // Override the default Meta-Enter shortcut to run the project,
    // so it doesn't insert a new line.
    {
      key: "Meta-Enter",
      run: () => {
        return true; // consume the event
      },
    },
    { key: "Enter", run: insertNewlineKeepIndent },
  ]),
);

// Custom keymaps that do not conflict with the default setup
const customKeymap = [indentWithTab];

export default function CodeEditor() {
  const editorRef = useRef<HTMLDivElement>(null);
  const viewRef = useRef<EditorView | null>(null); // this is codemirror's View

  useEffect(() => {
    if (!editorRef.current) return;

    const startState = EditorState.create({
      doc: useProjectStore.getState().sourceCode,
      extensions: [
        basicSetup,
        codeEditorTheme,
        overridenShortcuts,
        indentUnit.of("  "),
        keymap.of([...customKeymap, ...defaultKeymap]),
        errorMarkers,
        asm(),
        EditorView.editorAttributes.of({
          "data-testid": "code-editor",
        }),
        EditorView.contentAttributes.of({
          "data-testid": "code-editor-content",
        }),
      ],
    });
    const view = new EditorView({
      state: startState,
      parent: editorRef.current!,
    });
    viewRef.current = view;

    useEditorStore.getState().setReadCodeFn(() => view.state.doc.toString());

    // autofocus the editor on mount
    view.focus();

    return () => {
      useEditorStore.getState().setReadCodeFn(null);
      viewRef.current = null;
      view.destroy();
    };
  }, [editorRef.current]);

  const codeErrors = useEditorStore((state) => state.errors);
  useEffect(() => {
    viewRef.current?.dispatch({
      effects: setErrors.of(codeErrors),
    });
  }, [codeErrors]);

  return (
    <section
      ref={editorRef}
      className="h-full min-h-0 w-full overflow-hidden"
    ></section>
  );
}
