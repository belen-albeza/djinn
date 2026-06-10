import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { lezer } from "@lezer/generator/rollup";

export default defineConfig({
  plugins: [tailwindcss(), reactRouter(), lezer()],
  resolve: {
    tsconfigPaths: true,
  },
});
