import { Route, Switch } from "wouter";
import PageNotFound from "./routes/PageNotFound";
import Header from "./components/Header";
import Index from "./routes/Index";
import SignUp from "./routes/SignUp";
import { ContextProvider } from "./Context";
import LogIn from "./routes/LogIn";

export default function App() {
  return (
    <div id="app">
      <ContextProvider>
        <Header />
        <Switch>
          <Route path="/" component={Index} />
          <Route path="/log-in" component={LogIn} />
          <Route path="/sign-up" component={SignUp} />
          <Route path="*" component={PageNotFound} />
        </Switch>
      </ContextProvider>
    </div>
  );
}
