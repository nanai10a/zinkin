import { Post } from "./api";

import { cont, menu, date } from "./ShowPost.css.ts";

import { FormatDate } from "./FormatDate";
import { Markdown } from "./Markdown";
import { PostMenu } from "./PostMenu";

export const ShowPost = (post: Post) => (
  <div class={cont}>
    <div class={menu}>
      <PostMenu id={post.id} isDeleted={post.isDeleted} />
    </div>

    <Markdown html={post.content.html} />
    <p class={date}>
      <FormatDate date={post.postedAt} />
    </p>
  </div>
);
