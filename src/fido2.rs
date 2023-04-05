use log::debug;
use std::ptr::NonNull;

fn from_ptr_to_string(ptr: *const i8) -> String {
    let raw_message = unsafe { core::ffi::CStr::from_ptr(ptr) };

    String::from_utf8_lossy(raw_message.to_bytes()).to_string()
}

pub fn strerr(ret: std::os::raw::c_int) -> String {
    from_ptr_to_string(unsafe { libfido2_sys::fido_strerr(ret) })
}

unsafe extern "C" fn fido_log_handler(message: *const i8) {
    debug!("{}", from_ptr_to_string(message))
}

pub fn init() {
    unsafe {
        libfido2_sys::fido_init(libfido2_sys::FIDO_DEBUG);

        libfido2_sys::fido_set_log_handler(Some(fido_log_handler));
    }
}

pub(crate) struct FidoDeviceList {
    c: usize,
    n: usize,
    dev_list: NonNull<libfido2_sys::fido_dev_info>,
}

impl FidoDeviceList {
    pub fn new() -> Result<FidoDeviceList, String> {
        let mut dev_list = NonNull::new(unsafe { libfido2_sys::fido_dev_info_new(64) }).unwrap();

        let mut n: usize = 0;

        let err = unsafe { libfido2_sys::fido_dev_info_manifest(dev_list.as_mut(), 64, &mut n) };

        if err != libfido2_sys::FIDO_OK {
            unsafe { libfido2_sys::fido_dev_info_free(&mut dev_list.as_ptr(), n) };

            return Err(strerr(err));
        }

        Ok(FidoDeviceList { c: 0, n, dev_list })
    }
}

impl Iterator for FidoDeviceList {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.c == self.n {
            return None;
        }

        let dev = unsafe { libfido2_sys::fido_dev_info_ptr(self.dev_list.as_mut(), self.c) };

        self.c += 1;

        Some(from_ptr_to_string(unsafe {
            libfido2_sys::fido_dev_info_path(dev)
        }))
    }
}

impl Drop for FidoDeviceList {
    fn drop(&mut self) {
        unsafe {
            libfido2_sys::fido_dev_info_free(&mut self.dev_list.as_ptr(), self.n);
        }
    }
}

pub(crate) struct FidoDevice {
    pub path: String,
    pub dev: NonNull<libfido2_sys::fido_dev>,
}

impl FidoDevice {
    pub fn new(path: String) -> Result<FidoDevice, String> {
        // assume now that memory allocation will never fail
        let mut dev = NonNull::new(unsafe { libfido2_sys::fido_dev_new() }).unwrap();

        let err = unsafe {
            libfido2_sys::fido_dev_open(dev.as_mut(), path.as_bytes().as_ptr() as *const i8)
        };

        if err != libfido2_sys::FIDO_OK {
            unsafe { libfido2_sys::fido_dev_free(&mut dev.as_ptr()) };

            return Err(strerr(err));
        }

        Ok(FidoDevice { path, dev })
    }

    pub fn get_info(&self) -> Result<bool, String> {
        let cbor_info = NonNull::new(unsafe { libfido2_sys::fido_cbor_info_new() }).unwrap();

        let err =
            unsafe { libfido2_sys::fido_dev_get_cbor_info(self.dev.as_ptr(), cbor_info.as_ptr()) };

        if err != libfido2_sys::FIDO_OK {
            unsafe { libfido2_sys::fido_cbor_info_free(&mut cbor_info.as_ptr()) };

            return Err(strerr(err));
        }

        let _aaguid = unsafe { libfido2_sys::fido_cbor_info_aaguid_ptr(cbor_info.as_ptr()) };

        unsafe { libfido2_sys::fido_cbor_info_free(&mut cbor_info.as_ptr()) };

        Ok(true)
    }

    pub fn reset(&mut self) -> Result<bool, String> {
        let err = unsafe { libfido2_sys::fido_dev_reset(self.dev.as_mut()) };

        if err != libfido2_sys::FIDO_OK {
            unsafe { libfido2_sys::fido_dev_free(&mut self.dev.as_ptr()) };

            return Err(strerr(err));
        }

        Ok(true)
    }
}

impl Drop for FidoDevice {
    fn drop(&mut self) {
        unsafe {
            libfido2_sys::fido_dev_close(self.dev.as_mut());

            libfido2_sys::fido_dev_free(&mut self.dev.as_ptr());
        }
    }
}
