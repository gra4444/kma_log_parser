# Log Parser

Log Parser is a simple Rust-based log file parser that uses `pest` crate to parse log files, helping with extracting datetime, log levels and messages.

Log Parser can be used to extract logs with specified log `level` into another file OR/AND into `json` format. (TODO, `serde` will be used(?))

## Format

Logs are parsed in the following format: `Date Time Level Message`.

- **Date** - date specified in `YYYY-MM-DD` format, e.g. `2024-11-07`.
- **Time** - time specified in `HH:MM:SS` format, e.g. `15:41:07`.
- **Level** - one of three (currently) logging levels: `INFO`, `WARNING`, `ERROR`.
- **Message** - message of the log line.