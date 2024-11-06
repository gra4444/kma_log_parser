// use chrono::NaiveDateTime;
pub use pest::Parser;
use pest_derive::Parser;
// use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct LogParser;

#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

// #[derive(Debug, PartialEq)]
// pub struct LogLine {
//     pub datetime: NaiveDateTime,
//     pub level: LogLevel,
//     pub message: String,
// }

// #[derive(Debug, Error)]
// pub enum LogParseError {
//     #[error("Pest grammar error")]
//     PestError(pest::error::Error<Rule>),
//     #[error("Invalid log level")]
//     InvalidLogLevel,
//     #[error("Invalid date time provided")]
//     InvalidDateTime,
// }

// impl LogLine {
//     pub fn parse(line: &str) -> Result<LogLine, LogParseError> {
//         let parse_result = LogParser::parse(Rule::logline, line);
//         println!("LOG PARSER COMPLETED PARSE");
//         match parse_result {
//             Ok(mut pairs) => {
//                 println!("OK PAIRS");
//                 let pair = pairs.next().unwrap();

//                 let datetime_str = pair.clone().into_inner().next().unwrap().as_str();
//                 let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
//                     .map_err(|_| LogParseError::InvalidDateTime)?;

//                 let level_str = pair.clone().into_inner().nth(1).unwrap().as_str();
//                 let level = match level_str {
//                     "INFO" => LogLevel::Info,
//                     "WARNING" => LogLevel::Warning,
//                     "ERROR" => LogLevel::Error,
//                     _ => return Err(LogParseError::InvalidLogLevel),
//                 };

//                 let message = pair.into_inner().last().unwrap().as_str().to_string();
//                 println!("EVERYTHING PROCESSED");
//                 Ok(LogLine {
//                     datetime,
//                     level,
//                     message,
//                 })
//             }
//             Err(e) => {
//                 println!("ERROR OCCURED");
//                 Err(LogParseError::PestError(e))},
//         }
//     }
// }
