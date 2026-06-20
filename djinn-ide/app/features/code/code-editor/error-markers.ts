import type { Location } from "djinn-dev-wasm";
import { StateField, StateEffect, RangeSet, Prec } from "@codemirror/state";
import {
  Decoration,
  type DecorationSet,
  EditorView,
  gutter,
  GutterMarker,
} from "@codemirror/view";

import { CircleIcon } from "@phosphor-icons/react";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";

export interface CodeError {
  position: Location;
  message: string;
}

const errorIconMarkup = renderToStaticMarkup(
  createElement(CircleIcon, {
    weight: "duotone",
    size: 16,
    role: "img",
    "aria-label": "Error",
  }),
);

// 1. The effect that carries new errors into the editor state.
export const setErrors = StateEffect.define<CodeError[]>();

// 2. Whole-line tint.
const errorLineDeco = Decoration.line({
  attributes: { class: "cm-errorLine" },
});

// 3. The gutter icon.
class ErrorGutterMarker extends GutterMarker {
  toDOM() {
    const el = document.createElement("span");
    el.className = "cm-errorGutterMarker";
    el.innerHTML = errorIconMarkup;
    return el;
  }
}
const errorGutterMarker = new ErrorGutterMarker();

// 4. One field holds both decoration sets, rebuilt on each setErrors effect.
interface ErrorState {
  lines: DecorationSet;
  gutter: RangeSet<GutterMarker>;
}

const errorField = StateField.define<ErrorState>({
  create() {
    return { lines: Decoration.none, gutter: RangeSet.empty };
  },
  update(value, tr) {
    // remap existing ranges through edits so they track the right lines
    let lines = value.lines.map(tr.changes);
    let gutter = value.gutter.map(tr.changes);

    for (const effect of tr.effects) {
      if (effect.is(setErrors)) {
        const lineDecos = [];
        const gutterMarks = [];
        for (const err of effect.value) {
          if (err.position.line < 1 || err.position.line > tr.state.doc.lines)
            continue;
          const from = tr.state.doc.line(err.position.line).from;
          lineDecos.push(errorLineDeco.range(from));
          gutterMarks.push(errorGutterMarker.range(from));
        }
        lines = Decoration.set(lineDecos, true);
        gutter = RangeSet.of(gutterMarks, true);
      }
    }
    return { lines, gutter };
  },
  provide: (f) => EditorView.decorations.from(f, (v) => v.lines),
});

// 5. The gutter column that reads markers from the field.
const errorGutter = gutter({
  class: "cm-errorGutter",
  markers: (view) => view.state.field(errorField).gutter,
  initialSpacer: () => errorGutterMarker,
});

// Bundle as one extension.
export const errorMarkers = [errorField, Prec.high(errorGutter)];
