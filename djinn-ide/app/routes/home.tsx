import { redirect } from "react-router";

import type { Route } from "./+types/home";
import { useProjectStore } from "~/features/base/project.store";

export function meta({}: Route.MetaArgs) {
  return [{ title: "Djinn IDE" }];
}

export function clientLoader() {
  if (!useProjectStore.getState().title) {
    return redirect("/new");
  }

  return null;
}

export default function Home() {
  return <p>…</p>;
}
