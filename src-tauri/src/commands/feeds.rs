use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use tauri::State;
use diesel::prelude::*;
use diligent_date_parser::parse_date;
use diligent_date_parser::chrono::offset::FixedOffset;

use crate::error::Error;
use crate::models::feeds::{CreateFeed, Feed, FeedStatus};
use crate::models::fetch::fetch_content;
use crate::DbState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Note:  Commands will run on the main thread by default, unless marked async.
// The UI also runs on the main thread, so any blocking by commands in the main
// thread will also block the UI.
//
// As a general rule, commands should probably be async by default, unless
// there is a specific reason to run them on the main thread.
//
// See [this thread](https://github.com/tauri-apps/tauri/discussions/3561).

////////////////////////////////////////////////////////////////////////////////


#[tauri::command]
#[specta::specta]
pub fn my_custom_command() {
  println!("I was invoked from JavaScript!");
}

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

////////////////////////////////////////////////////////////////////////////////

#[derive(specta::Type)]
#[derive(Deserialize)]
pub struct FeedInfo {
  pub url: String
}

#[derive(specta::Type)]
#[derive(Serialize)]
pub struct EntryPreview {
  pub title: Option<String>,
  pub url: Option<String>,
  pub url_comments: Option<String>,
  pub published: Option<DateTime<FixedOffset>>
}

#[derive(specta::Type)]
#[derive(Serialize)]
pub struct FeedPreview {
  pub title: String,
  pub entries: Vec<EntryPreview>
}

#[tauri::command]
#[specta::specta]
pub async fn read_feed_title(data: FeedInfo) -> Result<FeedPreview, String> {
  // validate url
  if data.url.is_empty() {
    return Err(Error::EmptyString.to_string())
  }

  // fetch feed contents
  let html_content = fetch_content(&data.url).await.unwrap_or_else(|e| {
    e.to_string()
  });

  // TODO: if it's not a feed, attempt to recover gracefully by
  // looking for feeds at <url>/atom.xml or <url>/rss.xml
  match html_content.parse::<syndication::Feed>() {
    Err(_) => {
      return Err(Error::InvalidFeedLink(data.url).to_string());
    }
    Ok(feed) => {
      let result = match feed {
        syndication::Feed::Atom(atom) => FeedPreview {
          title: atom.title().to_string(),
          entries: atom.entries().to_vec().iter().map(|entry| {
            EntryPreview {
              title: Some(entry.title().to_string()),
              url: Some(entry.id().to_string()),
              url_comments: None,
              // TODO: if no published date, use updated date
              // see https://dicioccio.fr/atom.xml
              published: entry.published().and_then(parse_date)
            }
          }).collect()
        },
        syndication::Feed::RSS(rss) => FeedPreview {
          title: rss.title().to_string(),
          entries: rss.items().to_vec().iter().map(|item| {
            EntryPreview {
              title: item.title().map(|t| t.to_string()),
              // TODO: guid
              // TODO: <comments> as in https://lobste.rs/rss
              url: item.link().map(|t| t.to_string()),
              url_comments: item.comments().map(|t| t.to_string()),
              published: item.pub_date().and_then(parse_date)
            }
          }).collect()
        }
      };

      return Ok(result);
    }
  }
}

#[tauri::command]
#[specta::specta]
pub fn create_feed(
  db_state: State<DbState>,
  title: String,
  url: String,
  fetch_old_items: bool,
) -> Result<String, String> {
  use crate::schema::feeds;

  println!("create_feed");

  let mut db = db_state.db.lock().unwrap();

  let result = diesel::insert_into(feeds::table)
    .values(CreateFeed {
      title,
      url,
      fetch_old_items,
      status: FeedStatus::Subscribed,
      checked_at: NaiveDateTime::MIN,
    })
    .returning(Feed::as_returning())
    .get_result(&mut *db);

  match result {
    Ok(_)    => { return Ok("new feed added!".to_string()); }
    Err(err) => { return Err(err.to_string());              }
  }
}

#[tauri::command]
#[specta::specta]
pub fn read_all_feeds(db_state: State<DbState>) -> Result<Vec<Feed>, String> {
  use crate::schema::feeds::dsl::*;

  let mut db = db_state.db.lock().unwrap();

  let results = feeds
    // .filter(published.eq(true))
    // .limit(5)
    .select(Feed::as_select())
    .load(&mut *db);

  match results {
    Err(err) => Err(err.to_string()),
    Ok(fs) => Ok(fs)
  }
}

// #[tauri::command]
// #[specta::specta]
// pub fn read_feed(db_state: State<DbState>, id: i32) -> Result<Option<Feed>, String> {
//     let db = db_state.db.lock().unwrap();
//     match feeds::read(&db, id) {
//         Ok(feed) => Ok(feed),
//         Err(err) => Err(err.to_string()),
//     }
// }

// #[tauri::command]
// #[specta::specta]
// pub fn delete_feed(db_state: State<DbState>, id: i32) -> Result<String, String> {
//     let db = db_state.db.lock().unwrap();
//     match feeds::delete(&db, id) {
//         Ok(_) => Ok("Feed deleted".to_string()),
//         Err(err) => Err(err.to_string()),
//     }
// }