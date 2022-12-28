#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod schema;

use schema::CustomResponse;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> Result<CustomResponse, String> {
    let message = format!("Hello, {}! You've been greeted from Rust!", name);
    Ok(CustomResponse {
        message,
        other_val: 42,
    })
}

fn main() {
    unsafe {
        libfido2_sys::fido_init(0);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
