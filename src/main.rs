use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use log_parser_kma::{LogLevel, LogLine, LogParser};
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    let matches = Command::new("Log Parser")
        .version("0.1.2")
        .subcommand(
            Command::new("credits")
                        .about("Displays the credits")
        )
        .subcommand(
            Command::new("parse")
                        .about("Parses a log file with different options")
                        .arg(
                            Arg::new("file")
                                .short('f')
                                .help("The file to parse")
                                .required(true),
                        )
                        .arg(
                            Arg::new("output")
                                .short('o')
                                .help("Path to output file")
                                .required(true),
                        )
                        .arg(
                            Arg::new("level")
                                .short('l')
                                .help("Extract logs of specified level")
                                .required(false),
                        )
                        .arg(
                            Arg::new("to-json")
                                .short('j')
                                .help("Toggle to-JSON conversion")
                                .required(false)
                                .action(clap::ArgAction::SetTrue),
                        )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("help", _)) => {
            println!("Log Parser CLI - Help");
            println!("Usage: log_parser_kma [COMMAND] [OPTIONS]");
            println!("Commands:");
            println!("  parse   Parse a log file with various options");
            println!("  help    Display this help message");
            println!("  credits Show credits");
            println!("Use `log_parser_kma --help` for more details on the parse command");
        },
        Some(("credits", _)) => {
            println!("Log Parser");
            println!("Made by Krachylo Illia");
            println!("For NaUKMA Rust Programming Course of 2024");
            println!("Version 0.1.2");
        },
        Some(("parse", parse_matches)) => {
            process(parse_matches)?;
        },
        _ => {
            println!("Unknown command")
        }
    }

   
    Ok(())
}

fn process(matches: &ArgMatches) -> Result<()> {
    let output_path = matches
        .get_one::<String>("output")
        .expect("Output path is required!");
    let loglevel = matches.get_one::<String>("level");
    let to_json = matches.get_flag("to-json");

    if let Some(filepath) = matches.get_one::<String>("file") {
        let result = LogParser::parse_file(filepath)?;

        let filtered: Vec<LogLine> = if let Some(level) = loglevel {
            let level_enum = match level.as_str() {
                "INFO" => LogLevel::Info,
                "WARNING" => LogLevel::Warning,
                "ERROR" => LogLevel::Error,
                _ => return Err(anyhow::anyhow!("Invalid log level")),
            };
            result
                .into_iter()
                .filter(|line| line.level == level_enum)
                .collect()
        } else {
            result
        };

        let mut file = File::create(output_path)?;

        if to_json {
            let json_logs = serde_json::to_string(&filtered)?;
            writeln!(file, "{}", json_logs)?;
        } else {
            for l in filtered {
                writeln!(
                    file,
                    "{} {} {}",
                    l.datetime,
                    log_level_string(&l.level),
                    l.message
                )?;
            }
        }
    }

    Ok(())
}

fn log_level_string(level: &LogLevel) -> &str {
    match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
    }
}
