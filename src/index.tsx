/* @refresh reload */
import { render } from "solid-js/web";
import { App, FrontPage } from "./App";
import { Route, Router } from "@solidjs/router";
import { ManageFeeds } from "./components/ManageFeeds/ManageFeeds";
import { CreateFeed } from "./components/CreateFeed/CreateFeed";
import { SingleFeed } from "./components/SingleFeed/SingleFeed";

render(
  () => (<Router root={App}>
    <Route path="/" component={FrontPage} />
    <Route path="/createFeed" component={CreateFeed} />
    <Route path="/manageFeeds" component={ManageFeeds} />
    <Route path="/feeds/:feedId" component={SingleFeed} matchFilters={{
      feedId: /^\d+$/, // only allow numbers
    }}/>
  </Router>),
  document.getElementById("root") as HTMLElement
);
