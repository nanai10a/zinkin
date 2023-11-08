import { style } from "@vanilla-extract/css";

import { colors } from "./styles.css.ts";

export const root = style({
  padding: ".5rem",

  width: "100%",

  background: colors.slate[100],

  borderRadius: ".75rem",
  border: `2px solid ${colors.slate[300]}`,

  resize: "none",
});
