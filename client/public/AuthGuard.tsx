import { useCallback, useState, useEffect } from "preact/hooks";

import * as auth from "./auth";
import { tw } from "./twind";

// rome-ignore format:
const slashed = `bg-[repeating-linear-gradient(-45deg, ${tw.theme("colors.slate.300")} 0px 4px, transparent 4px 12px)]`.replaceAll(" ", "_");

const authw = {
  status: async () => (await auth.check()).refresh,
  signup: async () => {
    switch (await auth.register()) {
      case "success":
        break;

      case "unauthorized":
        throw new Error("unhandlable error");
    }
  },
  login: async () => {
    switch (await auth.claim()) {
      case "success":
        break;

      case "no credential":
      case "unauthorized":
        throw new Error("unhandlable error");
    }
  },
};

export const AuthGuard = () => {
  const [passed, setPassed] = useState(true);

  useEffect(() => void authw.status().then(setPassed), []);

  const signup = useCallback(async () => {
    if (await authw.status()) {
      return;
    }

    await authw.signup();
    await authw.login();

    setPassed(await authw.status());
  }, [setPassed]);

  const login = useCallback(async () => {
    if (await authw.status()) {
      return;
    }

    await authw.login();

    setPassed(await authw.status());
  }, [setPassed]);

  return (
    <div
      class={`absolute p-2 mx-auto w-full h-full top-0 left-0 ${slashed} flex place-content-center gap-8 ${passed ? "hidden" : ""}`}
    >
      <button
        class="px-4 py-2 my-auto h-fit bg-(slate-300 hover:slate-400) rounded-full border-(2 slate-400 hover:slate-500) transition"
        onClick={signup}
      >
        Sign up
      </button>
      <button
        class="px-4 py-2 my-auto h-fit bg-(slate-300 hover:slate-400) rounded-full border-(2 slate-400 hover:slate-500) transition"
        onClick={login}
      >
        Login
      </button>
    </div>
  );
};
