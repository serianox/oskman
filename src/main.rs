#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod fido2;

use log::debug;
use oskman_schemas::schemas::*;

#[tauri::command]
fn fido_init(flags: i32) {
    debug!("fido_init");

    fido2::init(flags)
}

#[tauri::command]
fn fido_list_devices() -> FidoDeviceList {
    debug!("fido_list_devices");

    FidoDeviceList {
        dev: Vec::from_iter(fido2::FidoDeviceList::new().unwrap()),
    }
}

#[tauri::command]
async fn fido_get_info(parameters: FidoGetInfoCommand) -> FidoGetInfoResponse {
    debug!("fido_get_info");

    let fido_device = fido2::FidoDevice::new(parameters.dev).unwrap();

    fido_device.get_info();

    FidoGetInfoResponse { message: None }
}

#[tauri::command]
async fn fido_reset(parameters: FidoResetCommand) -> FidoResetResponse {
    debug!("fido_reset");

    let mut fido_device = fido2::FidoDevice::new(parameters.dev).unwrap();

    match fido_device.reset() {
        Ok(_) => FidoResetResponse { message: None },
        Err(err) => FidoResetResponse { message: Some(err) },
    }
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            use tauri::Manager;

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fido_init,
            fido_list_devices,
            fido_get_info,
            fido_reset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
