import { useCallback, useState, useEffect } from "preact/hooks";

import * as auth from "./auth";

import { cont, butt } from "./AuthGuard.css.ts";

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

  if (passed) {
    return <></>;
  }

  return (
    <div class={cont} {...(passed ? { hidden: true } : {})}>
      <button class={butt} onClick={signup}>
        Sign up
      </button>
      <button class={butt} onClick={login}>
        Login
      </button>
    </div>
  );
};
