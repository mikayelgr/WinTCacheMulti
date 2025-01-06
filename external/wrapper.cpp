#include "wrapper.h"

#include <Windows.h>
#include <combaseapi.h>
#include <cstdlib>
#include <processthreadsapi.h>
#include <shobjidl_core.h>
#include <stdio.h>
#include <thumbcache.h>
#include <winbase.h>
#include <winerror.h>
#include <winnt.h>

/**
 * internal__GetShellItemFromPath
 *
 * This function retrieves an IShellItem interface pointer for a given file or
 * folder path. It serves as a helper function to convert a file system path
 * into a Shell Item object that can be used with various Windows Shell APIs.
 *
 * Parameters:
 * - PCWSTR path: The file or folder path as a wide-character string
 * (null-terminated). This parameter is required and must not be NULL.
 * - IShellItem **ppShellItem: A double pointer that will receive the IShellItem
 * interface. This parameter is required and must not be NULL.
 *
 * Returns:
 * - HRESULT: Returns S_OK on success. If either parameter is NULL, it returns
 * E_INVALIDARG. Other failure codes might be returned by
 * SHCreateItemFromParsingName.
 *
 * Notes:
 * - The caller is responsible for releasing the IShellItem interface obtained
 * through this function.
 */
static HRESULT
internal__GetShellItemFromPath(PCWSTR path, IShellItem** ppShellItem)
{
  // Checking to see whether any of the provided arguments are null just in
  // case we miss them in our internal function calls.
  if (!path || !ppShellItem) {
    return E_INVALIDARG;
  }

  // Call SHCreateItemFromParsingName to get the IShellItem. For more info on
  // SHCreateItemFromParsingName, check
  // https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromparsingname
  return SHCreateItemFromParsingName(
    path,                     // File or folder path
    NULL,                     // No bind context
    IID_PPV_ARGS(ppShellItem) // Request IShellItem interface
  );
}

// A helper function for computing the size of the thumbnail based on the size
// of the current file.
static inline int
internal__compute_optimal_thumb_size(IShellItem* entry)
{
  // TODO: Complete the implementation. Note, there might be a WST flag which
  // accomplishes this function and is a Windows built-in.
  return 128;
}

// A wrapper function around CoInitializeEx, for initializing the Windows
// Component Object Model (COM) in multi-thread apartment (MTA) mode.
HRESULT
wrapped__CoInitializeExMulti()
{
  return CoInitializeEx(nullptr, COINIT_MULTITHREADED);
}

// A helper function for extracting the thumbnail of a resource given its
// absolute path on disk. If successful, the function returns an HRESULT
// containing 0.
HRESULT
wrapped__GetThumbnailFromPath(PCWSTR path, WTS_FLAGS flags)
{
  IShellItem* entry = NULL;
  // Getting the shell item from the provided path
  HRESULT code = internal__GetShellItemFromPath(path, &entry);
  if (!SUCCEEDED(code)) {
    return code;
  }

  // Code taken and adapted from the following StackOverflow thread
  // https://stackoverflow.com/q/20949827
  IThumbnailCache* cache = nullptr;
  code = CoCreateInstance(
    CLSID_LocalThumbnailCache, nullptr, CLSCTX_INPROC, IID_PPV_ARGS(&cache));
  if (!SUCCEEDED(code)) {
    return code;
  }

  // For more information on Windows scheduling and priority classes, visit the
  // documentation at
  // https://learn.microsoft.com/en-us/windows/win32/procthread/scheduling-priorities
  // TODO: Experiment with this setting (maybe use this setting when running the
  // program)
  // SetPriorityClass(GetCurrentProcess(), REALTIME_PRIORITY_CLASS);

  // Instructing Windows to extract the thumbnail of the provided entry in the
  // filesystem and making writing it to the local thumbnail cache.
  // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/nf-thumbcache-ithumbnailprovider-getthumbnail
  int thumb_size = internal__compute_optimal_thumb_size(entry);
  code = cache->GetThumbnail(
    entry,
    thumb_size,
    // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/ne-thumbcache-wts_flags
    flags,
    nullptr, // Optionally, provide a bitmap for storing the thumbnail
    nullptr,
    nullptr);
  if (!SUCCEEDED(code)) {
    return code;
  }

  entry->Release();
  // In case of successful extraction, 0 is returned.
  return 0;
}
