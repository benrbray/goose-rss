pub mod commands {
  pub mod feeds;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      commands::feeds::my_custom_command,
      commands::feeds::greet
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
