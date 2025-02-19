extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(feature = "dylibfido2")]
    {
        // Tell cargo to tell rustc to link the system libfido2
        // shared library.
        println!("cargo:rustc-link-lib=fido2");

        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!("cargo:rerun-if-changed=wrapper.h");
    }

    #[cfg(not(feature = "dylibfido2"))]
    {
        // This is the directory where the `c` library is located.
        let libdir_path = PathBuf::from("libfido2")
            // Canonicalize the path as `rustc-link-search` requires an absolute
            // path.
            .canonicalize()
            .expect("cannot canonicalize path");

        // This is the path to the static library file.
        let libsrc_path = libdir_path.join("src");

        // This is the path to the static library file.
        let lib_path = libdir_path.join("build/src");

        // Tell cargo to look for shared libraries in the specified directory
        println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());

        // Tell cargo to tell rustc to link the static fido2 library
        println!("cargo:rustc-link-lib=static=fido2");

        // Tell cargo to tell rustc to link the system
        // shared libraries dependencies.
        println!("cargo:rustc-link-lib=udev");
        println!("cargo:rustc-link-lib=cbor");
        println!("cargo:rustc-link-lib=ssl");
        println!("cargo:rustc-link-lib=crypto");
        println!("cargo:rustc-link-lib=pcsclite");

        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!("cargo:rerun-if-changed=wrapper.h");

        // Tell cargo to invalidate the built crate whenever the submodule changes
        println!("cargo:rerun-if-changed={}", libsrc_path.to_str().unwrap());

        // Unwrap if it is not possible to spawn the process.
        if !std::process::Command::new("cmake")
            .current_dir(libdir_path.clone())
            .arg("-B")
            .arg("build")
            .output()
            .expect("could not spawn `cmake`")
            .status
            .success()
        {
            // Panic if the command was not successful.
            panic!("could not run cmake");
        }

        // Unwrap if it is not possible to spawn the process.
        if !std::process::Command::new("make")
            .current_dir(libdir_path.clone())
            .arg("-C")
            .arg("build")
            .output()
            .expect("could not spawn `make`")
            .status
            .success()
        {
            // Panic if the command was not successful.
            panic!("could not run make");
        }
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .allowlist_function("^fido_.*|^.*eddsa_pk_.*|^.*es256_pk_.*|^.*es384_pk_.*|^.*rs256_pk_.*")
        //.allowlist_type("")
        //.allowlist_var("")
        //.allowlist_file("fido.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
