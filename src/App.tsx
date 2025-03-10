import { createSignal, For, Show } from "solid-js";

import logo from "./assets/logo.svg";
import "./App.css";

// goose
import * as Api from "./api";

////////////////////////////////////////////////////////////////////////////////

function AppDefault() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  return (
    <main class="container">
      <h1>Welcome to Tauri + Solid</h1>

      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://solidjs.com" target="_blank">
          <img src={logo} class="logo solid" alt="Solid logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and Solid logos to learn more.</p>

      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          Api.commands.greet(name());
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg()}</p>
    </main>
  );
}

export const FeedPreview = (props: { preview : Api.FeedPreview}) => {
  return <div class="feed-preview">
    <div>{props.preview.title}</div>
    <div>
      <For each={props.preview.entries}>
        {(entry) => {
          return (<div>{entry.title}</div>);
        }}
      </For>
    </div>
  </div>
}

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
