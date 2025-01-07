#[cfg(target_os = "windows")]
fn main() {
    use std::{env, path::PathBuf};

    println!("cargo:rerun-if-changed=external/wrapper.cpp");
    println!("cargo:rerun-if-changed=external/wrapper.h");

    // Compiling the C++ wrapper library that we have created for interfacing with
    // the Windows APIs.
    cc::Build::new()
        .file("external/wrapper.cpp")
        .compile("wrapper");
    // Required DLLs for interfacing with Windows APIs
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=static=wrapper");

    // Generating some extra bindings from the <wrapper.h> header
    let extra_bindings = bindgen::Builder::default()
        .header("external/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_type("WTS_FLAGS")
        .allowlist_type("GetThumbnailFromPathResult")
        .allowlist_function("wrapped__GetThumbnailFromPath")
        .allowlist_function("CoInitializeEx")
        .allowlist_function("CoUninitialize")
        .allowlist_function("CoInitialize")
        // .allowlist_type("HIGH_PRIORITY_CLASS")
        // .allowlist_function("SetPriorityClass")
        // .allowlist_function("GetCurrentProcess")
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
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("threaded_thumb_cache is only supported for Windows operating systems.");
}
