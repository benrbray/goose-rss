import { For, Show } from "solid-js";
import "./FeedPreview.css";
import { ExternalLink } from "../ExternalLink";
import { Api } from "../../api";
import { Temporal } from "@js-temporal/polyfill";

/** Wrapper around `Temporal.Instant.from()` that catches errors. */
export const dateFrom = (date: string|null): Temporal.ZonedDateTime | null => {
  if(!date) { return null; }
  try {
    let instant = Temporal.Instant.from(date);
    return instant.toZonedDateTimeISO(Temporal.Now.timeZoneId());
  } catch(e) {
    return null;
  }
}

export namespace FeedPreview {
  export type Props = {
    feedTitle: string,
    entries: {
      title: string|null,
      url: string|null,
      url_comments: string|null,
      published: string|null
    }[]
  }
}

export const FeedPreview = (props: FeedPreview.Props) => {
  return <div class="feed-preview">
    <div>{props.feedTitle}</div>
    <div>
      <For each={props.entries}>
        {(entry) => {
          let datePublished = dateFrom(entry.published);

          return (<div class="entry-preview">
            <div class="entry-title">
              <ExternalLink href={entry.url || undefined}>
                {entry.title || entry.url || "<no title>"}
              </ExternalLink>
            </div>
            <div class="entry-info">
              <Show when={datePublished}>
                <span class="date date-published">
                {datePublished!.toLocaleString("en-GB", {
                  year: "numeric",
                  month: "long",
                  day: "numeric"
                })}
                </span>
              </Show>
              |
              <Show when={entry.url_comments}>
                <ExternalLink href={entry.url_comments!}>comments</ExternalLink>
              </Show>
            </div>
          </div>);
        }}
      </For>
    </div>
  </div>
}