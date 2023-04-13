use log::debug;
use std::ptr::NonNull;

fn from_ptr_to_string(ptr: *const core::ffi::c_char) -> String {
    let raw_message = unsafe { core::ffi::CStr::from_ptr(ptr) };

    String::from_utf8_lossy(raw_message.to_bytes()).to_string()
}

fn strerr(ret: std::os::raw::c_int) -> String {
    from_ptr_to_string(unsafe { libfido2_sys::fido_strerr(ret) })
}

unsafe extern "C" fn fido_log_handler(message: *const i8) {
    debug!("{}", from_ptr_to_string(message))
}

unsafe fn from_str_arr_to_vec<'a>(
    ptr: *mut *mut std::ffi::c_char,
    len: usize,
) -> Option<Vec<&'a str>> {
    ptr.as_ref().map(|ptr| {
        std::slice::from_raw_parts(ptr, len)
            .iter()
            .map(|ext| std::str::from_utf8_unchecked(core::ffi::CStr::from_ptr(*ext).to_bytes()))
            .collect()
    })
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
    pub _path: String,
    dev: NonNull<libfido2_sys::fido_dev>,
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

        Ok(FidoDevice { _path: path, dev })
    }

    pub fn get_info(&mut self) -> Result<AuthenticatorInfo, String> {
        AuthenticatorInfo::new(&mut self.dev)
    }

    pub fn set_pin(&mut self, new_pin: String) -> Result<bool, String> {
        let err = unsafe {
            libfido2_sys::fido_dev_set_pin(
                self.dev.as_mut(),
                new_pin.as_bytes().as_ptr() as *const i8,
                std::ptr::null(),
            )
        };

        if err != libfido2_sys::FIDO_OK {
            return Err(strerr(err));
        }

        Ok(true)
    }

    pub fn change_pin(&mut self, new_pin: String, old_pin: String) -> Result<bool, String> {
        let err = unsafe {
            libfido2_sys::fido_dev_set_pin(
                self.dev.as_mut(),
                new_pin.as_bytes().as_ptr() as *const i8,
                old_pin.as_bytes().as_ptr() as *const i8,
            )
        };

        if err != libfido2_sys::FIDO_OK {
            return Err(strerr(err));
        }

        Ok(true)
    }

    pub fn reset(&mut self) -> Result<bool, String> {
        let err = unsafe { libfido2_sys::fido_dev_reset(self.dev.as_mut()) };

        if err != libfido2_sys::FIDO_OK {
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

pub(crate) struct AuthenticatorInfo {
    cbor_info: NonNull<libfido2_sys::fido_cbor_info>,
}

impl AuthenticatorInfo {
    pub fn new(dev: &mut NonNull<libfido2_sys::fido_dev>) -> Result<AuthenticatorInfo, String> {
        // assume now that memory allocation will never fail
        let cbor_info = NonNull::new(unsafe { libfido2_sys::fido_cbor_info_new() }).unwrap();

        let err = unsafe { libfido2_sys::fido_dev_get_cbor_info(dev.as_ptr(), cbor_info.as_ptr()) };

        if err != libfido2_sys::FIDO_OK {
            unsafe { libfido2_sys::fido_cbor_info_free(&mut cbor_info.as_ptr()) };

            return Err(strerr(err));
        }

        Ok(AuthenticatorInfo { cbor_info })
    }

    pub fn get_versions<'a>(&'a mut self) -> Option<Vec<&'a str>> {
        unsafe {
            from_str_arr_to_vec(
                libfido2_sys::fido_cbor_info_versions_ptr(self.cbor_info.as_ptr()),
                libfido2_sys::fido_cbor_info_versions_len(self.cbor_info.as_ptr()),
            )
        }
    }

    pub fn get_aaguid<'a>(&'a mut self) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(
                libfido2_sys::fido_cbor_info_aaguid_ptr(self.cbor_info.as_ptr()),
                libfido2_sys::fido_cbor_info_aaguid_len(self.cbor_info.as_ptr()),
            )
        }
    }

    pub fn get_extensions<'a>(&'a mut self) -> Option<Vec<&'a str>> {
        unsafe {
            from_str_arr_to_vec(
                libfido2_sys::fido_cbor_info_extensions_ptr(self.cbor_info.as_ptr()),
                libfido2_sys::fido_cbor_info_extensions_len(self.cbor_info.as_ptr()),
            )
        }
    }

    pub fn get_options<'a>(&'a mut self) -> Option<Vec<(&'a str, &'a bool)>> {
        unsafe {
            let keys = from_str_arr_to_vec(
                libfido2_sys::fido_cbor_info_options_name_ptr(self.cbor_info.as_ptr()),
                libfido2_sys::fido_cbor_info_options_len(self.cbor_info.as_ptr()),
            );

            let values: Option<Vec<&bool>> =
                libfido2_sys::fido_cbor_info_options_value_ptr(self.cbor_info.as_ptr())
                    .as_ref()
                    .map(|ptr| {
                        std::slice::from_raw_parts(
                            ptr,
                            libfido2_sys::fido_cbor_info_options_len(self.cbor_info.as_ptr()),
                        )
                        .iter()
                        .map(|value| value)
                        .collect()
                    });

            keys.map(|keys| std::iter::zip(keys, values.unwrap()).collect())
        }
    }
}

impl Drop for AuthenticatorInfo {
    fn drop(&mut self) {
        unsafe { libfido2_sys::fido_cbor_info_free(&mut self.cbor_info.as_ptr()) };
    }
}
