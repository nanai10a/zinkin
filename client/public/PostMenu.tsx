import { fetchAPI, Post } from "./api";

import { Icon } from "./Icon";
import { posts } from "./posts";

import { cont, butt } from "./PostMenu.css.ts";

const updatePost = (idx: number, val: Post) => {
  const a = posts.value;
  a[idx] = val;

  posts.value = [...a];
};

const setIsDelete = async (id: number, isDeleted: boolean) => {
  const idx = posts.value.findIndex((p) => p.id === id);
  if (idx === -1) {
    return new Error();
  }

  updatePost(idx, await fetchAPI(`/posts/${id}`, "PATCH", { isDeleted }));
};
export const PostMenu = ({ id, isDeleted }: Pick<Post, "id" | "isDeleted">) => (
  <div class={cont}>
    <button
      class={butt}
      onClick={() => setIsDelete(id, true)}
      hidden={isDeleted}
    >
      <Icon icon="material-symbols:delete-outline-rounded" />
    </button>
    <button
      class={butt}
      onClick={() => setIsDelete(id, false)}
      hidden={!isDeleted}
    >
      <Icon icon="material-symbols:settings-backup-restore-rounded" />
    </button>
  </div>
);
