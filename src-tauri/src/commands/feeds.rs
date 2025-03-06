use serde::Deserialize;
use syndication::Feed;

use crate::error::Error;


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
pub struct FeedToRead {
  pub url: String,
}

#[tauri::command]
#[specta::specta]
pub fn read_feed(data: FeedToRead) -> Result<String, String> {
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
  match html_content.parse::<Feed>() {
    Err(e) => {
      return Err(Error::InvalidFeedLink(data.url).to_string());
    }
    Ok(feed) => {
      let title = match feed {
        Feed::Atom(atom) => { atom.title().to_string() }
        Feed::RSS(rss) =>   { rss.title().to_string()  }
      };

      return Ok(title);
    }
  }
}