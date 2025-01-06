#include <ShObjIdl.h>          // For IShellItem
#include <Shlwapi.h>           // For SHCreateItemFromParsingName
#include <Windows.h>           // For general windows typedefs and functions
#include <processthreadsapi.h> // For SetPriorityClass
#include <thumbcache.h>        // For interfacing with the thumbnail cache
#include <winbase.h>

#ifndef WRAPPER_H
#define WRAPPER_H

#ifdef __cplusplus
extern "C"
{
#endif
  HRESULT wrapped__CoInitializeExMulti();
  // The default recommended flag is WTS_FLAGS::WTS_EXTRACT, however, for
  // benchmarking purposes, we will need to use WTS_FORCEEXTRACTION as described
  // in Microsoft's documentation for <thumbcache.h> header.
  // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/ne-thumbcache-wts_flags#:~:text=format%2C%20typically%2096x96.-,WTS_FORCEEXTRACTION,-Value%3A%200x4
  HRESULT wrapped__GetThumbnailFromPath(PCWSTR path, WTS_FLAGS flags);

#ifdef __cplusplus
}
#endif

#endif // WRAPPER_H
