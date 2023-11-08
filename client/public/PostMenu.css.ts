import { style } from "@vanilla-extract/css";

import { colors } from "./styles.css.ts";

export const cont = style({
  display: "flex",

  flexDirection: "row",
  gap: ".5rem",

  width: "fit-content",

  lineHeight: "1",
});

export const butt = style({
  padding: ".5rem",

  lineHeight: "0",

  background: colors.slate[200],

  borderRadius: ".5rem",
  borderWidth: "0",

  transition: "background 150ms ease-in-out",

  ":hover": {
    background: colors.slate[300],
  },
});
