import "preact/debug";

import { useEffect } from "preact/hooks";

import { fetchAPI } from "./api";
import { posts } from "./posts";

import { AuthGuard } from "./AuthGuard";
import { ShowPost } from "./ShowPost";
import { Submit } from "./Submit";

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

        <div class="relative p-4">
          <Submit />
          <AuthGuard />
        </div>
      </div>
    </main>
  );
}
