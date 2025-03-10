use serde::Deserialize;

use tauri::State;

use crate::error::Error;
use crate::models::feeds::{self, Feed, FeedToCreate};
use crate::DbState;


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

////////////////////////////////////////////////////////////////////////////////

use crate::models::fetch::fetch_content;

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
    Err(e) => {
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
pub fn create_feed(db_state: State<DbState>, data: FeedToCreate) -> Result<String, String> {
  let db = db_state.db.lock().unwrap();

  match feeds::create(&db, &data) {
    Ok(_) => {
      // TODO: immediately fetch feed items
      Ok("new feed added!".to_string())
    }
    Err(err) => {
      Err(err.to_string())
    }
  }
}



#[tauri::command]
#[specta::specta]
pub fn read_all_feeds(db_state: State<DbState>) -> Result<Vec<Feed>, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::read_all(&db) {
        Ok(feeds) => Ok(feeds),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
#[specta::specta]
pub fn read_feed(db_state: State<DbState>, id: i32) -> Result<Option<Feed>, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::read(&db, id) {
        Ok(feed) => Ok(feed),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
#[specta::specta]
pub fn delete_feed(db_state: State<DbState>, id: i32) -> Result<String, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::delete(&db, id) {
        Ok(_) => Ok("Feed deleted".to_string()),
        Err(err) => Err(err.to_string()),
    }
}