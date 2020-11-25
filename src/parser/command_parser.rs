use crate::parser::command_definition_parser::CommandDefinitionParser;
use crate::parser::Parser;
use crate::types::{Date, TimeScale, Version};
use nom::bytes::complete::{tag, take_until};
use nom::sequence::tuple;
use nom::IResult;

#[allow(dead_code)]
struct CommandParser;

impl Parser<TimeScale> for CommandParser {
    fn parse(i: &str) -> IResult<&str, TimeScale> {
        let command = tag("$timescale");
        let end = tag("$end");
        let (input, (_, timescale, _)) = tuple((command, CommandDefinitionParser::parse, end))(i)?;
        Ok((input, timescale))
    }
}

impl Parser<Version> for CommandParser {
    fn parse(i: &str) -> IResult<&str, Version> {
        let command = tag("$version");
        let version = take_until("$end");
        let end = tag("$end");
        let (input, (_, version, _)) = tuple((command, version, end))(i)?;
        Ok((input, Version(version.to_string())))
    }
}

impl Parser<Date> for CommandParser {
    fn parse(i: &str) -> IResult<&str, Date> {
        let command = tag("$date");
        let date = take_until("$end");
        let end = tag("$end");
        let (input, (_, date, _)) = tuple((command, date, end))(i)?;
        Ok((input, Date(date.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{TimeScale, TimeUnit};

    #[test]
    fn basic_timescale() {
        assert_eq!(
            CommandParser::parse("$timescale 1 ps $end"),
            Ok(("", TimeScale(1, TimeUnit::PS)))
        );
    }

    #[test]
    fn basic_version() {
        assert_eq!(
            CommandParser::parse("$version won point oh $end"),
            Ok(("", Version(" won point oh ".to_string()))) // TODO: How to strip space?
        );
    }

    #[test]
    fn basic_date() {
        assert_eq!(
            CommandParser::parse("$date September 24th, 2020 $end"),
            Ok(("", Date(" September 24th, 2020 ".to_string()))) // TODO: How to strip space?
        );
    }
}
