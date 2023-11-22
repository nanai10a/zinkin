import { z } from "zod";
import * as auth from "./auth";

export interface Post extends z.TypeOf<typeof Post> {}
export const Post = z.object({
  id: z.number(),
  content: z.object({
    src: z.string(),
    html: z.string(),
  }),
  postedAt: z.date({ coerce: true }),
  createdAt: z.date({ coerce: true }),
  isDeleted: z.boolean(),
});

type Schema = {
  "/posts": {
    GET: {
      req: null;
      res: Post[];
    };
    POST: {
      req: { content: string };
      res: Post;
    };
  };
  [_: `/posts/${number}`]: {
    GET: {
      req: null;
      res: Post | null;
    };
    PATCH: {
      req: { content: string } | { isDeleted: boolean };
      res: Post;
    };
  };
};

type Validator = { req: z.ZodSchema; res: z.ZodSchema };

const getValidator = (url: string, method: string): Validator => {
  const routes = [
    [
      /\/posts/,
      {
        GET: {
          req: z.null(),
          res: Post.array(),
        } /* satisfies Validator */,
        POST: {
          req: z.object({ content: z.string() }),
          res: Post,
        } /* satisfies Validator */,
      },
    ],
    [
      /\/posts\/\d+/,
      {
        GET: {
          req: z.null(),
          res: Post.nullable(),
        } /* satisfies Validator */,
        PATCH: {
          req: z.union([
            z.object({ content: z.string() }),
            z.object({ isDeleted: z.boolean() }),
          ]),
          res: Post,
        } /* satisfies Validator */,
      },
    ],
  ] as const;

  for (const [patt, branch] of routes) {
    if (patt.test(url) && method in branch) {
      return branch[method as keyof typeof branch];
    }
  }

  throw new Error("unknown route");
};

namespace vld {
  export type req<
    P extends keyof Schema,
    M extends keyof Schema[P],
  > = Schema[P][M] extends { req: infer R } ? R : never;

  export type res<
    P extends keyof Schema,
    M extends keyof Schema[P],
  > = Schema[P][M] extends { res: infer R } ? R : never;
}

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const fetchAPI = async <
  U extends string & keyof Schema,
  M extends string & keyof Schema[U],
>(
  url: U,
  method: M,
  obj: vld.req<U, M>,
): Promise<vld.res<U, M>> => {
  await auth.refresh();

  const vld = getValidator(url, method);

  const json = JSON.stringify(vld.req.parse(obj));
  const body = json === "null" ? null : json;

  const headers = { "Content-Type": "application/json" };

  const res = await fetch(BASE_URL + url, { body, headers, method });
  switch (res.status) {
    case 200:
      return vld.res.parse(await res.json());

    case 400:
      throw new Error(await res.text());

    case 401:
      throw new Error("unauthorized");

    default:
      throw new Error("unknown status");
  }
};
