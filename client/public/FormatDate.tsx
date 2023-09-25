import { useMemo } from "preact/hooks";

const opts = {
  dateStyle: "medium",
  timeStyle: "medium",
} as const;

export const FormatDate = ({ date }: { date: Date }) => {
  const dateTime = useMemo(() => date.toISOString(), [date]);
  const content = useMemo(() => date.toLocaleString("ja-JP", opts), [date]);

  return (
    <time class="block mt-4 opacity-50 text-right" dateTime={dateTime}>
      {content}
    </time>
  );
};
