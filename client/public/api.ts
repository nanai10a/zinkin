import { useState, useCallback } from "preact/hooks";
import { z } from "zod";

export interface Post extends z.TypeOf<typeof Post> {}
export const Post = z.object({
  id: z.number(),
  content: z.string(),
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

const schema = (key: string): any => {
  const routes = [
    [
      /\/posts/,
      {
        GET: {
          req: z.null(),
          res: Post.array(),
        },
        POST: {
          req: z.object({ content: z.string() }),
          res: Post,
        },
      },
    ],
    [
      /\/posts\/\d+/,
      {
        GET: {
          req: z.null(),
          res: Post.nullable(),
        },
        PATCH: {
          req: z.union([
            z.object({ content: z.string() }),
            z.object({ isDeleted: z.boolean() }),
          ]),
        },
      },
    ],
  ] as const;

  for (const [patt, methods] of routes) {
    if (patt.test(key)) return methods;
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
): [
  res: res<U, M> | null,
  loading: boolean,
  fire: (req: req<U, M>) => void,
] => {
  const [val, set] = useState<res<U, M> | null>(null);
  const [status, resolved] = useState(false);

  const fire = useCallback(
    (req: unknown) => {
      const v = schema(url)?.[method];
      if (typeof v !== "object") throw new Error("cannot retrieve validator");

      const body = req === null ? null : JSON.stringify(v.req.parse(req));
      const headers = { "content-type": "application/json" };

      resolved(false);

      fetch(new URL(url, BASE_URL), { body, headers, method })
        .then((res) => res.json())
        .then((obj) => v.res.parse(obj))
        .then((val) => set(val))
        .then(() => resolved(true));
    },
    [set],
  );

  return [val, status, fire];
};
