use std::{fs, path::PathBuf, sync::Mutex};

use rusqlite::Connection;
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};
use specta_typescript::Typescript;

pub mod commands {
  pub mod feeds;
}

////////////////////////////////////////////////////////////////////////////////

pub mod error;

pub mod models {
  pub mod database;
  pub mod feeds;
  pub mod fetch;
}

////////////////////////////////////////////////////////////////////////////////

pub struct DbState {
  db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // tauri_specta builder
  let builder = Builder::<tauri::Wry>::new()
    .commands(collect_commands![
      commands::feeds::my_custom_command,
      commands::feeds::greet,
      commands::feeds::read_feed
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
      let db = models::database::open_connection(&app_data_dir).unwrap();
      let _ = models::database::migrate(&db);

      app.manage(DbState { db: Mutex::new(db) });

      // TODO: worker for polling RSS
      // worker::start(app, &app_data_dir);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
