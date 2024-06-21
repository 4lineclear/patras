import { Route, Switch } from "wouter";
import PageNotFound from "./routes/PageNotFound";
import IndexRouter from "./IndexRouter";
import Header from "./components/Header";

const App = () => (
  <div id="app">
    <Header />
    <Switch>
      <Route path="/" component={IndexRouter} />
      <Route path="*" component={PageNotFound} />
    </Switch>
  </div>
);

export default App
