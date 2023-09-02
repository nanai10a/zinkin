import { useCallback, useReducer } from "preact/hooks";
import { z } from "zod";

type Post = z.TypeOf<typeof Post>;
const Post = z.object({
  content: z.string(),
});

const ShowPost = ({ post }: { post: Post }) => {
  return (
    <div class="">
      <p class="">{post.content}</p>
    </div>
  );
};

const Timeline = ({ posts }: { posts: Post[] }) => {
  return (
    <ul class="w-full divide-(y slate-300)">
      {posts.map((post) => (
        <li class="px-2 py-4">
          <ShowPost post={post} />
        </li>
      ))}
    </ul>
  );
};

const assert = (fn: () => boolean): void | never => {
  if (!fn()) {
    throw new Error("Assertion failed: " + fn.toString());
  }
};

const submitReducer = (
  state: Partial<Post> & { callback: (p: Post) => void },
  action:
    | { ty: "update"; v: Partial<Post & { callback: (p: Post) => void }> }
    | { ty: "submit" },
) => {
  switch (action.ty) {
    case "update":
      return { ...state, ...action.v };

    case "submit":
      state.callback(Post.parse(state));
      return { callback: state.callback };
  }
};

const Submit = ({ callback }: { callback: (p: Post) => void }) => {
  const [state, dispatch] = useReducer(submitReducer, { callback });

  if (state.callback !== callback) dispatch({ ty: "update", v: { callback } });

  // rome-ignore format:
  const update = useCallback((e: Event) => {
    if (e.target instanceof HTMLTextAreaElement) {
      dispatch({ ty: "update", v: { content: e.target.value } });
    } else {
      throw new Error("Unexpected event target: " + e.target);
    }
  }, [dispatch]);

  // rome-ignore format:
  const submit = useCallback((e: KeyboardEvent) => {
    if (e.ctrlKey && e.key === "Enter") dispatch({ ty: "submit" });
  }, [dispatch]);

  return (
    <textarea
      class="p-2 w-full bg-slate-100 rounded-xl border-(2 slate-300) resize-none"
      rows={3}
      value={state.content ?? ""}
      onInput={update}
      onKeyDown={submit}
    />
  );
};

export default function Home() {
  const [posts, append] = useReducer<Post[], Post>(
    (posts, additional) => [...posts, additional],
    [
      { content: "こんにちは！" },
      { content: "こんにちは！" },
      { content: "こんにちは！" },
    ],
  );

  return (
    <main class="w-full min-h-screen flex flex-row">
      <div class="basis-3/12">
        <div class="" />
      </div>

      <div class="basis-7/12 flex flex-col">
        <div class="grow overflow-y-auto">
          <Timeline posts={posts} />
        </div>

        <div class="mb-4">
          <Submit callback={append} />
        </div>
      </div>

      <div class="basis-2/12">
        <div class="" />
      </div>
    </main>
  );
}
