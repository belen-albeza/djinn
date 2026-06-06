import type { Route } from "./+types/home";
import { LegoIcon } from "@phosphor-icons/react";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ];
}

export default function Home() {
  return (
    <>
      <header className="p-4 bg-ink text-paper">
        <h1 className="text-2xl font-bold uppercase text-burst flex items-center gap-2">
          <LegoIcon size={32} /> Djinn
        </h1>
      </header>
      <main className="p-8">
        <blockquote className="max-w-xl mx-auto">
          <p className="text-lg mb-4">
            When you don't create things, you become defined by your tastes
            rather than ability. Your tastes only narrow and exclude people. So
            create.
          </p>
          <p>—Why the lucky stiff</p>
        </blockquote>
      </main>
    </>
  );
}
