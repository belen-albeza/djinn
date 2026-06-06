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
    <div className="flex min-h-[calc(100dvh-8rem)] items-center justify-center">
      <blockquote className="max-w-xl">
        <p className="text-lg mb-4 italic">
          “When you don't create things, you become defined by your tastes
          rather than ability. Your tastes only narrow and exclude people.{" "}
          <b>So create.</b>”
        </p>
        <p>—Why the Lucky Stiff</p>
      </blockquote>
    </div>
  );
}
