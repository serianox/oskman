#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod fido2;

use log::debug;
use oskman_schemas::schemas::*;

#[tauri::command]
fn fido_init() {
    debug!("fido_init");

    fido2::init()
}

#[tauri::command]
fn fido_list_devices() -> Result<FidoDeviceList, String> {
    debug!("fido_list_devices");

    fido2::FidoDeviceList::new().map(|device_list| FidoDeviceList {
        dev: Vec::from_iter(device_list),
    })
}

#[tauri::command]
async fn fido_get_info(parameters: FidoGetInfoCommand) -> Result<FidoGetInfoResponse, String> {
    debug!("fido_get_info");

    let mut fido_device = fido2::FidoDevice::new(parameters.dev)?;

    let mut authenticator_info = fido_device.get_info()?;

    let versions = authenticator_info
        .get_versions()
        .map(|vec| vec.iter().map(|str| str.to_string()).collect());

    let aaguid = hex::encode(authenticator_info.get_aaguid());

    let extensions = authenticator_info
        .get_extensions()
        .map(|vec| vec.iter().map(|str| str.to_string()).collect());

    let options = authenticator_info
        .get_options()
        .map(|vec| vec.iter().map(|str| str.to_string()).collect());

    Ok(FidoGetInfoResponse {
        versions,
        aaguid,
        extensions,
        options,
    })
}

#[tauri::command]
async fn fido_reset(parameters: FidoResetCommand) -> Result<FidoResetResponse, String> {
    debug!("fido_reset");

    let mut fido_device = fido2::FidoDevice::new(parameters.dev)?;

    fido_device
        .reset()
        .map(|_| FidoResetResponse { result: true })
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            use tauri::Manager;

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                app.get_window("main").map(|window| {
                    window.open_devtools();
                    window.close_devtools();
                });
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
