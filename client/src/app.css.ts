import { style } from "@vanilla-extract/css";

import { colors } from "./styles.css.ts";

export const root = style({
  position: "absolute",
  inset: "0",

  width: "100%",
  height: "100svh", // `svh`, あんま体験良くないかも

  fontSize: "1rem",
  lineHeight: "1.5rem",
});

export const cont = style({
  display: "flex",

  flexDirection: "column",

  margin: "0 auto",

  minWidth: "0",
  maxWidth: "42rem",
  height: "100%",
  minHeight: "0",
});

export const list = style({
  display: "flex",
  overflowY: "auto",

  flexDirection: "column-reverse",
  flexGrow: "1",

  width: "100%",
});

export const elem = style({
  padding: "1rem .5rem",
});

export const space = style({
  height: ".5rem",

  background: colors.slate[300],

  ":last-child": {
    display: "hidden",
  },
});

export const f__k = style({
  position: "relative",

  margin: "1rem",
});
