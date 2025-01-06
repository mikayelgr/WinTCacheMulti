/// This module contains the raw bindings for all the FFI functions that
/// we are going to need.
mod sys {
    // For more information, check out the official documentation from
    // Microsoft at https://learn.microsoft.com/en-us/windows/win32/com/the-com-library

    use crate::bindings;
    unsafe extern "system" {
        /// Binding to the Component Object Model (COM) initializer.
        pub unsafe fn CoInitialize(_: *mut core::ffi::c_void) -> core::ffi::c_long;
        /// Uninitializes the Component Object Model (COM).
        pub unsafe fn CoUninitialize();
    }

    // Wrapper function definitions come from the wrapper.cpp file in the `external`
    // directory.
    unsafe extern "C" {
        pub unsafe fn wrapped__GetThumbnailFromPath(
            path: *const u16,
            flags: bindings::extra::WTS_FLAGS,
        ) -> ::std::os::raw::c_int;
        /// A wrapper function the Component Object Model (COM)
        /// https://learn.microsoft.com/en-us/windows/win32/api/_com/
        pub unsafe fn wrapped__CoInitializeExMulti() -> core::ffi::c_long;
    }
}

/// This module contains internal function definitions which are useful for working
/// with various types as well as interfacing with FFI.
mod internal {
    use std::{os::windows::ffi::OsStrExt, path::PathBuf};

    #[inline(always)]
    pub fn path_to_wstr(pb: PathBuf) -> Vec<u16> {
        return std::ffi::OsStr::new(&pb)
            .encode_wide()
            // Appending a null terminator manually
            .chain(Some(0))
            .collect();
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

        include!(concat!(env!("OUT_DIR"), "/extra_bindings.rs"));
    }

    #[inline]
    fn get_thumbnail_from_path_raw(path: PathBuf, flags: extra::WTS_FLAGS) -> io::Result<()> {
        // Convert the Rust String to a wide string (UTF-16) and null-terminate it
        let wstr: Vec<u16> = internal::path_to_wstr(path);
        let code = unsafe { sys::wrapped__GetThumbnailFromPath(wstr.as_ptr(), flags) };
        if code == 0 {
            return Ok(());
        }

        Err(io::Error::new(
            ErrorKind::Other,
            format!("`wrapped__GetThumbnailFromPath` failed: code={}", code),
        ))
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

    /// Safely initializes the COM library in multithreaded mode and identifies the concurrency model
    /// as single-thread apartment (STA).
    pub fn coinitialize_sta() -> io::Result<()> {
        let code = unsafe { sys::CoInitialize(0 as *mut core::ffi::c_void) };
        if code == 0 {
            return Ok(());
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to initialize COM in STA mode",
        ))
    }

    /// Safely initializes the COM library in multithreaded mode and identifies the concurrency model
    /// as multi-thread apartment (MTA).
    pub fn coinitialize_mta() -> io::Result<()> {
        let output = unsafe { sys::wrapped__CoInitializeExMulti() };
        if output == 0 {
            return Ok(());
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to initialize COM in MTA mode",
        ))
    }

    /// In the end, the progam must call this function to uninitialize the
    /// Component Object Model (COM) library.
    pub fn couninitialize() {
        unsafe { sys::CoUninitialize() }
    }
}
