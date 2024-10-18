#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri_plugin_window_state::Builder;

fn main() {
  tauri::Builder::default()
    .plugin(Builder::default().build())
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      let mut zoom_factor = 1.0;

      window.on_menu_event(move |event| {
        match event.menu_item_id() {
          "zoom_in" => {
            zoom_factor += 0.1;
            window.with_webview(|webview| {
              webview.eval(&format!("document.body.style.zoom = '{}'", zoom_factor)).unwrap();
            });
          }
          "zoom_out" => {
            zoom_factor -= 0.1;
            window.with_webview(|webview| {
              webview.eval(&format!("document.body.style.zoom = '{}'", zoom_factor)).unwrap();
            });
          }
          _ => {}
        }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
