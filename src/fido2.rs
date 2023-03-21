struct FidoDevice {
    path: String,
    dev: *mut libfido2_sys::fido_dev,
}

impl FidoDevice {
    pub fn new(path: String) -> Option<FidoDevice> {
        let mut dev;

        unsafe {
            dev = libfido2_sys::fido_dev_new();

            libfido2_sys::fido_dev_open(dev, path.as_bytes().as_ptr() as *const i8);
        }

        Some(FidoDevice { path, dev })
    }
}

impl Drop for FidoDevice {
    fn drop(&mut self) {
        unsafe {
            libfido2_sys::fido_dev_close(self.dev);

            libfido2_sys::fido_dev_free(&mut self.dev);
        }
    }
}
