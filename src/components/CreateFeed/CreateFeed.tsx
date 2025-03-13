import { createEffect, createMemo, createResource, createSignal, Show, Suspense } from "solid-js";
import style from "./CreateFeed.module.css";
import { Api } from "../../api";
import { FeedPreview } from "../FeedPreview/FeedPreview";
import { ComputedTextInput } from "../ComputedTextInput/ComputedTextInput";
import { createScheduled, debounce } from "@solid-primitives/scheduled";

export const CreateFeed = () => {

  
  const [feedUrl, setFeedUrl] = createSignal("");
  const [feedTitle, setFeedTitle] = createSignal("");
  const [fetchOld, setFetchOld] = createSignal(false);
  
  // update feed preview when URL changes, debounced
  const previewScheduled = createScheduled(fn => debounce(fn, 800));

  const debouncedUrl = createMemo((p: string = "") => {
    // track source signal
    const value = feedUrl();
    // track the debounced signal and check if it's dirty
    return previewScheduled() ? value : p;
  });

  const [feedPreview, setFeedPreview] = createResource(debouncedUrl, async (url: string) => {
    console.log(`url: ${url}`);

    const result = await Api.commands.readFeedTitle({ url });
    if(result.status === "ok") {
      return result.data;
    } else {
      return null;
    }
  })
  
  const feedPreviewTitle = (): string|null => {
    return feedPreview()?.title || null;
  }

  const createFeed = async () => {
    console.log("create feed");
    let result = await Api.commands.createFeed(
      feedTitle(),
      feedUrl(),
      fetchOld()
    );
    
    console.log("result", result);
    
    // TODO: show success notification
    // TODO: clear input fields
  };
  
  return (<div class={style.feedCreate}>
  <h1>Create Feed</h1>
    <form
      onSubmit={async (e) => {
        console.log("submit");
        e.preventDefault();
        await createFeed();
      }}
    >
      {/* url */}
      <div>
      <input
        class="input-url"
        type="text" placeholder="Feed URL"
        value={feedUrl()}
        onInput={(e) => {setFeedUrl(e.currentTarget.value)}}
      />
      </div>
      {/* title */}
      <div>
        <ComputedTextInput
          defaultOptions={[
            { label: "Auto", value: feedPreviewTitle() },
          ]}
          onUpdate={setFeedTitle}
        />
      </div>
      <div>
        <input
          id="fetch_old_items"
          type="checkbox"
          checked={ fetchOld() }
          onChange={(e) => { e.preventDefault(); setFetchOld(e.currentTarget.checked)}}
        />
        <label for="fetch_old_items">Fetch Old Items?</label>
      </div>
      <button type="button">Preview</button>
      <button type="submit">Subscribe</button>
    </form>
    <div>
      <div>Title: {feedTitle()}</div>
      <div>URL: {feedUrl()}</div>
      <div>FetchOld: {fetchOld() ? "Yes" : "No"}</div>
    </div>
    <div>
      <h2>Feed Preview</h2>
      <Show when={feedPreview()}>
          <FeedPreview
            feedTitle={feedPreview()!.title}
            entries={feedPreview()!.entries.map(e => ({
              title: e.title,
              published: e.published,
              url: e.url,
              url_comments: e.url_comments
            }))}
          />
      </Show>
    </div>
  </div>);
}