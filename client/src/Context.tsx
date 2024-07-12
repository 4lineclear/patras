import {
  createContext,
  useState,
  ReactNode,
  useCallback,
  DependencyList,
} from "react";

interface Context {
  auth: Auth;
}

interface Auth {
  /**
   * `true` if user logged in
   */
  login: boolean;
  setLogin: (login: boolean) => void;
}

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
  const [login, setLogin] = useStateCallback(false);

  return (
    <Context.Provider value={{ auth: { login, setLogin } }}>
      {children}
    </Context.Provider>
  );
}
