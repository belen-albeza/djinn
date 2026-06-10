import { useRef, useEffect } from "react";

import { basicSetup } from "codemirror";
import { EditorView, keymap } from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { defaultKeymap } from "@codemirror/commands";

import { codeEditorTheme } from "./code-editor.theme";
import { useProjectStore } from "~/features/base/project.store";
import { useStatusBarStore } from "~/features/base/status-bar/status.store";

const customShortcuts = keymap.of([
  {
    key: "Mod-s",
    preventDefault: true,
    run: (view) => {
      useProjectStore.getState().setSourceCode(view.state.doc.toString());
      useStatusBarStore.getState().notifySaved();
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

    // autofocus the editor on mount
    view.focus();

    return () => {
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
