#include <cassert>
#include <combaseapi.h>
#include <comdef.h>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <execution>
#include <filesystem>
#include <objbase.h>
#include <shobjidl_core.h>
#include <string>
#include <thumbcache.h>
#include <tuple>
#include <vector>
#include <winerror.h>
#include <winnt.h>

// A generic function for scaling the thumbnail size based on the size of the
// given file.
static inline int
compute_scaled_thumb_size(uintmax_t filesize,
                          uintmax_t min_domain,
                          uintmax_t max_domain)
{
  static constexpr int MIN_RANGE = 32;
  static constexpr int MAX_RANGE = 1024;
  // Definition taken from https://stats.stackexchange.com/a/281164
  return ((filesize - min_domain) / (max_domain - min_domain)) *
           (MAX_RANGE - MIN_RANGE) +
         MIN_RANGE;
}

// Sequentially reads all the files in the provided directory into the provided
// vector and determines the maximum and minimum sizes of files in the
// directory. The min, max values are supposed to be used later for determining
// the optimal thumbnail size.
static inline std::tuple<uintmax_t, uintmax_t>
get_entries_and_minmax(std::string path,
                       std::vector<std::filesystem::directory_entry>& entries)
{
  bool min_set = false;
  uintmax_t min_size = 0;
  uintmax_t max_size = 0;
  for (const auto& entry : std::filesystem::directory_iterator(path)) {
    if (entry.is_regular_file()) {
      entries.push_back(entry);

      // The file size condition will only be checked for the regular files
      // since folders can be empty as well.
      uintmax_t fsz = entry.file_size();
      if (fsz != 0 && (!min_set || fsz < min_size)) {
        min_size = fsz;
        min_set = true;
      }

      if (fsz > max_size) {
        max_size = fsz;
      }
    }
  }

  return std::tuple{ min_size, max_size };
}

// A helper function for extracting the thumbnail of a resource given its
// absolute path on disk. If successful, the function returns an HRESULT
// containing 0.
#pragma comment(lib, "ole32")
#pragma comment(lib, "user32")
#pragma comment(lib, "shell32")
static std::tuple<bool, HRESULT, std::string>
get_thumbnail_from_path(IThumbnailCache* cache,
                        PCWSTR path,
                        WTS_FLAGS flags,
                        int thumb_size)
{
#define HANDLE_ERROR(code)                                                     \
  if (FAILED(code)) {                                                          \
    _com_error err(code);                                                      \
    auto error = std::format(                                                  \
      "error: {}:{} {}", __FUNCTION__, __LINE__, err.ErrorMessage());          \
    return std::tuple{ false, code, error };                                   \
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
  HANDLE_ERROR(code);

  // Instructing Windows to extract the thumbnail of the provided entry in the
  // filesystem and making writing it to the local thumbnail cache.
  // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/nf-thumbcache-ithumbnailprovider-getthumbnail
  code = cache->GetThumbnail(
    entry,
    thumb_size,
    // https://learn.microsoft.com/en-us/windows/win32/api/thumbcache/ne-thumbcache-wts_flags
    flags,
    nullptr, // Optionally, provide a bitmap for storing the thumbnail
    nullptr,
    nullptr);
  HANDLE_ERROR(code);

  entry->Release(); // releasing the file entry
  cache->Release(); // releasing the file cache
  // In case of successful extraction, 0 is returned.
  return std::tuple{ true, code, "" };
}

int
main(const int argc, const char** argv)
{
  if (argc < 2) {
    printf("Usage: %s <directory>\n", argv[0]);
    return 1;
  }

  auto path = std::filesystem::absolute(argv[1]);
  std::vector<std::filesystem::directory_entry> entries;
  auto [min, max] = get_entries_and_minmax(path.string(), entries);
  printf("min: %llu, max: %llu\n", min, max);

  // We need to make sure that the component object model (COM) is initialized
  HRESULT hr = CoInitializeEx(nullptr, COINIT_MULTITHREADED);
  if (FAILED(hr)) {
    _com_error err(hr);
    printf("error: %s:%i, message='%s'\n",
           __FUNCTION__,
           __LINE__,
           err.ErrorMessage());
    return hr;
  }

  // Code taken and adapted from the following StackOverflow thread
  // https://stackoverflow.com/q/20949827
  IThumbnailCache* cache = nullptr;
  if (FAILED(CoCreateInstance(CLSID_LocalThumbnailCache,
                              nullptr,
                              CLSCTX_INPROC,
                              IID_PPV_ARGS(&cache)))) {
    printf("Failed to get cache\n");
  }

  int proc = 0;
  std::for_each(
    std::execution::par,
    entries.begin(),
    entries.end(),
    [&min, &max, &proc, &cache](const std::filesystem::directory_entry& entry) {
      auto [ok, code, error] = get_thumbnail_from_path(
        cache,
        entry.path().c_str(),
        WTS_FLAGS::WTS_EXTRACT,
        compute_scaled_thumb_size(entry.file_size(), min, max));
      if (!ok) {
        if (code != WTS_E_FAILEDEXTRACTION) {
          printf("%s\n", error.data());
        }
      } else {
        proc++;
      }
    });

  cache->Release();
  printf("%i\n", proc);
  CoUninitialize();
  return 0;
}