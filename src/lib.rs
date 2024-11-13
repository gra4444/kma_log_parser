#![doc = include_str!("../docs.md")]

use chrono::NaiveDateTime;
pub use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct LogParser;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LogLine {
    pub datetime: NaiveDateTime,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum LogParseError {
    #[error("Pest grammar error")]
    ParseError(Box<pest::error::Error<Rule>>),
    #[error("Invalid log level")]
    InvalidLogLevel,
    #[error("Invalid date time provided")]
    InvalidDateTime,
    #[error("Error reading file")]
    FileError(std::io::Error),
}

impl LogParser {
    pub fn parse_file(path: &str) -> Result<Vec<LogLine>, LogParseError> {
        let content = std::fs::read_to_string(path).map_err(LogParseError::FileError)?;

        let mut parsed_logs: Vec<LogLine> = Vec::new();

        for line in content.lines() {
            match LogLine::parse(line) {
                Ok(log_line) => parsed_logs.push(log_line),
                Err(e) => eprintln!("Failed to parse line: {:?}, err: {:?}", line, e),
            }
        }

        Ok(parsed_logs)
    }
}

impl LogLine {
    pub fn parse(line: &str) -> Result<LogLine, LogParseError> {
        let mut parse_result = LogParser::parse(Rule::logline, line)
            .map_err(|e| LogParseError::ParseError(Box::new(e)))?;
        let pair = parse_result.next().unwrap();

        let datetime_str = pair.clone().into_inner().next().unwrap().as_str();
        let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| LogParseError::InvalidDateTime)?;

        let level_str = pair.clone().into_inner().nth(1).unwrap().as_str();
        let level = match level_str {
            "INFO" => LogLevel::Info,
            "WARNING" => LogLevel::Warning,
            "ERROR" => LogLevel::Error,
            _ => return Err(LogParseError::InvalidLogLevel),
        };

        let message = pair.into_inner().last().unwrap().as_str().to_string();
        Ok(LogLine {
            datetime,
            level,
            message,
        })
    }
}
