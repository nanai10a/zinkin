import { ErrorBoundary } from "preact-iso";

import "./app.css.ts";

import Index from "./index";

export const App = () => {
  return (
    <ErrorBoundary>
      <Index />
    </ErrorBoundary>
  );
};

// --- --- --- --- --- --- --- --- ---
import { hydrate } from "preact-iso";

hydrate(<App />);

// --- --- --- --- --- --- --- --- ---
import { prerender as withPreact } from "preact-iso";

type Data = {
  ssr: boolean;
  url: string;
  route: Record<string, string>;
};

export const prerender = (data: Data) => {
  return withPreact(<App />);
};
