import clsx from "clsx";
import { useNavigate } from "react-router";

import type { Route } from "./+types/home";
import TextInput from "~/ui/text-input";
import Button from "~/ui/button";
import { useProjectStore } from "~/features/base/project.store";

export function meta({}: Route.MetaArgs) {
  return [{ title: "New Project" }];
}

function Quote({ className }: { className?: string }) {
  return (
    <blockquote className={clsx("max-w-xl text-sand-600", className)}>
      <p className="text-lg mb-4 italic">
        “When you don't create things, you become defined by your tastes rather
        than ability. Your tastes only narrow and exclude people.{" "}
        <b className="text-ink">So create.</b>”
      </p>
      <p className="text-right">—Why the Lucky Stiff</p>
    </blockquote>
  );
}

function NewGameForm() {
  const navigate = useNavigate();

  const handleSubmit = (event: React.SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();

    const formData = new FormData(event.target as HTMLFormElement);
    const title = formData.get("title") as string;
    useProjectStore.getState().setTitle(title);

    navigate("/");
  };

  return (
    <form className="grid gap-4" onSubmit={handleSubmit}>
      <TextInput
        name="title"
        placeholder="Rocky Galaxy"
        label="Project title"
        required
        autoFocus
      />
      <p className="text-sand-600 text-sm flex flex-row gap-4 items-center">
        <Button type="submit" className="w-fit">
          Create project
        </Button>{" "}
        <span>
          or press{" "}
          <kbd className=" text-sand-800 px-2 py-0.5 rounded-sharp border-ui border-sand-200 font-mono text-sm">
            Enter
          </kbd>
        </span>
      </p>
    </form>
  );
}

export default function New() {
  return (
    <section className="grid flex-1 gap-8 place-content-center">
      <h1 className="font-sans text-6xl font-bold">
        Name your game<span className="text-burst">.</span>
      </h1>

      <NewGameForm />
      <Quote className="mt-2" />
    </section>
  );
}
