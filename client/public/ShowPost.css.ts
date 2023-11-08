import { style, globalStyle } from "@vanilla-extract/css";

export const cont = style({
  position: "relative",
});

export const menu = style({
  position: "absolute",
  top: ".5rem",
  right: ".5rem",

  transition: "opacity 150ms ease-in-out",

  selectors: {
    [`${cont} > &`]: {
      opacity: "0",
    },

    [`${cont}:hover > &`]: {
      opacity: "100%",
    },
  },
});

export const date = style({
  marginTop: "1rem",
});

globalStyle(`${date} > *`, {
  marginLeft: "auto",
});
