export type SaveFile = (filename: string, blob: Blob) => void;
export type PickFile = () => Promise<string | null>;

export const saveFileToDisk: SaveFile = (filename, blob) => {
  const url = URL.createObjectURL(blob);

  // trigger download by clicking an orphan, temporary link
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();

  // clean up the temporary URL
  URL.revokeObjectURL(url);
};

export const pickFileFromDisk: PickFile = () => {
  return new Promise((resolve) => {
    const fileInput = document.createElement("input");
    fileInput.type = "file";
    fileInput.accept = "application/json";
    fileInput.onchange = (event) => {
      const file = (event.target as HTMLInputElement).files?.[0];
      if (!file) {
        resolve(null);
        return;
      }

      const reader = new FileReader();
      reader.onload = (loadEvent) => {
        const result = loadEvent.target?.result;
        resolve(typeof result === "string" ? result : null);
      };
      reader.onerror = () => resolve(null);
      reader.readAsText(file);
    };
    fileInput.click();
  });
};
