#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::debug;
use oskman_schemas::schemas::CustomResponse;

#[tauri::command]
fn fido_init(flags: i32) {
    debug!("fido_init");

    unsafe {
        libfido2_sys::fido_init(flags);
    }
}

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
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fido_init, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
