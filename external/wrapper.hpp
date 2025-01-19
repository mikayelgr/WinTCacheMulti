#include <thumbcache.h>
#include <winnt.h>

#ifndef WRAPPER_H
#define WRAPPER_H

typedef struct
{
  bool ok;
  HRESULT code;
  const char* error;
} GetThumbnailResult;

// The default recommended flag is WTS_FLAGS::WTS_EXTRACT, however, for
// benchmarking purposes, we will need to use WTS_FORCEEXTRACTION as described
// in Microsoft's documentation for <thumbcache.h> header.
// https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/ne-thumbcache-wts_flags#:~:text=format%2C%20typically%2096x96.-,WTS_FORCEEXTRACTION,-Value%3A%200x4
GetThumbnailResult
GetThumbnail(PCWSTR path, WTS_FLAGS flags);

#endif // WRAPPER_H
