import { Post } from "../../api";

import { FormatDate } from "./FormatDate";
import { Markdown } from "./Markdown";
import { PostMenu } from "./PostMenu";

export const ShowPost = (post: Post) => (
  <div class="relative has-menu">
    <div class="menu absolute top-2 right-2 transition-opacity">
      <PostMenu id={post.id} isDeleted={post.isDeleted} />
    </div>

    <Markdown html={post.content.html} />
    <FormatDate date={post.postedAt} />
  </div>
);
