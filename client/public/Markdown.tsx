import { frame } from "./Markdown.css.ts";

export const Markdown = ({ html: __html }: { html: string }) => (
  <div class={frame} dangerouslySetInnerHTML={{ __html }} />
);
