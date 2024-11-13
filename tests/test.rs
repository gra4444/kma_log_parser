use anyhow::Result;
use chrono::*;
use log_parser_kma::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_correct() -> Result<()> {
        let test_time1 = "22:22:10";
        assert!(LogParser::parse(Rule::time, test_time1).is_ok());

        let test_time2 = "11:11:11";
        assert!(LogParser::parse(Rule::time, test_time2).is_ok());

        let test_time3 = "00:00:00";
        assert!(LogParser::parse(Rule::time, test_time3).is_ok());

        Ok(())
    }

    #[test]
    fn test_time_error() -> Result<()> {
        let incorrect_hour = "24:10:11";
        assert!(LogParser::parse(Rule::time, incorrect_hour).is_err());

        let incorrect_hour2 = "1:10:11";
        assert!(LogParser::parse(Rule::time, incorrect_hour2).is_err());

        let incorrect_min = "22:77:11";
        assert!(LogParser::parse(Rule::time, incorrect_min).is_err());

        let incorrect_sec = "22:22:77";
        assert!(LogParser::parse(Rule::time, incorrect_sec).is_err());

        Ok(())
    }

    #[test]
    fn test_date_correct() -> Result<()> {
        let test_date1 = "2024-11-06";
        assert!(LogParser::parse(Rule::date, test_date1).is_ok());

        let test_date2 = "2000-01-01";
        assert!(LogParser::parse(Rule::date, test_date2).is_ok());

        Ok(())
    }

    #[test]
    fn test_date_error() -> Result<()> {
        let incorrect_year = "900-11-05";
        let incorrect_month = "2024-13-05";
        let incorrect_day = "2024-11-0";
        let incorrect_date = "1-1-1";
        let incorrect_format = "2024 11 11";

        assert!(LogParser::parse(Rule::date, incorrect_year).is_err());
        assert!(LogParser::parse(Rule::date, incorrect_month).is_err());
        assert!(LogParser::parse(Rule::date, incorrect_day).is_err());
        assert!(LogParser::parse(Rule::date, incorrect_date).is_err());
        assert!(LogParser::parse(Rule::date, incorrect_format).is_err());

        Ok(())
    }

    #[test]
    fn test_datetime_correct() -> Result<()> {
        let correct = "2024-11-11 11:11:11";
        assert!(LogParser::parse(Rule::datetime, correct).is_ok());

        Ok(())
    }

    #[test]
    fn test_datetime_error() -> Result<()> {
        let incorrect_space = "2024-11-12  10:11:12";
        assert!(LogParser::parse(Rule::datetime, incorrect_space).is_err());

        let no_space = "2024-11-1210:11:12";
        assert!(LogParser::parse(Rule::datetime, no_space).is_err());

        let incorrect_date = "202-1-1 12:12:12";
        assert!(LogParser::parse(Rule::datetime, incorrect_date).is_err());

        let incorrect_time = "2024-11-11 12:60:59";
        assert!(LogParser::parse(Rule::datetime, incorrect_time).is_err());

        Ok(())
    }

    #[test]
    fn test_loglevel_correct() -> Result<()> {
        assert!(LogParser::parse(Rule::loglevel, "INFO").is_ok());
        assert!(LogParser::parse(Rule::loglevel, "WARNING").is_ok());
        assert!(LogParser::parse(Rule::loglevel, "ERROR").is_ok());

        Ok(())
    }

    #[test]
    fn test_loglevel_error() -> Result<()> {
        let trailing_spaces = " INFO   ";
        assert!(LogParser::parse(Rule::loglevel, trailing_spaces).is_err());

        let nonexistent_level = "NONEXISTENT";
        assert!(LogParser::parse(Rule::loglevel, nonexistent_level).is_err());

        Ok(())
    }

    #[test]
    fn test_message_correct() -> Result<()> {
        let msg = "Literally any string should be fine here";
        assert!(LogParser::parse(Rule::message, msg).is_ok());

        let msg_trailing =
            "   Even trailing spaces should theoretically be fine in this scenario  ";
        assert!(LogParser::parse(Rule::message, msg_trailing).is_ok());

        let msg_diff_chars = "1234567890qwertyuiop[asdfghjkl;'zxcvbnm,./!@#$%^&*()_+~čœæ";
        assert!(LogParser::parse(Rule::message, msg_diff_chars).is_ok());

        Ok(())
    }

    #[test]
    fn test_message_error() -> Result<()> {
        let linebreak = "\n";
        assert!(LogParser::parse(Rule::message, linebreak).is_err());

        let linebreak2 = "\r\n";
        assert!(LogParser::parse(Rule::message, linebreak2).is_err());

        Ok(())
    }

    #[test]
    fn test_logline_correct() -> Result<()> {
        let logline1 = "2024-11-06 22:22:00 ERROR Test error message";
        assert!(LogParser::parse(Rule::logline, logline1).is_ok());

        let logline2 = "2025-12-16 12:07:37 WARNING Something something warning warning)\n";
        assert!(LogParser::parse(Rule::logline, logline2).is_ok());

        let logline3 = "2024-11-06 22:22:00 INFO Such an informative message!\n";
        assert!(LogParser::parse(Rule::logline, logline3).is_ok());

        Ok(())
    }

    #[test]
    fn test_logline_error() -> Result<()> {
        let no_loglevel = "2024-11-06 22:22:00 Test error message";
        assert!(LogParser::parse(Rule::logline, no_loglevel).is_err());

        let no_datetime = "ERROR Missing datetime here";
        assert!(LogParser::parse(Rule::logline, no_datetime).is_err());

        let extra_space = "2024-11-06  22:22:00 ERROR Test error message";
        assert!(LogParser::parse(Rule::logline, extra_space).is_err());

        let no_message = "2024-11-06 22:22:00 ERROR";
        assert!(LogParser::parse(Rule::logline, no_message).is_err());

        Ok(())
    }

    #[test]
    fn test() -> Result<()> {
        let test_line = "2024-11-06 22:22:00 ERROR Test error message";
        let expected = LogLine {
            datetime: NaiveDate::from_ymd_opt(2024, 11, 6)
                .unwrap()
                .and_hms_opt(22, 22, 0)
                .unwrap(),
            level: LogLevel::Error,
            message: String::from("Test error message"),
        };
        let actual = LogLine::parse(test_line).unwrap();
        assert_eq!(expected, actual);

        Ok(())
    }
}
