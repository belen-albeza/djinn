import { useRef, useEffect } from "react";

import { basicSetup } from "codemirror";
import { EditorView, keymap } from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { defaultKeymap } from "@codemirror/commands";

import { codeEditorTheme } from "./code-editor.theme";
import { useProjectStore } from "~/features/base/project.store";

const customShortcuts = keymap.of([
  {
    key: "Mod-s",
    preventDefault: true,
    run: (view) => {
      useProjectStore.getState().setSourceCode(view.state.doc.toString());
      return true;
    },
  },
]);

export default function CodeEditor() {
  const editorRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!editorRef.current) return;

    const startState = EditorState.create({
      doc: useProjectStore.getState().sourceCode,
      extensions: [
        basicSetup,
        codeEditorTheme,
        keymap.of(defaultKeymap),
        customShortcuts,
      ],
    });
    const view = new EditorView({
      state: startState,
      parent: editorRef.current!,
    });

    return () => {
      view.destroy();
    };
  }, [editorRef.current]);

  return (
    <article className="grid h-full min-h-0 w-full grid-rows-[1fr_auto] overflow-hidden">
      <section
        ref={editorRef}
        className="h-full min-h-0 w-full overflow-hidden"
      ></section>
      <footer className="px-4 py-1 border-t border-sand-200 bg-sand-100">
        <p className="text-small">No errors.</p>
      </footer>
    </article>
  );
}
