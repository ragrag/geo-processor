Blazing fast geocode and reverse-geocode powered by HERE Maps and Redis
- Battle tested for production environments
- Supports retries with exponential backoff or 429 Retry-After value if provided by HERE
- Uses a blazingly fast [redis async client lib](https://docs.rs/fred/latest/fred/) but also includes different redis pool implementations
- Structured Logging
- Custom cache key prefix
