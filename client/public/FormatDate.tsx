import { useMemo } from "preact/hooks";

import { root } from "./FormatDate.css.ts";

const opts = {
  dateStyle: "medium",
  timeStyle: "medium",
} as const;

export const FormatDate = ({ date }: { date: Date }) => {
  const dateTime = useMemo(() => date.toISOString(), [date]);
  const content = useMemo(() => date.toLocaleString("ja-JP", opts), [date]);

  return (
    <time class={root} dateTime={dateTime}>
      {content}
    </time>
  );
};
