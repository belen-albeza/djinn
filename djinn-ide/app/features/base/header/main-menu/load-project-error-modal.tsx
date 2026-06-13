import { Modal } from "~/ui/modal";
import Button from "~/ui/button";

export function LoadProjectErrorModal({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  const header = (
    <>
      <p className="text-label">Oh, no!</p>
      <h2 className="text-heading">Failed to load project</h2>
    </>
  );
  return (
    <Modal
      open={open}
      header={header}
      onClose={onClose}
      className="min-w-md"
      variant="error"
    >
      <p className="text-body">The project file is invalid.</p>
      <footer className="flex flex-row gap-2 justify-end">
        <Button variant="secondary" onClick={onClose}>
          Got it
        </Button>
      </footer>
    </Modal>
  );
}
