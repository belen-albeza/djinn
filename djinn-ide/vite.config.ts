import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig, type ViteDevServer } from "vite";
import { lezer } from "@lezer/generator/rollup";
import wasm from "vite-plugin-wasm";

import path from "node:path";

function watchDjinnWasm() {
  const wasmPkg = path.resolve(__dirname, "../packages/djinn-dev-wasm");
  return {
    name: "watch-djinn-wasm",
    configureServer(server: ViteDevServer) {
      server.watcher.add(wasmPkg);
      server.watcher.on("change", (file: string) => {
        if (file.startsWith(wasmPkg)) {
          server.ws.send({ type: "full-reload" });
        }
      });
    },
  };
}

export default defineConfig({
  plugins: [tailwindcss(), reactRouter(), wasm(), lezer(), watchDjinnWasm()],
  optimizeDeps: {
    exclude: ["djinn-dev-wasm"],
  },
  resolve: {
    tsconfigPaths: true,
  },
  build: {
    target: "es2022",
  },
});
