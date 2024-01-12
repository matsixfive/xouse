// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use std::thread;
use tauri::Manager;

mod mouser;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet]).setup(|app| {
        let window = app.get_window("main").unwrap();
        thread::spawn(move || {
          mouser::start(window).unwrap();
        });
        Ok(())
      })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello {}!", name)
}
