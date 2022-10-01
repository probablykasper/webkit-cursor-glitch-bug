#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{WindowBuilder, WindowUrl};

fn main() {
  let ctx = tauri::generate_context!();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![])
    .setup(|app| {
      let _window = WindowBuilder::new(app, "main", WindowUrl::default())
        .title("Tauri Template")
        .inner_size(800.0, 550.0)
        .min_inner_size(400.0, 200.0)
        .build()
        .expect("Unable to create window");
      Ok(())
    })
    .run(ctx)
    .expect("error while running tauri application");
}
