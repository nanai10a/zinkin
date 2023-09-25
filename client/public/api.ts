import { useState, useCallback } from "preact/hooks";
import { z } from "zod";

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

const onRoute = (url: string, method: string): Validator => {
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

type req<
  P extends keyof Schema,
  M extends keyof Schema[P],
> = Schema[P][M] extends { req: infer R } ? R : never;

type res<
  P extends keyof Schema,
  M extends keyof Schema[P],
> = Schema[P][M] extends { res: infer R } ? R : never;

const BASE_URL = "http://localhost:9090";

export const useAPI = <
  U extends string & keyof Schema,
  M extends string & keyof Schema[U],
>(
  url: U,
  method: M,
): {
  res: res<U, M> | null;
  loading: boolean;
  fire: (req: req<U, M>) => void;
} => {
  const [res, set] = useState<res<U, M> | null>(null);
  const [loading, resolved] = useState(false);

  const fire = useCallback(
    (req: unknown) => {
      const vld = onRoute(url, method);

      const body = req === null ? null : JSON.stringify(vld.req.parse(req));
      const headers = { "Content-Type": "application/json" };

      resolved(false);

      fetch(new URL(url, BASE_URL), { body, headers, method })
        .then((res) => res.json())
        .then((obj) => vld.res.parse(obj))
        .then((val) => set(val))
        .then(() => resolved(true));
    },
    [set],
  );

  return { res, loading, fire };
};

export const fetchAPI = async <
  U extends string & keyof Schema,
  M extends string & keyof Schema[U],
>(
  url: U,
  method: M,
  obj: req<U, M>,
): Promise<res<U, M>> => {
  const vld = onRoute(url, "POST");

  const body = obj === null ? null : JSON.stringify(vld.req.parse(obj));
  const headers = { "Content-Type": "application/json" };

  return fetch(new URL(url, BASE_URL), { body, headers, method })
    .then((res) => res.json())
    .then((obj) => vld.res.parse(obj));
};
