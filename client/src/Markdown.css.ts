import { style, globalStyle } from "@vanilla-extract/css";

import { colors } from "./styles.css.ts";

export const frame = style({
  overflowWrap: "break-word",
});

globalStyle(`${frame} h1`, {
  margin: "1rem 0 .5rem",

  fontSize: "1.5rem",
  fontWeight: "bold",
  lineHeight: "2rem",
});

globalStyle(`${frame} h2`, {
  margin: "1rem 0 .5rem",

  fontSize: "1.25rem",
  fontWeight: "bold",
  lineHeight: "1.75rem",
});

globalStyle(`${frame} h3`, {
  fontSize: "1.125rem",
  fontWeight: "bold",
  lineHeight: "1.75rem",
});

globalStyle(`${frame} h4, ${frame} h5, ${frame} h6`, {
  fontWeight: "bold",
});

globalStyle(`${frame} a`, {
  color: colors.blue[500],
  textDecorationLine: "underline",
});

globalStyle(`${frame} ul`, {
  margin: ".25rem 0 .5rem",

  listStyle: "decimal inside",
});

globalStyle(`${frame} ol`, {
  margin: ".25rem 0 .5rem",

  listStyle: "disc inside",
});

globalStyle(`${frame} li > ul > li, ${frame} li > ul > li`, {
  marginLeft: "1rem",
});

globalStyle(`${frame} blockquote`, {
  position: "relative",

  paddingLeft: "1rem",
});

globalStyle(`${frame} blockquote::before`, {
  display: "block",
  position: "absolute",

  left: "0",

  width: ".25rem",
  height: "100%",

  content: "", // `''`?

  background: colors.slate[300],

  borderRadius: ".25rem",
});

globalStyle(`${frame} code`, {
  display: "inline-block",

  margin: ".25rem 0",
  padding: "0 .5rem",

  fontFamily:
    "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace",

  background: colors.slate[300],

  borderRadius: ".25rem",
});

globalStyle(`${frame} pre`, {
  overflowY: "auto",
});

globalStyle(`${frame} img`, {
  boxSizing: "border-box",

  padding: "1rem",

  maxWidth: "100%",
  height: "auto",
});

globalStyle(`${frame} hr`, {
  margin: "1rem",
});
