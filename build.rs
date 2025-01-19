#[cfg(target_os = "windows")]
fn main() {
    use std::{env, path::PathBuf};

    println!("cargo:rerun-if-changed=external/wrapper.cpp");
    println!("cargo:rerun-if-changed=external/wrapper.h");

    // Compiling the C++ wrapper library that we have created for interfacing with
    // the Windows APIs.
    cc::Build::new()
        .cpp(true)
        .compiler("clang++")
        // The wrapper is written in C++ 20
        .file("external/wrapper.cpp")
        .compile("wrapper");

    // Generating some extra bindings from the <wrapper.h> header
    let extra_bindings = bindgen::Builder::default()
        .header("external/wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .constified_enum("WTS_FLAGS")
        .allowlist_var("WTS_FLAGS")
        .allowlist_type("GetThumbnailResult")
        .allowlist_function("GetThumbnail")
        .allowlist_function("CoInitializeEx")
        .allowlist_function("CoUninitialize")
        .allowlist_function("CoInitialize")
        .allowlist_var("WTS_E_FAILEDEXTRACTION")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .generate()
        .expect("Unable to generate type bindings for <wrapper.h>");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    extra_bindings
        .write_to_file(out_path.join("extra_bindings.rs"))
        .expect("Couldn't write bindings!");

    // Required DLLs for interfacing with Windows APIs
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=static=wrapper");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("threaded_thumb_cache is only supported for Windows operating systems.");
}
