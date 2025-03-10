use std::{fs, path::PathBuf, sync::Mutex};

use tauri::Manager;
use tauri_specta::{collect_commands, Builder};
use specta_typescript::Typescript;

use diesel::prelude::*;


////////////////////////////////////////////////////////////////////////////////

pub mod error;

pub mod commands {
  pub mod feeds;
}


pub mod models {
  pub mod database;
  pub mod feeds;
  pub mod fetch;
}

pub mod schema;

////////////////////////////////////////////////////////////////////////////////

pub struct DbState {
  db: Mutex<SqliteConnection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // tauri_specta builder
  let builder = Builder::<tauri::Wry>::new()
    .commands(collect_commands![
      commands::feeds::my_custom_command,
      commands::feeds::greet,
      commands::feeds::read_feed_title,
      commands::feeds::create_feed,
      commands::feeds::read_all_feeds,
    ]);

  // export typescript bindings (non-release builds only)
  #[cfg(debug_assertions)]
  builder
    .export(Typescript::default(), "../src/api/index.ts")
    .expect("Failed to export typescript bindings");

  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(builder.invoke_handler())
    .setup(move |app| {
      // mount events defined by tauri_specta onto tauri app
      builder.mount_events(app);

      // app data directory
      let app_data_dir = if cfg!(dev) {
        PathBuf::from("data")
      } else {
        app.handle().path().app_data_dir().unwrap()
      };

      // create app data directory
      fs::create_dir_all(&app_data_dir).unwrap();

      // open the database connection, applying any pending migrations
      let mut db = models::database::open_connection(&app_data_dir);
      models::database::run_migrations(&mut db).expect("error running db migrations");
      
      // store the database connection in tauri app state
      app.manage(DbState { db: Mutex::new(db) });

      // start worker for periodically polling RSS feeds
      // worker::start(app, &app_data_dir);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
