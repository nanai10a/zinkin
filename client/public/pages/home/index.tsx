import "preact/debug";

import { injectGlobal } from "@twind/core";

import { Icon } from "./Icon";
import { Markdown } from "./Markdown";
import { FormatDate } from "./FormatDate";

import { useMemo, useEffect, useCallback, useState } from "preact/hooks";

import { useAPI, Post } from "../../api";

const useObserve = <T,>(
  target: T,
  check: (prev: T, curr: T) => boolean,
  callback: () => void,
) => {
  const [prev, set] = useState<T>();

  if (prev === undefined) {
    return set(target);
  }

  if (check(prev, target)) {
    callback();
    set(undefined);
  }
};

const PostMenu = ({
  reload,
  id,
  isDeleted,
}: { reload: () => void } & Pick<Post, "id" | "isDeleted">) => {
  const { fire, loading } = useAPI(`/posts/${id}`, "PATCH");

  const deletx = useCallback(() => fire({ isDeleted: true }), [fire]);
  const restore = useCallback(() => fire({ isDeleted: false }), [fire]);

  useObserve(loading, (p, c) => !p && c, reload);

  return (
    <div class="w-fit leading-none flex-(& row) gap-2">
      <button
        class="p-2 bg-(slate-200 hover:slate-300) rounded-lg transition"
        onClick={deletx}
      >
        <Icon icon="material-symbols:delete-outline-rounded" />
      </button>
      <button
        class="p-2 bg-(slate-200 hover:slate-300) rounded-lg transition"
        onClick={restore}
        hidden={!isDeleted}
      >
        <Icon icon="material-symbols:settings-backup-restore-rounded" />
      </button>
    </div>
  );
};

injectGlobal`
  .has-menu > .menu {
    @apply opacity-0;
  }

  .has-menu:hover > .menu {
    @apply opacity-100;
  }
`;

const ShowPost = ({ post, reload }: { post: Post; reload: () => void }) => {
  return (
    <div class="relative has-menu">
      <div class="menu absolute top-2 right-2 transition-opacity">
        <PostMenu {...post} reload={reload} />
      </div>

      <Markdown html={post.content.html} />
      <FormatDate date={post.postedAt} />
    </div>
  );
};

const Submit = ({ reload }: { reload: () => void }) => {
  const [text, setText] = useState("");

  const update = useCallback(
    (e: Event) => {
      if (e.target instanceof HTMLTextAreaElement) {
        setText(e.target.value);
      } else {
        throw new Error("Unexpected event target: " + e.target);
      }
    },
    [setText],
  );

  const { fire } = useAPI("/posts", "POST");

  const submit = useCallback(
    (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key === "Enter") {
        setText((text) => {
          fire({ content: text });
          return "";
        });

        reload();
      }
    },
    [fire, setText],
  );

  return (
    <textarea
      class="p-2 w-full bg-slate-100 rounded-xl border-(2 slate-300) resize-none"
      rows={3}
      value={text}
      onInput={update}
      onKeyDown={submit}
    />
  );
};

export default function Home() {
  const { fire, res: posts } = useAPI("/posts", "GET");

  const get = useCallback(() => fire(null), [fire]);

  // initially load
  useEffect(get, [get]);

  return (
    <main class="absolute inset-0 w-full h-[100svh]">
      <div class="mx-auto min-w-0 max-w-2xl min-h-0 h-full flex flex-col">
        <ul class="grow w-full flex-(& col-reverse) overflow-y-auto">
          {posts?.map((post) => (
            <>
              <li class="px-2 py-4">
                <ShowPost post={post} reload={get} />
              </li>
              <hr class="h-0.5 bg-slate-300 last:hidden" />
            </>
          ))}
        </ul>

        <div class="p-4">
          <Submit reload={get} />
        </div>
      </div>
    </main>
  );
}
