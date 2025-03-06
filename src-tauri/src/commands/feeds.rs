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

