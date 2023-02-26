#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::debug;
use oskman_schemas::schemas::{CustomResponse, FidoResetCommand, FidoResetResponse};

fn from_ptr_to_string(ptr: *const i8) -> String {
    let raw_message = unsafe { core::ffi::CStr::from_ptr(ptr) };

    String::from_utf8_lossy(raw_message.to_bytes()).to_string()
}

unsafe extern "C" fn fido_log_handler(message: *const i8) {
    println!("{}", from_ptr_to_string(message))
}

#[tauri::command]
fn fido_init(flags: i32) {
    debug!("fido_init");

    unsafe {
        libfido2_sys::fido_init(flags);

        libfido2_sys::fido_set_log_handler(Some(fido_log_handler));
    }
}

#[tauri::command]
async fn fido_reset(parameters: FidoResetCommand) -> FidoResetResponse {
    debug!("fido_reset");

    let ret: i32;

    unsafe {
        let dev = libfido2_sys::fido_dev_new();

        libfido2_sys::fido_dev_open(dev, parameters.dev.as_bytes().as_ptr() as *const i8);

        libfido2_sys::fido_dev_close(dev);

        ret = libfido2_sys::fido_dev_reset(dev);
    }

    let message: String = from_ptr_to_string(unsafe { libfido2_sys::fido_strerr(ret) });

    FidoResetResponse { ret, message }
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
        .invoke_handler(tauri::generate_handler![fido_init, fido_reset, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
