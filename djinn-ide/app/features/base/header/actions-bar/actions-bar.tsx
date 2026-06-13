import Button from "~/ui/button";
import { HammerIcon, PlayIcon } from "@phosphor-icons/react";
import { buildProject } from "~/features/base/shortcuts/actions/build-project";
import { runProject } from "~/features/base/shortcuts/actions/run-project";
import {
  useCanBuild,
  useCanRun,
} from "~/features/base/shortcuts/actions/execution.rules";

export default function ActionsBar() {
  const canBuild = useCanBuild();
  const canRun = useCanRun();

  return (
    <menu className="flex direction-row gap-1 items-center">
      <li className="h-full flex items-center">
        <Button
          variant="ghost"
          iconSize={24}
          icon={HammerIcon}
          aria-label="Build project"
          onClick={buildProject}
          disabled={!canBuild}
          className="hover:text-burst"
        >
          Build
        </Button>
      </li>
      <li className="h-full flex items-center">
        <Button
          variant="primary"
          iconSize={24}
          icon={PlayIcon}
          aria-label="Run project"
          onClick={runProject}
          disabled={!canRun}
        >
          Run
        </Button>
      </li>
    </menu>
  );
}
