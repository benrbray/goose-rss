use tauri_specta::{collect_commands, Builder};
use specta_typescript::Typescript;

pub mod commands {
  pub mod feeds;
}

pub mod error;

pub mod models {
  pub mod fetch;
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
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
