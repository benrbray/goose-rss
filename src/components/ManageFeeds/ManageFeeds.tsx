import { createResource, Show, Switch, Match, Accessor, JSX, For } from "solid-js";
import { Api } from "../../api";
import "./ManageFeeds.css";
import { A } from "@solidjs/router";
import { isErr, isOk, matches } from "../../util";

////////////////////////////////////////////////////////////////////////////////

export const ManageFeeds = () => {
  let [feeds, setFeeds] = createResource(async () => {
    let result = await Api.commands.readAllFeeds();
    return result;
  });
  
  return <div>
    <h1>Manage Feeds</h1>
    <Show when={feeds()}>
      <Switch fallback={<div></div>}>
        <Match when={matches(feeds()!, isOk)}>
          {(result) => {
            return <For each={result().data}>
              {(feed) => (<div>
                <A href={`/feeds/${feed.id}`}>{feed.title}</A>
                <span>({feed.url}) {feed.checked_at}</span>
              </div>)}
              </For>;
          }}
        </Match>
        <Match when={matches(feeds()!, isErr)}>
          {(result) => {
            return <div>{result().error}</div>
          }}
        </Match>
      </Switch>
    </Show>
  </div>;
}