#include "wrapper.hpp"
#include <combaseapi.h>
#include <comdef.h>
#include <objbase.h>
#include <shobjidl_core.h>
#include <string>
#include <thumbcache.h>
#include <winerror.h>
#include <winnt.h>

#define HANDLE_ERROR(code)                                                     \
  if (FAILED(code)) {                                                          \
    _com_error err(code);                                                      \
    std::string error = "error: " + std::string(__FUNCTION__) + ":" +          \
                        std::to_string(__LINE__) + " " + err.ErrorMessage();   \
    return GetThumbnailResult{ false, code, error.c_str() };                   \
  }

#pragma comment(lib, "ole32")
#pragma comment(lib, "user32")
#pragma comment(lib, "shell32")
// A helper function for extracting the thumbnail of a resource given its
// absolute path on disk. If successful, the function returns an HRESULT
// containing 0.
GetThumbnailResult
GetThumbnail(PCWSTR path, WTS_FLAGS flags)
{
  IShellItem* entry = NULL;
  // Call SHCreateItemFromParsingName to get the IShellItem. For more info on
  // SHCreateItemFromParsingName, check
  // https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromparsingname
  HRESULT code = SHCreateItemFromParsingName(
    path,                // File or folder path
    NULL,                // No bind context
    IID_PPV_ARGS(&entry) // Request IShellItem interface
  );
  HANDLE_ERROR(code);

  // Code taken and adapted from the following StackOverflow thread
  // https://stackoverflow.com/q/20949827
  IThumbnailCache* cache = nullptr;
  code = CoCreateInstance(
    CLSID_LocalThumbnailCache, nullptr, CLSCTX_INPROC, IID_PPV_ARGS(&cache));
  HANDLE_ERROR(code);

  // Instructing Windows to extract the thumbnail of the provided entry in the
  // filesystem and making writing it to the local thumbnail cache.
  // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/nf-thumbcache-ithumbnailprovider-getthumbnail
  code = cache->GetThumbnail(
    entry,
    // Both WinThumbsPreloader and WinThumbsPreloader-V2 use 128x128 resolution
    128,
    // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/ne-thumbcache-wts_flags
    flags,
    nullptr, // Optionally, provide a bitmap for storing the thumbnail
    nullptr,
    nullptr);
  HANDLE_ERROR(code);

  // Resources need to be released after extracting the thumbnail
  entry->Release();
  cache->Release();
  // In case of successful extraction, 0 is returned.
  return GetThumbnailResult{ true, code, "" };
}
