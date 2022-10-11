# 1.1.0 (2022-10-11)
- Switched from `reqwest` to `ureq` as the HTTP client library
  - This results in fewer dependencies and thus shorter builds
  - But it also makes all requests run in blocking mode
  - The API client is thread-safe, however, so making requests in parallel is possible and up to the user of this library

# 1.0.1 (2022-10-10)
- Small changes to meta information and README

# 1.0.0 (2022-10-10)
- First stable version of the library