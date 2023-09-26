import { lazy, hydrate, prerender as ssr, ErrorBoundary } from "preact-iso";

import inline from "@twind/with-react/inline";
import { tw } from "./twind";

const Index = lazy(() => import("./index"));

export const App = () => {
  return (
    <ErrorBoundary>
      <Index />
    </ErrorBoundary>
  );
};

hydrate(<App />);

export const prerender = async (data: object) => {
  return await ssr(<App {...data} />).then((r) => inline(r.html, tw));
};
