import { Route, Switch } from "wouter";
import PageNotFound from "./routes/PageNotFound";
import Header from "./components/Header";
import Index from "./routes/Index";
import SignUp from "./routes/SignUp";

const App = () => (
  <div id="app">
    <Header />
    <Switch>
      <Route path="/" component={Index} />
      <Route path="/log-in" component={SignUp} />
      <Route path="/sign-up" component={SignUp} />
      <Route path="*" component={PageNotFound} />
    </Switch>
  </div>
);

export default App;
