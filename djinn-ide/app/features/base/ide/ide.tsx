import Header from "../header";
import StatusBar from "../status-bar";
import WasmProvider from "../wasm-provider";
import { useEmulatorStore } from "~/features/runner/emulator.store";
import { useGlobalShortcuts, globalShortcuts } from "../shortcuts";
import EmulatorView from "~/features/runner/emulator-view";

interface IdeProps {
  children: React.ReactNode;
}

export default function Ide({ children }: IdeProps) {
  const showEmulatorView = useEmulatorStore((state) => state.visible);

  useGlobalShortcuts(globalShortcuts);

  return (
    <WasmProvider>
      <div className="grid h-dvh grid-rows-[auto_1fr_auto]">
        <Header />
        <main className="grid h-full min-h-0 overflow-hidden">
          {children}
          <EmulatorView
            open={showEmulatorView}
            onClose={() => useEmulatorStore.getState().setVisible(false)}
          />
        </main>
        <StatusBar />
      </div>
    </WasmProvider>
  );
}
