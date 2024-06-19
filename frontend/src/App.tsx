import { Route, Switch } from "wouter";
import PageNotFound from "./routes/PageNotFound";
import IndexRouter from "./IndexRouter";
import Header from "./components/Header";

const App = () => (
  <>
    <Header />
    <Switch>
      <Route path="/" component={IndexRouter} />
      <Route path="*" component={PageNotFound} />
    </Switch>
  </>
);

export default App
