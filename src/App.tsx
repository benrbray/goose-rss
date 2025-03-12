import { createSignal, ParentProps, Show } from "solid-js";
import { Router, Route } from "@solidjs/router"

import "./App.css";

// goose
import { Api } from "./api";
import { FeedPreview } from "./components/FeedPreview/FeedPreview";
import { Navigation } from "./components/Navigation/Navigation";

////////////////////////////////////////////////////////////////////////////////

export const Content = (props: ParentProps) => {
  return <div class="content">
    {props.children}
  </div>
}

export const FrontPage = () => {
  return <div>
    Front Page
  </div>
}

export const App = (props: ParentProps) => {
  // content is determined by a SolidJS Router,
  // passed in through the children prop
  return (<div class="container">
    <Navigation />
    <Content>
      {props.children}
    </Content>
  </div>);
}
