import { createSignal, ParentProps, Show } from "solid-js";
import { Router, Route } from "@solidjs/router"

import "./App.css";

// goose
import { Api } from "./api";
import { FeedPreview } from "./components/FeedPreview/FeedPreview";
import { Navigation } from "./components/Navigation/Navigation";
import { ManageFeeds } from "./components/ManageFeeds/ManageFeeds";

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

export const CreateFeed = () => {

  const [linkToCreate, setLinkToCreate] = createSignal("");
  const [feedPreview, setFeedPreview] = createSignal<Api.FeedPreview|null>(null);

  const createFeed = async () => {
    // await feedApi.createFeed({ title: "auto", link: linkToCreate(), fetch_old_items: fetchOldItems() });
    // setFeeds(await feedApi.readAllFeeds());
    // setLinkToCreate("");
  };
  
  return (<div class="feed-create">
  <h1>Create Feed</h1>
    <form
      onSubmit={async (e) => {
        e.preventDefault();

        const result = await Api.commands.readFeedTitle({ url: linkToCreate() });
        
        if(result.status === "ok") {
          setFeedPreview(result.data);
        } else {
          setFeedPreview(null);
        }
      }}
    >
      <input
        class="input-url"
        type="text" placeholder="Feed URL"
        value={linkToCreate()}
        onInput={(e) => {setLinkToCreate(e.currentTarget.value)}}
      />
      <button type="submit">Subscribe</button>
    </form>
    <div>
      <h2>Feed Preview</h2>
      <Show when={feedPreview() !== null}>
        <FeedPreview preview={feedPreview()!} />
      </Show>
    </div>
  </div>);
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
