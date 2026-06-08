import clsx from "clsx";
import { ListIcon, PlusIcon } from "@phosphor-icons/react";
import type { Icon as IconType } from "@phosphor-icons/react";
import { useMatch, useNavigate } from "react-router";
import slugify from "slugify";

import {
  toProjectSnapshot,
  useProjectStore,
} from "~/features/base/project.store";
import { Modal } from "~/ui/modal";
import Button from "~/ui/button";
import { useState } from "react";

interface MenuItemProps {
  children: React.ReactNode;
  icon?: IconType;
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
}

function MenuItem({
  children,
  icon,
  onClick,
  disabled,
  className,
  ...other
}: MenuItemProps) {
  const Icon = icon;

  return (
    <li
      className={clsx(
        "py-2 px-4",
        !disabled && "hover:bg-burst hover:text-ink",
        className,
      )}
      {...other}
    >
      <button
        disabled={disabled}
        onClick={onClick}
        popoverTarget="main-menu"
        popoverTargetAction="hide"
        className="w-full text-left text-sm font-semibold flex direction-row gap-1 items-center disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {Icon && <Icon size={12} weight="bold" />}
        {children}
      </button>
    </li>
  );
}

export function ConfirmNewProjectModal({
  open,
  onCancel,
  onConfirm,
}: {
  open: boolean;
  onCancel: () => void;
  onConfirm: () => void;
}) {
  const projectTitle = useProjectStore((state) => state.title);

  return (
    <Modal
      open={open}
      onClose={onCancel}
      variant="destructive"
      className="max-w-md"
    >
      <header>
        <p className="text-label">Discard changes</p>
        <h2 className="text-heading">Start a new project?</h2>
      </header>
      <p className="text-body">
        Are you sure? The current project{" "}
        <b className="text-ink">{projectTitle ? `${projectTitle}` : ""}</b> will
        be lost. This cannot be undone.
      </p>
      <footer className="flex flex-row gap-2 justify-end">
        <Button variant="ghost" onClick={onCancel}>
          Stay editing
        </Button>
        <Button variant="destructive" onClick={onConfirm}>
          Discard & create
        </Button>
      </footer>
    </Modal>
  );
}

function downloadProject() {
  const project = toProjectSnapshot(useProjectStore.getState());
  // build a JSON blob to download
  const json = JSON.stringify(project);
  const blob = new Blob([json], { type: "application/json" });
  const url = URL.createObjectURL(blob);

  // build a filename from the project title, discarding special characters
  const { title } = project;
  const filename = `${slugify(title, { lower: true })}.json`;

  // trigger download by clicking an orphan, temporary link
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();

  // clean up the temporary URL
  URL.revokeObjectURL(url);
}

function loadProject() {
  const fileInput = document.createElement("input");
  fileInput.type = "file";
  fileInput.accept = "application/json";
  fileInput.onchange = (event) => {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (event) => {
        const result = event.target?.result as string;
        if (result) {
          try {
            const project = JSON.parse(result);
            // TODO: validate the project
            useProjectStore.getState().setProject(project);
          } catch (error) {
            // TODO: show a toast or modal to the user
            console.error("Invalid file", error);
          }
        }
      };
      reader.readAsText(file);
    }
  };
  fileInput.click();
}

export default function MainMenu() {
  const navigate = useNavigate();
  const isOnNewProjectRoute = !!(useMatch({ path: "/new" }) ?? false);
  const [confirmNewProjectModalOpen, setConfirmNewProjectModalOpen] =
    useState(false);

  function handleNewProject() {
    setConfirmNewProjectModalOpen(true);
  }

  function handleConfirmNewProject() {
    setConfirmNewProjectModalOpen(false);
    navigate("/new");
  }

  return (
    <>
      <button
        popoverTarget="main-menu"
        className="anchor-[--main-menu-button] text-2xl font-bold uppercase text-burst flex items-center gap-2"
      >
        <ListIcon size={24} /> Djinn
      </button>

      <menu
        id="main-menu"
        popover="auto"
        aria-label="Main menu"
        className="anchor-below-[--main-menu-button] z-999 bg-ink text-sand-200 min-w-48 rounded-sharp"
      >
        <MenuItem
          icon={PlusIcon}
          onClick={handleNewProject}
          disabled={isOnNewProjectRoute}
        >
          New Project
        </MenuItem>
        <MenuItem onClick={loadProject}>Open Project</MenuItem>
        <MenuItem onClick={downloadProject}>Download</MenuItem>
      </menu>
      <ConfirmNewProjectModal
        open={confirmNewProjectModalOpen}
        onCancel={() => setConfirmNewProjectModalOpen(false)}
        onConfirm={handleConfirmNewProject}
      />
    </>
  );
}
