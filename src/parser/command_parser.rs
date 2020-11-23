use crate::parser::command_definition_parser::CommandDefinitionParser;
use crate::parser::Parser;
use crate::types::{TimeScale, Version};
use nom::bytes::complete::{tag, take_until};
use nom::sequence::tuple;
use nom::IResult;

#[allow(dead_code)]
struct CommandParser;

impl Parser<TimeScale> for CommandParser {
    fn parse(i: &'static str) -> IResult<&'static str, TimeScale> {
        let command = tag("$timescale");
        let end = tag("$end");
        let (input, (_, timescale, _)) = tuple((command, CommandDefinitionParser::parse, end))(i)?;
        Ok((input, timescale))
    }
}

impl<'a> Parser<Version<'a>> for CommandParser {
    fn parse(i: &'static str) -> IResult<&'static str, Version<'a>> {
        let command = tag("$version");
        let version = take_until("$end");
        let end = tag("$end");
        let (input, (_, version, _)) = tuple((command, version, end))(i)?;
        Ok((input, Version(version)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{TimeScale, TimeUnit};

    #[test]
    fn basic() {
        assert_eq!(
            CommandParser::parse("$timescale 1 ps $end"),
            Ok(("", TimeScale(1, TimeUnit::PS)))
        );

        assert_eq!(
            CommandParser::parse("$version won point oh $end"),
            Ok(("", Version(" won point oh "))) // TODO: How to strip space?
        );
    }
}
