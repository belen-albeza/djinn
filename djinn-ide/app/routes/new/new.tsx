import clsx from "clsx";
import { useState } from "react";
import { useNavigate } from "react-router";
import { DiceThreeIcon } from "@phosphor-icons/react";

import type { Route } from "./+types/new";
import TextInput from "~/ui/text-input";
import Button from "~/ui/button";
import { useProjectStore } from "~/features/base/project.store";
import { randomGameTitle } from "~/utils/random";

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
  const placeholderTitle = randomGameTitle();
  const [inputValue, setInputValue] = useState("");
  const [rollKey, setRollKey] = useState(0);

  const handleSubmit = (event: React.SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();

    useProjectStore.getState().setTitle(inputValue);
    navigate("/");
  };

  const handleRandomize = () => {
    setInputValue(randomGameTitle());
    setRollKey((key) => key + 1);
  };

  return (
    <form className="grid gap-4" onSubmit={handleSubmit}>
      <p className="flex flex-row gap-2 items-center">
        <TextInput
          name="title"
          value={inputValue}
          onChange={(event) => setInputValue(event.target.value)}
          placeholder={placeholderTitle}
          label="Project title"
          required
          autoFocus
          className="flex-1"
        />
        <Button
          type="button"
          variant="ghost"
          icon={DiceThreeIcon}
          iconSize={48}
          iconKey={rollKey}
          title="Randomize!"
          onClick={handleRandomize}
          iconClassName={clsx(
            "mt-6",
            rollKey > 0 &&
              "motion-safe:animate-[spin_0.45s_ease-in-out_1] motion-reduce:animate-none",
          )}
        />
      </p>
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
