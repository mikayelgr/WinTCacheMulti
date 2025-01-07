#include <ShObjIdl.h>          // For IShellItem
#include <Shlwapi.h>           // For SHCreateItemFromParsingName
#include <Windows.h>           // For general windows typedefs and functions
#include <processthreadsapi.h> // For SetPriorityClass
#include <roapi.h>             // For Windows::Foundation::Initialize
#include <thumbcache.h>        // For interfacing with the thumbnail cache
#include <winbase.h>
#include <winerror.h>

#ifndef WRAPPER_H
#define WRAPPER_H

#ifdef __cplusplus
extern "C"
{
#endif
  typedef enum
  {
    ok,
    // Custom error for the actual wrapper function
    e_missing_codeptr,

    e_CoInitialize_FAILED,
    e_SHCreateItemFromParsingName_FAILED,

    // https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance#:~:text=was%20successfully%20created.-,REGDB_E_CLASSNOTREG,-A%20specified%20class
    e_CoCreateInstance_REGDB_E_CLASSNOTREG,
    e_CoCreateInsance_CLASS_E_NOAGGREGATION,
    e_CoCreateInstance_E_NOINTERFACE,
    e_CoCreateInstance_E_POINTER,

    // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/nf-thumbcache-ithumbnailcache-getthumbnail
    e_GetThumbnail_E_INVALIDARG,
    e_GetThumbnail_WTS_E_FAILEDEXTRACTION,
    e_GetThumbnail_WTS_E_EXTRACTIONTIMEDOUT,
    e_GetThumbnail_WTS_E_SURROGATEUNAVAILABLE,
    e_GetThumbnail_WTS_E_FASTEXTRACTIONNOTSUPPORTED,
  } GetThumbnailFromPathResult;

  // The default recommended flag is WTS_FLAGS::WTS_EXTRACT, however, for
  // benchmarking purposes, we will need to use WTS_FORCEEXTRACTION as described
  // in Microsoft's documentation for <thumbcache.h> header.
  // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/ne-thumbcache-wts_flags#:~:text=format%2C%20typically%2096x96.-,WTS_FORCEEXTRACTION,-Value%3A%200x4
  GetThumbnailFromPathResult wrapped__GetThumbnailFromPath(PCWSTR path,
                                                           WTS_FLAGS flags,
                                                           int*);

#ifdef __cplusplus
}
#endif

#endif // WRAPPER_H
