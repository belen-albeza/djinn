import { useProjectStore } from "~/features/base/project.store";
import { Modal } from "~/ui/modal";
import Button from "~/ui/button";

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
  const header = (
    <>
      <p className="text-label">Discard changes</p>
      <h2 className="text-heading">Start a new project?</h2>
    </>
  );

  return (
    <Modal
      open={open}
      onClose={onCancel}
      header={header}
      variant="destructive"
      className="max-w-md"
    >
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
