import install from "@twind/with-react";
import autoprefix from "@twind/preset-autoprefix";
import tailwind from "@twind/preset-tailwind";

export const tw = install({
  presets: [tailwind(), autoprefix()],
});
