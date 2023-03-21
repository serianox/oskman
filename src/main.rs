#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod fido2;

use log::debug;
use oskman_schemas::schemas::*;

fn from_ptr_to_string(ptr: *const i8) -> String {
    let raw_message = unsafe { core::ffi::CStr::from_ptr(ptr) };

    String::from_utf8_lossy(raw_message.to_bytes()).to_string()
}

unsafe extern "C" fn fido_log_handler(message: *const i8) {
    debug!("{}", from_ptr_to_string(message))
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
fn fido_list_devices() -> FidoDeviceList {
    debug!("fido_list_devices");

    let device_list: FidoDeviceList;

    device_list = unsafe {
        let mut dev_list = libfido2_sys::fido_dev_info_new(64);

        let mut n: usize = 0;

        let err = libfido2_sys::fido_dev_info_manifest(dev_list, 64, &mut n);

        if err != libfido2_sys::FIDO_OK {}

        let mut ret: FidoDeviceList = FidoDeviceList { dev: Vec::new() };

        for dev_id in 0..n {
            let dev = libfido2_sys::fido_dev_info_ptr(dev_list, dev_id);

            ret.dev
                .push(from_ptr_to_string(libfido2_sys::fido_dev_info_path(dev)));
        }

        libfido2_sys::fido_dev_info_free(&mut dev_list, n);

        ret
    };

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

        let aaguid = libfido2_sys:: fido_cbor_info_aaguid_ptr(cbor_info);

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

    let ret: i32 = unsafe {
        let mut dev = libfido2_sys::fido_dev_new();

        libfido2_sys::fido_dev_open(dev, parameters.dev.as_bytes().as_ptr() as *const i8);

        let ret = libfido2_sys::fido_dev_reset(dev);

        libfido2_sys::fido_dev_close(dev);

        libfido2_sys::fido_dev_free(&mut dev);

        ret
    };

    let message: String = from_ptr_to_string(unsafe { libfido2_sys::fido_strerr(ret) });

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
