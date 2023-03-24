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

    let mut device_list: FidoDeviceList = FidoDeviceList { dev: Vec::new() };

    // TODO unwrap
    for dev_path in fido2::FidoDeviceList::new().unwrap() {
        device_list.dev.push(dev_path);
    }

    device_list
}

#[tauri::command]
async fn fido_get_info(parameters: FidoGetInfoCommand) -> FidoGetInfoResponse {
    debug!("fido_get_info");

    let ret: i32 = unsafe {
        let mut cbor_info = libfido2_sys::fido_cbor_info_new();

        let mut dev = libfido2_sys::fido_dev_new();

        libfido2_sys::fido_dev_open(dev, parameters.dev.as_bytes().as_ptr() as *const i8);

        let ret = libfido2_sys::fido_dev_get_cbor_info(dev, cbor_info);

        let aaguid = libfido2_sys::fido_cbor_info_aaguid_ptr(cbor_info);

        libfido2_sys::fido_dev_close(dev);

        libfido2_sys::fido_dev_free(&mut dev);

        libfido2_sys::fido_cbor_info_free(&mut cbor_info);

        ret
    };

    FidoGetInfoResponse { ret, message: None }
}

#[tauri::command]
async fn fido_reset(parameters: FidoResetCommand) -> FidoResetResponse {
    debug!("fido_reset");

    let fido_device = fido2::FidoDevice::new(parameters.dev);

    let ret = unsafe { libfido2_sys::fido_dev_reset(fido_device.unwrap().dev.as_mut()) };

    let message: String = fido2::strerr(ret);

    FidoResetResponse { ret, message }
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
            fido_reset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
