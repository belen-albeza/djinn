import { ListIcon, PlusIcon } from "@phosphor-icons/react";
import { useMatch, useNavigate } from "react-router";
import { useState } from "react";

import { MenuItem } from "./menu-item";
import { ConfirmNewProjectModal } from "./confirm-new-project-modal";
import { LoadProjectErrorModal } from "./load-project-error-modal";
import { downloadProject, loadProject } from "./project-io";

export default function MainMenu() {
  const navigate = useNavigate();
  const isOnNewProjectRoute = !!(useMatch({ path: "/new" }) ?? false);
  const [confirmNewProjectModalOpen, setConfirmNewProjectModalOpen] =
    useState(false);
  const [loadProjectErrorModalOpen, setLoadProjectErrorModalOpen] =
    useState(false);

  async function handleLoadProject() {
    const result = await loadProject();
    if (result === "error") {
      setLoadProjectErrorModalOpen(true);
    } else if (result === "success") {
      navigate("/");
    }
  }

  function handleNewProject() {
    setConfirmNewProjectModalOpen(true);
  }

  function handleDownloadProject() {
    downloadProject();
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
        <MenuItem onClick={handleLoadProject}>Open Project</MenuItem>
        <MenuItem onClick={handleDownloadProject}>Download</MenuItem>
      </menu>
      <ConfirmNewProjectModal
        open={confirmNewProjectModalOpen}
        onCancel={() => setConfirmNewProjectModalOpen(false)}
        onConfirm={handleConfirmNewProject}
      />
      <LoadProjectErrorModal
        open={loadProjectErrorModalOpen}
        onClose={() => setLoadProjectErrorModalOpen(false)}
      />
    </>
  );
}
