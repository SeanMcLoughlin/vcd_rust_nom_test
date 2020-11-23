use crate::helpers::strip;
use crate::parser::command_definition_parser::CommandDefinitionParser;
use crate::parser::Parser;
use crate::types::{TimeScale, TimeUnit};
use nom::bytes::complete::take_while1;
use nom::sequence::tuple;
use nom::{AsChar, IResult};
use std::str::FromStr;

impl Parser<TimeScale> for CommandDefinitionParser {
    fn parse(i: &str) -> IResult<&str, TimeScale> {
        let time_value = strip(take_while1(move |c: char| c.is_digit(10)));
        let time_unit = strip(take_while1(move |c: char| c.is_alpha()));
        let (input, (time_value, time_unit)) = tuple((time_value, time_unit))(i)?;

        Ok((
            input,
            TimeScale(
                time_value.parse().unwrap(),
                TimeUnit::from_str(time_unit).unwrap(),
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            CommandDefinitionParser::parse("1 ns"),
            Ok(("", TimeScale(1, TimeUnit::NS)))
        );
        assert_eq!(
            CommandDefinitionParser::parse("2 us"),
            Ok(("", TimeScale(2, TimeUnit::US)))
        );
    }

    #[test]
    fn extra_whitespace() {
        assert_eq!(
            CommandDefinitionParser::parse("3    ms"),
            Ok(("", TimeScale(3, TimeUnit::MS)))
        );
        assert_eq!(
            CommandDefinitionParser::parse("  3  ms  "),
            Ok(("", TimeScale(3, TimeUnit::MS)))
        );
        assert_eq!(
            CommandDefinitionParser::parse("3  ms "),
            Ok(("", TimeScale(3, TimeUnit::MS)))
        );
    }

    #[test]
    fn newlines() {
        assert_eq!(
            CommandDefinitionParser::parse(
                r#"4
        ns"#
            ),
            Ok(("", TimeScale(4, TimeUnit::NS)))
        );
    }
}
