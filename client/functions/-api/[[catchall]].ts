/// <reference types="@cloudflare/workers-types" />

type Env = {
  CF_API_BASE_URL: string;
};

export const onRequest = async (ctx: EventContext<Env, string, {}>) => {
  const subpaths = ctx.params["catchall"];
  if (!Array.isArray(subpaths)) {
    return new Response(null, { status: 500 });
  }

  const url = new URL(ctx.env["CF_API_BASE_URL"] + subpaths.join("/"));

  const { method, headers, body } = ctx.request;
  return fetch(url, { method, headers, body });
};
