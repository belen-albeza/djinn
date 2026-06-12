import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { lezer } from "@lezer/generator/rollup";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [tailwindcss(), reactRouter(), wasm(), lezer()],
  resolve: {
    tsconfigPaths: true,
  },
  build: {
    target: "es2022",
  },
});
