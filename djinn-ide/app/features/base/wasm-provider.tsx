import { useEffect } from "react";
import { init as initWasm } from "djinn-dev-wasm";

export default function WasmProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  useEffect(() => {
    initWasm();
  }, []);

  return <>{children}</>;
}
