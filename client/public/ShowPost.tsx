import { injectGlobal } from "@twind/core";

import { Post } from "./api";

import { FormatDate } from "./FormatDate";
import { Markdown } from "./Markdown";
import { PostMenu } from "./PostMenu";

injectGlobal`
  .has-menu > .menu {
    @apply opacity-0;
  }

  .has-menu:hover > .menu {
    @apply opacity-100;
  }

  .menu {
    @apply transition-opacity;
  }
`;

export const ShowPost = (post: Post) => (
  <div class="relative has-menu">
    <div class="menu absolute top-2 right-2">
      <PostMenu id={post.id} isDeleted={post.isDeleted} />
    </div>

    <Markdown html={post.content.html} />
    <FormatDate date={post.postedAt} />
  </div>
);
