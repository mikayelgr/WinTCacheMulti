#include "wrapper.h"
#include <combaseapi.h>
#include <objbase.h>
#include <shobjidl_core.h>
#include <thumbcache.h>
#include <winerror.h>
#include <winnt.h>

// A helper function for extracting the thumbnail of a resource given its
// absolute path on disk. If successful, the function returns an HRESULT
// containing 0.
GetThumbnailFromPathResult
wrapped__GetThumbnailFromPath(PCWSTR path, WTS_FLAGS flags, int* codeptr)
{
  // if (!SUCCEEDED(CoInitialize(nullptr))) {
  //   CoUninitialize();
  //   return GetThumbnailFromPathResult::e_CoInitialize_FAILED;
  // }

  // The code pointer is specifically designed for getting the actual error
  // code from Windows and making the function easily debuggable.
  if (!codeptr) {
    // CoUninitialize();
    return GetThumbnailFromPathResult::e_missing_codeptr;
  }

  IShellItem* entry = NULL;
  // Call SHCreateItemFromParsingName to get the IShellItem. For more info on
  // SHCreateItemFromParsingName, check
  // https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromparsingname
  HRESULT code = SHCreateItemFromParsingName(
    path,                // File or folder path
    NULL,                // No bind context
    IID_PPV_ARGS(&entry) // Request IShellItem interface
  );
  if (!SUCCEEDED(code)) {
    // CoUninitialize();
    *codeptr = code;
    return GetThumbnailFromPathResult::e_SHCreateItemFromParsingName_FAILED;
  }

  // Code taken and adapted from the following StackOverflow thread
  // https://stackoverflow.com/q/20949827
  IThumbnailCache* cache = nullptr;
  code = CoCreateInstance(
    CLSID_LocalThumbnailCache, nullptr, CLSCTX_INPROC, IID_PPV_ARGS(&cache));
  if (!SUCCEEDED(code)) {
    // CoUninitialize();
    *codeptr = code;

    // Handled as described in Microsoft docs
    // https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance
    if (code == REGDB_E_CLASSNOTREG) {
      return GetThumbnailFromPathResult::e_CoCreateInstance_REGDB_E_CLASSNOTREG;
    }

    if (code == CLASS_E_NOAGGREGATION) {
      return GetThumbnailFromPathResult::
        e_CoCreateInsance_CLASS_E_NOAGGREGATION;
    }

    if (code == E_NOINTERFACE) {
      return GetThumbnailFromPathResult::e_CoCreateInstance_E_NOINTERFACE;
    }

    return GetThumbnailFromPathResult::e_CoCreateInstance_E_POINTER;
  }

  // Instructing Windows to extract the thumbnail of the provided entry in the
  // filesystem and making writing it to the local thumbnail cache.
  // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/nf-thumbcache-ithumbnailprovider-getthumbnail
  int thumb_size = 128; // For now, defaulting to 128x128
  code = cache->GetThumbnail(
    entry,
    thumb_size,
    // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/ne-thumbcache-wts_flags
    flags,
    nullptr, // Optionally, provide a bitmap for storing the thumbnail
    nullptr,
    nullptr);
  if (!SUCCEEDED(code)) {
    // CoUninitialize();
    *codeptr = code;

    if (code == E_INVALIDARG) {
      return GetThumbnailFromPathResult::e_GetThumbnail_E_INVALIDARG;
    }

    if (code == WTS_E_FAILEDEXTRACTION) {
      return GetThumbnailFromPathResult::e_GetThumbnail_WTS_E_FAILEDEXTRACTION;
    }

    if (code == WTS_E_EXTRACTIONTIMEDOUT) {
      return GetThumbnailFromPathResult::
        e_GetThumbnail_WTS_E_EXTRACTIONTIMEDOUT;
    }

    if (code == WTS_E_SURROGATEUNAVAILABLE) {
      return GetThumbnailFromPathResult::
        e_GetThumbnail_WTS_E_SURROGATEUNAVAILABLE;
    }

    return GetThumbnailFromPathResult::
      e_GetThumbnail_WTS_E_FASTEXTRACTIONNOTSUPPORTED;
  }

  entry->Release();
  cache->Release();
  // In case of successful extraction, 0 is returned.
  return GetThumbnailFromPathResult::ok;
}
