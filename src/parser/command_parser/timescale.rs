use crate::helpers::strip;
use crate::parser::command_parser::CommandParser;
use crate::parser::Parser;
use crate::types::{TimeScale, TimeUnit};
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::sequence::tuple;
use nom::{AsChar, IResult};
use std::str::FromStr;

fn timescale_inside(i: &str) -> IResult<&str, TimeScale> {
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

impl Parser<TimeScale> for CommandParser {
    fn parse(i: &str) -> IResult<&str, TimeScale> {
        let space = take_while(|c: char| c == ' ');
        let command = tag("$timescale");
        let end = tag("$end");
        let (input, (_, _, timescale, _)) = tuple((space, command, timescale_inside, end))(i)?;
        Ok((input, timescale))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            CommandParser::parse("$timescale 1 ps $end"),
            Ok(("", TimeScale(1, TimeUnit::PS)))
        );

        assert_eq!(
            CommandParser::parse("$timescale 1 ns $end"),
            Ok(("", TimeScale(1, TimeUnit::NS)))
        );
        assert_eq!(
            CommandParser::parse("$timescale 2 us $end"),
            Ok(("", TimeScale(2, TimeUnit::US)))
        );
    }

    #[test]
    fn extra_whitespace() {
        assert_eq!(
            CommandParser::parse("$timescale  3    ms $end"),
            Ok(("", TimeScale(3, TimeUnit::MS)))
        );
        assert_eq!(
            CommandParser::parse("   $timescale  3  ms  $end"),
            Ok(("", TimeScale(3, TimeUnit::MS)))
        );
    }

    #[test]
    fn newlines() {
        assert_eq!(
            CommandParser::parse(
                r#"$timescale
            4
        ns
$end"#
            ),
            Ok(("", TimeScale(4, TimeUnit::NS)))
        );
    }
}
