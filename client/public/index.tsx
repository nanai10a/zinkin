import install from "@twind/with-react";
import inline from "@twind/with-react/inline";
import autoprefix from "@twind/preset-autoprefix";
import tailwind from "@twind/preset-tailwind";

import {
  LocationProvider,
  Router,
  Route,
  lazy,
  ErrorBoundary,
  hydrate,
  prerender as ssr,
} from "preact-iso";
import Home from "./pages/home/index";
import NotFound from "./pages/_404";
import Header from "./header";

const About = lazy(() => import("./pages/about/index"));

const tw = install({
  presets: [tailwind(), autoprefix()],
});

export function App() {
  return (
    <LocationProvider>
      <div class="app">
        <Header />
        <ErrorBoundary>
          <Router>
            <Route path="/" component={Home} />
            <Route path="/about" component={About} />
            <Route default component={NotFound} />
          </Router>
        </ErrorBoundary>
      </div>
    </LocationProvider>
  );
}

hydrate(<App />);

export async function prerender(data: any) {
  return await ssr(<App {...data} />).then((r) => inline(r.html, tw));
}
