import "preact/debug";

import { injectGlobal } from "@twind/core";

import { Icon as _Icon, IconProps } from "@iconify/react";

import { useEffect, useCallback, useState } from "preact/hooks";

import { useAPI, Post } from "../../api";

const Icon = (props: IconProps) => {
  // ignore type error
  const Icon = _Icon as any;

  return <Icon {...props} />;
};

injectGlobal`
  .md-frame {
    & {
      @apply break-words;
    }

    & h1 {
      @apply mt-4 mt-2 text-2xl font-bold;
    }

    & h2 {
      @apply mt-4 mb-2 text-xl font-bold;
    }

    & h3 {
      @apply text-lg font-bold;
    }

    & h4, & h5, & h6 {
      @apply font-bold;
    }

    & a {
      @apply text-blue-500 underline;
    }

    & ul {
      @apply mt-1 mb-2 list-disc list-inside;
    }

    & ol {
      @apply mt-1 mb-2 list-decimal list-inside;
    }

    & li > ul > li,
    & li > ol > li {
      @apply ml-4;
    }

    & blockquote {
      @apply relative pl-4;

      &::before {
        content: "";
        @apply block absolute w-1 h-full bg-slate-300 rounded left-0;
      }
    }

    & code {
      @apply inline-block my-1 px-2 bg-slate-300 rounded font-mono;
    }

    & pre {
      @apply overflow-y-auto;
    }

    & img {
      @apply p-4 max-w-full h-auto;
    }
  }
`;

const fmts: Intl.DateTimeFormatOptions = {
  dateStyle: "medium",
  timeStyle: "medium",
};

const ShowPost = ({ post }: { post: Post }) => {

  return (
    <div>
      <div
        class="md-frame"
        dangerouslySetInnerHTML={{ __html: post.content.html }}
      />
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

injectGlobal`
  .root-layout {
    display: grid;
    grid-template-columns: 3fr 7fr 2fr;
  }
`;

export default function Home() {
  const { fire, res: posts } = useAPI("/posts", "GET");

  const get = useCallback(() => fire(null), [fire]);

  // initially load
  useEffect(get, [get]);

  return (
    <main class="w-full min-h-screen root-layout">
      <div class="">left</div>

      <div class="min-w-0 flex flex-col">
        <div class="grow overflow-x-auto">
          <Timeline posts={posts ?? []} />
        </div>

        <div class="mb-4">
          <Submit reload={get} />
        </div>
      </div>

      <div class="">right</div>
    </main>
  );
}
