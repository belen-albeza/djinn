import { useRef, useEffect } from "react";

import { basicSetup } from "codemirror";
import { EditorView, keymap } from "@codemirror/view";
import { EditorState, Prec } from "@codemirror/state";
import { defaultKeymap } from "@codemirror/commands";

import { codeEditorTheme } from "./code-editor.theme";
import { useProjectStore } from "~/features/base/project.store";
import { useEditorStore } from "./editor.store";
import { asm } from "../asm-lang/asm-lezer";

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
  ]),
);

export default function CodeEditor() {
  const editorRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!editorRef.current) return;

    const startState = EditorState.create({
      doc: useProjectStore.getState().sourceCode,
      extensions: [
        basicSetup,
        codeEditorTheme,
        overridenShortcuts,
        keymap.of(defaultKeymap),
        asm(),
        EditorView.contentAttributes.of({
          "data-testid": "code-editor-content",
        }),
      ],
    });
    const view = new EditorView({
      state: startState,
      parent: editorRef.current!,
    });

    useEditorStore.getState().setReadCodeFn(() => view.state.doc.toString());

    // autofocus the editor on mount
    view.focus();

    return () => {
      useEditorStore.getState().setReadCodeFn(null);
      view.destroy();
    };
  }, [editorRef.current]);

  return (
    <section
      ref={editorRef}
      className="h-full min-h-0 w-full overflow-hidden"
    ></section>
  );
}
