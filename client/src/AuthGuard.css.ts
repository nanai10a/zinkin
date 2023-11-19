import { style } from "@vanilla-extract/css";

import { colors, c } from "./styles.css.ts";

// 一括指定プロパティは:
//   --property: all;
//   --property: {top, bottom} {right, left};
//   --property: top {right, left} bottom;
//   --property: top right bottom left;

export const cont = style({
  boxSizing: "border-box",
  display: "flex",
  position: "absolute",

  top: "0",
  left: "0",

  gap: "2rem",
  placeContent: "center",

  padding: ".5rem",
  margin: "0 auto",

  width: "100%",
  height: "100%",

  background: `repeating-linear-gradient(-45deg, ${colors.slate["300"]} 0px 4px, transparent 4px 12px)`,
});

export const butt = style({
  padding: ".5rem 1rem",
  margin: "auto 0",

  height: "fit-content",

  background: colors.slate["300"],

  borderRadius: c.inf,
  border: `2px solid ${colors.slate["400"]}`,

  transition: "background 150ms ease-in-out",

  ":hover": {
    background: colors.slate["400"],
    borderColor: colors.slate["500"],
  },
});
