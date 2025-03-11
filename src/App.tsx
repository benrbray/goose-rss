import { createSignal, Show } from "solid-js";

import "./App.css";

// goose
import { Api } from "./api";
import { FeedPreview } from "./components/FeedPreview/FeedPreview";

////////////////////////////////////////////////////////////////////////////////

export const Feeds = () => {

  const [linkToCreate, setLinkToCreate] = createSignal("");
  const [feedPreview, setFeedPreview] = createSignal<Api.FeedPreview|null>(null);

  const createFeed = async () => {
    // await feedApi.createFeed({ title: "auto", link: linkToCreate(), fetch_old_items: fetchOldItems() });
    // setFeeds(await feedApi.readAllFeeds());
    // setLinkToCreate("");
  };
  
  return (<>
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
  </>);
}

export const App = () => {
  return (<main>
    <Feeds />
  </main>);
}
