#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod fido2;

use log::debug;
use oskman_schemas::schemas::*;
use tauri::Manager;

#[tauri::command]
fn fido_init(window: tauri::Window) {
    debug!("fido_init");

    fido2::init();

    use inotify::{Inotify, WatchMask};

    let mut inotify = Inotify::init().expect("Failed to initialize an inotify instance");

    inotify
        .add_watch("/dev", WatchMask::CREATE | WatchMask::ATTRIB | WatchMask::DELETE)
        .expect("Failed to add file watch");

    std::thread::spawn(move || loop {
        let mut buffer = [0; 1024];
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error while waiting for events");

        for event in events {
            event.name.map(|event_name| {
                event_name.to_str().map(|event_name| {
                    if event_name.starts_with("hidraw") {
                        window.app_handle().emit_all("hid-watch", {}).unwrap();
                    }
                });
            });
        }
    });
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

    let options = {
        authenticator_info.get_options().map(|options| {
            let mut plat = None;
            let mut rk = None;
            let mut client_pin = None;
            let mut up = None;
            let mut uv = None;

            for (key, &value) in options.iter() {
                match key {
                    &"plat" => plat = Some(value),
                    &"rk" => rk = Some(value),
                    &"clientPin" => client_pin = Some(value),
                    &"up" => up = Some(value),
                    &"uv" => uv = Some(value),
                    _ => {}
                }
            }

            FidoGetInfoOptions {
                plat,
                rk,
                client_pin,
                up,
                uv,
            }
        })
    };

    Ok(FidoGetInfoResponse {
        versions,
        aaguid,
        extensions,
        options,
    })
}

#[tauri::command]
async fn fido_set_pin(parameters: FidoSetPinCommand) -> Result<FidoSetPinResponse, String> {
    debug!("fido_set_pin");

    let mut fido_device = fido2::FidoDevice::new(parameters.dev)?;

    fido_device
        .set_pin(parameters.new_pin)
        .map(|_| FidoSetPinResponse { result: true })
}

#[tauri::command]
async fn fido_change_pin(
    parameters: FidoChangePinCommand,
) -> Result<FidoChangePinResponse, String> {
    debug!("fido_change_pin");

    let mut fido_device = fido2::FidoDevice::new(parameters.dev)?;

    fido_device
        .change_pin(parameters.new_pin, parameters.old_pin)
        .map(|_| FidoChangePinResponse { result: true })
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
            fido_set_pin,
            fido_change_pin,
            fido_reset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
