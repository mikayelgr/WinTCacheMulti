/// This module contains the raw bindings for all the FFI functions that
/// we are going to need.
pub mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/extra_bindings.rs"));
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
    use std::{ffi::CStr, path::PathBuf};

    use crate::{internal, sys};

    #[derive(Debug, Clone)]
    pub struct GetThumbnailError {
        pub code: sys::HRESULT,
        pub msg: String,
    }

    type GetThumbnailResult = Result<(), GetThumbnailError>;

    #[inline]
    fn get_thumbnail_from_path_raw(path: PathBuf, flags: sys::WTS_FLAGS) -> GetThumbnailResult {
        // Convert the Rust String to a wide string (UTF-16) and null-terminate it
        let wstr: Vec<u16> = internal::path_to_wstr(path);
        let output = unsafe { sys::GetThumbnail(wstr.as_ptr(), flags) };
        if output.ok {
            return Ok(());
        }

        Err(GetThumbnailError {
            code: output.code,
            msg: unsafe { CStr::from_ptr(output.error) }
                .to_str()
                .unwrap()
                .to_string(),
        })
    }

    /// Safe binding to the wrapper function wrapped__GetThumbnailFromPath. Given a system
    /// path as a [PathBuf], the function converts the path into a wide string and supplies
    /// it to the actual function. Additional result processing is implemented.
    pub fn get_thumbnail_from_path(path: PathBuf) -> GetThumbnailResult {
        get_thumbnail_from_path_raw(path, sys::WTS_FLAGS_WTS_EXTRACT)
    }

    /// A **helper method to be used only from benchmarking contexts** to force the extraction
    /// of already cached files. This ensures that no administrator permissions are required
    /// for manual removal of the thumbcache data from the respective folders.
    pub fn __bench_force_get_thumbnail_from_path(path: PathBuf) -> GetThumbnailResult {
        get_thumbnail_from_path_raw(path, sys::WTS_FLAGS_WTS_FORCEEXTRACTION)
    }
}
