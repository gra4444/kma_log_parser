# Grammar rules

Here pest grammar rules are described.

## time

This rule matches a time in the format `HH:MM:SS`.

Structure: `hour`:`minsec`:`minsec`

```pest
time = { hour ~ ":" ~ minsec ~ ":" ~ minsec }
```

Components:

- **hour**: Matches an hour value from 00 to 23.

```pest
hour = { "0" ~ ASCII_DIGIT | "1" ~ ASCII_DIGIT | "2" ~ '0'..'3' }
```

- **minsec**: Matches minutes or seconds from 00 to 59.

```pest
minsec = { '0'..'5' ~ ASCII_DIGIT }
```

## date

This rule matches a date in the format `YYYY-MM-DD`.

Structure: YYYY-month-day

```pest
date = { ASCII_DIGIT{4} ~ "-" ~ month ~ "-" ~ day }
```

Components:

- **ASCII_DIGIT{4}**: Matches a four-digit year (e.g., 2023).

- **month**: Matches month values from 01 to 12.

```pest
month = { "0" ~ ASCII_NONZERO_DIGIT | "1" ~ '0'..'2' }
```

- **day**: Matches a two-digit day.

```pest
day = { "0" ~ ASCII_NONZERO_DIGIT | '1'..'2' ~ ASCII_DIGIT | "3" ~ '0'..'1' }
```

## datetime

Combines a `date` and a `time` to represent a full timestamp.

```pest
datetime = { date ~ " " ~ time }
```

## loglevel

Matches specific log levels in log entries. 3 different values are supported: INFO, WARNING, ERROR.

```pest
loglevel = { "INFO" | "WARNING" | "ERROR" }
```

## message

Captures message part of the log line. Matches any sequence of characters until a newline, ensuring the content is part of a single line.

```pest
message = { (!NEWLINE ~ ANY)+ }
```

## logline

Represents a full log entry, combining all previous rules. Matches a complete log line that starts with a timestamp, followed by a log level, and ends with a log message.

Structure: `datetime` `loglevel` `message`

```pest
logline = { datetime ~ " " ~ loglevel ~ " " ~ message }
```

### Example log line:

```log
2024-11-06 18:46:37 WARNING A warning log line example
```