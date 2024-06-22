import { Route, Switch } from "wouter";
import PageNotFound from "./routes/PageNotFound";
import Header from "./components/Header";
import Index from "./routes/Index";

const App = () => (
  <div id="app">
    <Header />
    <Switch>
      <Route path="/" component={Index} />
      <Route path="*" component={PageNotFound} />
    </Switch>
  </div>
);

export default App
