import { defineConfig } from "wmr";

import VanillaExtract from "rollup-plugin-vanilla-extract";

// Full list of options: https://wmr.dev/docs/configuration
export default defineConfig({
  /* Your configuration here */
  plugins: [VanillaExtract({ basedir: "public" })],

  alias: {
    react: "preact/compat",
    "react-dom": "preact/compat",
  },
});
