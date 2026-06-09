import { useRef, useEffect } from "react";

import { basicSetup } from "codemirror";
import { EditorView, keymap } from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { defaultKeymap } from "@codemirror/commands";

import { codeEditorTheme } from "./code-editor.theme";

// TODO: Replace with the program stored in the project state.
const sampleProgram = `~main:
  #0 STG $score
  SPWN ~score
  POP

  SPWN ~ship
  POP

  @frame-loop:
    YLD
    #0 #100 DEV :math :rand-range
    #10 LTH
    JNZ @spawn-enemy
    JMP @frame-loop
    @spawn-enemy:
      #0 #160 DEV :math :rand-range
      SPWN ~enemy
      POP
    JMP @frame-loop

~score:
  #2 STL $x
  #2 STL $y
  #255 STL $z

  @frame-loop:
    #7 LDL $z LDL $y LDL $x LDG $score DEV :video :draw-text
    YLD
    JMP @frame-loop

~ship:
  #1 STL $image
  #80 STL $x
  #136 STL $y
  #0 STL $t
  #false STL $shoot-locked

  @frame-loop:
    ; move right?
    #:btn-right DEV :gamepad :is-button-down
    JNZ @move-right
    JMP @check-left
    @move-right:
      LDL $x #1.5 ADD
      #160 MOD STL $x

    ; move left?
    @check-left:
      #:btn-left DEV :gamepad :is-button-down
      JNZ @move-left
      JMP @check-shoot
    @move-left:
      LDL $x #-1.5 ADD
      #160 MOD STL $x

    @check-shoot:
      #:btn-o DEV :gamepad :is-button-down
      JNZ @check-locked
        #false STL $shoot-locked
        #0 STL $t
        JMP @frame
      @check-locked:
        LDL $shoot-locked
        JNZ @check-freq
          #true STL $shoot-locked
          JMP @spawn-bullet
      @check-freq:
        LDL $t INC STL $t
        LDL $t #8 MOD
        #0 EQ
        JNZ @spawn-bullet
          #true STL $shoot-locked
          JMP @frame

    @spawn-bullet:
      LDL $y #-4 ADD
      LDL $x
      SPWN ~bullet
      POP

    ; render ship
    @frame:
      YLD
      JMP @frame-loop

~bullet($x, $y):
  #10 STL $image

  @frame-loop:
    LDL $y #-2 ADD
    DUP
    STL $y
    #-4 LEQ
    JNZ @end
    YLD
    JMP @frame-loop
  @end:

~enemy($x):
  #-4 STL $y
  #3 STL $image
  #-1 #1 DEV :math :rand-range
  STL $speed-x
  #1 #2 DEV :math :rand-range
  STL $speed-y

  @frame-loop:
    ; move enemy
    LDL $x LDL $speed-x ADD STL $x
    LDL $y LDL $speed-y ADD
    DUP
    STL $y

    ; render sprite
    YLD
    ; check for collision with bullet
    #~bullet DEV :video :collision
    DUP
    STL $bullet-id
    JNZ @kill

    ; out of bounds check with $y
    #148 GTH
    JNZ @end
    JMP @frame-loop

    ; kill enemy and bullet
    @kill:
      LDL $bullet-id SIG :kill
      ; increment score
      LDG $score #5 ADD STG $score
      ; spawn explosion
      LDL $y LDL $x SPWN ~explosion
      POP

    @end:

~explosion($x, $y):
  #1 STL $r
  @frame-loop:
    ; draw circle
    #7 LDL $r LDL $y LDL $x DEV :video :draw-circle-filled
    LDL $r #1.25 ADD STL $r
    ; render
    YLD
    ; check for end of explosion
    LDL $r #8 LTH
    JNZ @frame-loop
`;

export default function CodeEditor() {
  const editorRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!editorRef.current) return;

    const startState = EditorState.create({
      doc: sampleProgram,
      extensions: [basicSetup, codeEditorTheme, keymap.of(defaultKeymap)],
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
