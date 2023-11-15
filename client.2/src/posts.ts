import { signal } from "@preact/signals";

import { Post } from "./api";

export const posts = signal<Post[]>([]);
