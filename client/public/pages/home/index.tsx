import "preact/debug";

import { useAPI, Post } from "../../api";
import { useEffect, useCallback, useState } from "preact/hooks";

const fmts: Intl.DateTimeFormatOptions = {
  dateStyle: "medium",
  timeStyle: "medium",
};

const ShowPost = ({ post }: { post: Post }) => {
  return (
    <div class="">
      <p class="">{post.content}</p>
      <time
        class="block mt-4 opacity-50 text-right"
        dateTime={post.postedAt.toISOString()}
      >
        {post.postedAt.toLocaleString("ja-JP", fmts)}
      </time>
    </div>
  );
};

const Timeline = ({ posts }: { posts: Post[] }) => {
  return (
    <ul class="w-full divide-(y slate-300)">
      {posts.toReversed().map((post) => (
        <li class="px-2 py-4">
          <ShowPost post={post} />
        </li>
      ))}
    </ul>
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

  const [post, loading, fire] = useAPI("/posts", "POST");

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
  const [posts, loading, fire] = useAPI("/posts", "GET");

  const get = useCallback(() => fire(null), [fire]);

  // initially load
  useEffect(get, [get]);

  return (
    <main class="w-full min-h-screen flex flex-row">
      <div class="basis-3/12">
        <div class="" />
      </div>

      <div class="basis-7/12 flex flex-col">
        <div class="grow overflow-y-auto">
          <Timeline posts={posts ?? []} />
        </div>

        <div class="mb-4">
          <Submit reload={get} />
        </div>
      </div>

      <div class="basis-2/12">
        <div class="" />
      </div>
    </main>
  );
}
