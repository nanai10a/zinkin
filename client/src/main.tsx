import { render } from "preact";

import App from "./app.tsx";

import "reset-css/reset.css";
import "./main.css.ts";

render(<App />, document.getElementsByTagName("main").item(0)!);
