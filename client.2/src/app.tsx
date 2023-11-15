import "preact/debug";

import { useEffect } from "preact/hooks";

import { fetchAPI } from "./api";
import { posts } from "./posts";

import { AuthGuard } from "./AuthGuard";
import { ShowPost } from "./ShowPost";
import { Submit } from "./Submit";

import { root, cont, list, elem, space, f__k } from "./app.css.ts";
import { apply } from "./styles.css.ts";

export default function () {
  useEffect(() => {
    fetchAPI("/posts", "GET", null).then((res) => {
      posts.value = res;
    });
  }, []);

  return (
    <div class={`${root} ${apply}`}>
      <div class={cont}>
        <ul class={list}>
          {posts.value.map((post) => (
            <>
              <li class={elem}>
                <ShowPost {...post} />
              </li>
              <hr class={space} />
            </>
          ))}
        </ul>

        <div class={f__k}>
          <Submit />
          <AuthGuard />
        </div>
      </div>
    </div>
  );
}
