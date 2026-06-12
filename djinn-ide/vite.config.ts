import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { lezer } from "@lezer/generator/rollup";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [tailwindcss(), reactRouter(), wasm(), topLevelAwait(), lezer()],
  resolve: {
    tsconfigPaths: true,
  },
});
