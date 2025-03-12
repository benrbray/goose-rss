/* @refresh reload */
import { render } from "solid-js/web";
import { App, FrontPage } from "./App";
import { Route, Router } from "@solidjs/router";
import { ManageFeeds } from "./components/ManageFeeds/ManageFeeds";
import { CreateFeed } from "./components/CreateFeed/CreateFeed";

render(
  () => (<Router root={App}>
    <Route path="/" component={FrontPage} />
    <Route path="/createFeed" component={CreateFeed} />
    <Route path="/manageFeeds" component={ManageFeeds} />
  </Router>),
  document.getElementById("root") as HTMLElement
);
