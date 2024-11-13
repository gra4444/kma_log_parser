# Log Parser

Log Parser is a simple Rust-based log file parser that uses `pest` crate to parse log files, helping with extracting datetime, log levels and messages.

Log Parser can be used to extract logs with specified log `level` into another file OR/AND into `json` format using `serde`.

## Format

Logs are parsed in the following format: `Date Time Level Message`.

- **Date** - date specified in `YYYY-MM-DD` format, e.g. `2024-11-07`.
- **Time** - time specified in `HH:MM:SS` format, e.g. `15:41:07`.
- **Level** - one of three (currently) logging levels: `INFO`, `WARNING`, `ERROR`.
- **Message** - message of the log line.

## CLI Usage

Log parser can be used via CLI (created using `clap`) with the following options:

### Basic command

```
log_parser_kma -f <file_path> -o <output_path> [options]
```

### Options

- `-f, --file <file_path>` - specifies the path to the log file to be parsed. **(Required)**
- `-o, --output <output_path>` - specifies the path to the output file. **(Required)**
- `-l, --level <log_level>` - specifies log level to filter by (`INFO`, `WARNING` or `ERROR`). **(Optional)**
- `-j, --to-json` - outputs the logs in JSON format instead of plain text. **(Optional)**

### Examples

Parse a log file and save output as JSON

```
log_parser_kma -f logs.log -o results.txt -j
```

Parse a log file, filter `ERROR` level logs and output as JSON

```
log_parser_kma -f newlogs.log -o errorlogs.txt -l ERROR -j
```

## Published on

- **crates.io**: https://crates.io/crates/log_parser_kma

- **docs.rs**: 

  - https://docs.rs/log_parser_kma/0.1.0/log_parser_kma

  - https://docs.rs/crate/log_parser_kma/0.1.0