use serde::Deserialize;

use tauri::State;
use diesel::prelude::*;

use crate::error::Error;
use crate::models::feeds::{CreateFeed, Feed};
use crate::models::fetch::fetch_content;
use crate::DbState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

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

#[tauri::command]
#[specta::specta]
pub fn read_feed_title(data: FeedInfo) -> Result<String, String> {
  // validate url
  if data.url.is_empty() {
    return Err(Error::EmptyString.to_string())
  }

  // fetch feed contents
  let html_content = fetch_content(&data.url).unwrap_or_else(|e| {
    e.to_string()
  });

  // TODO: if it's not a feed, attempt to recover gracefully by
  // looking for feeds at <url>/atom.xml or <url>/rss.xml
  match html_content.parse::<syndication::Feed>() {
    Err(_) => {
      return Err(Error::InvalidFeedLink(data.url).to_string());
    }
    Ok(feed) => {
      let title = match feed {
        syndication::Feed::Atom(atom) => { atom.title().to_string() }
        syndication::Feed::RSS(rss) =>   { rss.title().to_string()  }
      };

      return Ok(title);
    }
  }
}

#[tauri::command]
#[specta::specta]
pub fn create_feed(db_state: State<DbState>, data: CreateFeed) -> Result<String, String> {
  use crate::schema::feeds;

  let mut db = db_state.db.lock().unwrap();

  let result = diesel::insert_into(feeds::table)
    .values(&data)
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