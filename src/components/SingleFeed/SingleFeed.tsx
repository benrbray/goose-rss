import { useParams } from "@solidjs/router";
import style from "./SingleFeed.module.css";
import { Api } from "../../api";
import { createResource, For, Switch, Match, Show } from "solid-js";
import { isErr, isOk, matches } from "../../util";
import { FeedPreview } from "../FeedPreview/FeedPreview";

export const SingleFeed = () => {
  {/* TODO: well-typed path parameters? */}
  const pathParams = useParams<{ feedId: string }>();

  let [entriesResult] = createResource(async () => {
    return await Api.commands.readFeedEntries(+pathParams.feedId);
  });

  return <div>
    <Show when={entriesResult()} fallback={<div>Loading Feeds...</div>}>
    <Switch>
    <Match when={matches(entriesResult()!, isOk)}>
        {(entries) => 
          <FeedPreview
            feedTitle={"Feed Preview"}
            entries={entries().data.map(e => ({
              title: e.title,
              url: e.url,
              url_comments: e.url_comments,
              published: e.published
            }))}
          />
        }
      </Match>
      <Match when={matches(entriesResult()!, isErr)}>
        {(entries) => <div>{entries().error}</div>}
      </Match>
    </Switch>
    </Show>
  </div>
};