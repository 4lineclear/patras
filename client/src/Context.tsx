import {
  createContext,
  useState,
  ReactNode,
  useCallback,
  DependencyList,
  useEffect
} from "react";

interface Context {
  auth: Auth;
}

interface Auth {
  login: Login;
  setLogin: (login: Login) => void;
}

type Login = "logged-in" | "logged-out" | "loading";

function useStateCallback<S>(
  initialState: S | (() => S),
  callback?: (state: S) => void,
  deps?: DependencyList,
): [S, (state: S) => void] {
  if (!callback) {
    callback = (state: S) => {
      doSetState(state);
    };
  }
  const [state, doSetState] = useState(initialState);
  const setState = useCallback(callback, [callback, ...(!deps ? [] : deps)]);
  return [state, setState];
}

export const Context = createContext<Context>(null!);

export function ContextProvider({ children }: { children: ReactNode }) {
  const [login, setLogin] = useStateCallback<Login>("loading");

  useEffect(() => {
    fetch("/api/check-login", { method: "GET" }).then((res) => {
      switch (res.status) {
        case 200:
          setLogin("logged-in");
          break;
        case 401:
          setLogin("logged-out");
          break;
        default:
          setLogin("logged-out");
          break;
      }
    });
  }, [setLogin])

  return (
    <Context.Provider value={{ auth: { login, setLogin } }}>
      {children}
    </Context.Provider>
  );
}
