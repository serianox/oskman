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

pub fn init(flags: i32) {
    unsafe {
        libfido2_sys::fido_init(flags);

        libfido2_sys::fido_set_log_handler(Some(fido_log_handler));
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
}

impl Drop for FidoDevice {
    fn drop(&mut self) {
        unsafe {
            libfido2_sys::fido_dev_close(self.dev.as_mut());

            libfido2_sys::fido_dev_free(&mut self.dev.as_ptr());
        }
    }
}
