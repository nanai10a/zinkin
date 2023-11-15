import { defineConfig } from "vite";

import Preact from "@preact/preset-vite";
import { vanillaExtractPlugin as VanillaExtract } from "@vanilla-extract/vite-plugin";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [Preact(), VanillaExtract()],
});
