/// This module contains the raw bindings for all the FFI functions that
/// we are going to need.
mod sys {
    use crate::bindings::extra;
    pub use extra::wrapped__GetThumbnailFromPath;
}

/// This module contains internal function definitions which are useful for working
/// with various types as well as interfacing with FFI.
pub(crate) mod internal {
    use std::{os::windows::ffi::OsStrExt, path::PathBuf};

    #[inline(always)]
    pub(crate) fn path_to_wstr(pb: PathBuf) -> Vec<u16> {
        std::ffi::OsStr::new(&pb)
            .encode_wide()
            // Appending a null terminator manually
            .chain(Some(0))
            .collect()
    }
}

pub mod bindings {
    use std::{
        io::{self, ErrorKind},
        path::PathBuf,
    };

    use crate::{internal, sys};

    pub mod extra {
        #![allow(non_upper_case_globals)]
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]

        use std::fmt::Display;

        include!(concat!(env!("OUT_DIR"), "/extra_bindings.rs"));

        impl Display for GetThumbnailFromPathResult {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    GetThumbnailFromPathResult::ok => "OK",
                    GetThumbnailFromPathResult::e_missing_codeptr => "E_MISSING_CODEPTR",
                    GetThumbnailFromPathResult::e_CoCreateInstance_REGDB_E_CLASSNOTREG => {
                        "e_CoCreateInstance_REGDB_E_CLASSNOTREG"
                    }
                    GetThumbnailFromPathResult::e_CoCreateInsance_CLASS_E_NOAGGREGATION => {
                        "e_CoCreateInsance_CLASS_E_NOAGGREGATION"
                    }
                    GetThumbnailFromPathResult::e_CoCreateInstance_E_NOINTERFACE => {
                        "e_CoCreateInstance_E_NOINTERFACE"
                    }
                    GetThumbnailFromPathResult::e_CoCreateInstance_E_POINTER => {
                        "e_CoCreateInstance_E_POINTER"
                    }
                    GetThumbnailFromPathResult::e_GetThumbnail_E_INVALIDARG => {
                        "e_GetThumbnail_E_INVALIDARG"
                    }
                    GetThumbnailFromPathResult::e_GetThumbnail_WTS_E_FAILEDEXTRACTION => {
                        "e_GetThumbnail_WTS_E_FAILEDEXTRACTION"
                    }
                    GetThumbnailFromPathResult::e_GetThumbnail_WTS_E_EXTRACTIONTIMEDOUT => {
                        "e_GetThumbnail_WTS_E_EXTRACTIONTIMEDOUT"
                    }
                    GetThumbnailFromPathResult::e_GetThumbnail_WTS_E_SURROGATEUNAVAILABLE => {
                        "e_GetThumbnail_WTS_E_SURROGATEUNAVAILABLE"
                    }
                    GetThumbnailFromPathResult::e_GetThumbnail_WTS_E_FASTEXTRACTIONNOTSUPPORTED => {
                        "e_GetThumbnail_WTS_E_FASTEXTRACTIONNOTSUPPORTED"
                    }
                    GetThumbnailFromPathResult::e_CoInitialize_FAILED => "e_CoInitialize_FAILED",
                    GetThumbnailFromPathResult::e_SHCreateItemFromParsingName_FAILED => {
                        "e_SHCreateItemFromParsingName_FAILED"
                    }
                })
            }
        }
    }

    #[inline]
    fn get_thumbnail_from_path_raw(path: PathBuf, flags: extra::WTS_FLAGS) -> io::Result<()> {
        // Convert the Rust String to a wide string (UTF-16) and null-terminate it
        let wstr: Vec<u16> = internal::path_to_wstr(path);
        let mut code: std::ffi::c_int = 0 as std::ffi::c_int;
        let code_ptr: *mut i32 = &mut code;
        match unsafe { sys::wrapped__GetThumbnailFromPath(wstr.as_ptr(), flags, code_ptr) } {
            extra::GetThumbnailFromPathResult::ok => Ok(()),
            error => Err(io::Error::new(
                ErrorKind::Other,
                format!(
                    "Failed to get thumbnail from path. Error: {}, Code: {}",
                    error, code
                ),
            )),
        }
    }

    /// Safe binding to the wrapper function wrapped__GetThumbnailFromPath. Given a system
    /// path as a [PathBuf], the function converts the path into a wide string and supplies
    /// it to the actual function. Additional result processing is implemented.
    pub fn get_thumbnail_from_path(path: PathBuf) -> io::Result<()> {
        get_thumbnail_from_path_raw(path, extra::WTS_FLAGS::WTS_EXTRACT)
    }

    /// A helper method to be used only from benchmarking contexts to force the extraction
    /// of already cached files. This ensures that no administrator permissions are required
    /// for manual removal of the thumbcache data from the respective folders.
    pub fn __bench_force_get_thumbnail_from_path(path: PathBuf) -> io::Result<()> {
        get_thumbnail_from_path_raw(path, extra::WTS_FLAGS::WTS_EXTRACTDONOTCACHE)
    }
}
