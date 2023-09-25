import "preact/debug";

import { injectGlobal } from "@twind/core";

import { Icon } from "./Icon";
import { Markdown } from "./Markdown";
import { FormatDate } from "./FormatDate";

import { signal } from "@preact/signals";
import { useMemo, useEffect, useCallback, useState } from "preact/hooks";

const posts = signal<Post[]>([]);

import { fetchAPI, Post } from "../../api";

const PostMenu = ({ id, isDeleted }: Pick<Post, "id" | "isDeleted">) => {
  const deletx = useCallback(async () => {
    const idx = posts.value.findIndex((p) => p.id === id);
    if (idx === -1) {
      return new Error();
    }

    posts.value[idx] = await fetchAPI(`/posts/${id}`, "PATCH", {
      isDeleted: true,
    });
  }, []);

  const restore = useCallback(async () => {
    const idx = posts.value.findIndex((p) => p.id === id);
    if (idx === -1) {
      return new Error();
    }

    posts.value[idx] = await fetchAPI(`/posts/${id}`, "PATCH", {
      isDeleted: false,
    });
  }, []);

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

const ShowPost = (post: Post) => {
  return (
    <div class="relative has-menu">
      <div class="menu absolute top-2 right-2 transition-opacity">
        <PostMenu id={post.id} isDeleted={post.isDeleted} />
      </div>

      <Markdown html={post.content.html} />
      <FormatDate date={post.postedAt} />
    </div>
  );
};

const Submit = () => {
  const [text, setText] = useState("");

  const update = useCallback(
    (e: Event) => setText((e.target as HTMLTextAreaElement).value),
    [setText],
  );

  const submit = useCallback(
    async (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key === "Enter") {
        const res = await fetchAPI("/posts", "POST", { content: text });

        posts.value.unshift(res);
        setText("");
      }
    },
    [text, setText],
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
  useEffect(() => {
    fetchAPI("/posts", "GET", null).then((res) => {
      posts.value = res;
    });
  }, []);

  return (
    <main class="absolute inset-0 w-full h-[100svh]">
      <div class="mx-auto min-w-0 max-w-2xl min-h-0 h-full flex flex-col">
        <ul class="grow w-full flex-(& col-reverse) overflow-y-auto">
          {posts.value.map((post) => (
            <>
              <li class="px-2 py-4">
                <ShowPost {...post} />
              </li>
              <hr class="h-0.5 bg-slate-300 last:hidden" />
            </>
          ))}
        </ul>

        <div class="p-4">
          <Submit />
        </div>
      </div>
    </main>
  );
}
