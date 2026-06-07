import type { Route } from "./+types/home";

export function meta({}: Route.MetaArgs) {
  return [{ title: "Djinn IDE" }];
}

export function Quote() {
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

export default function Home() {
  return <Quote />;
}
