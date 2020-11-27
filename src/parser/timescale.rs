use crate::parser::helpers::contents_as_string;
use crate::parser::CommandParser;
use crate::parser::Parser;
use crate::types::TimeScale;
use nom::IResult;

impl Parser<TimeScale> for CommandParser {
    fn parse(i: &str) -> IResult<&str, TimeScale> {
        let (input, timescale) = contents_as_string(i, "$timescale")?;
        Ok((input, TimeScale(timescale)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            CommandParser::parse("$timescale 1 ps $end"),
            Ok(("", TimeScale("1 ps".to_string())))
        );

        assert_eq!(
            CommandParser::parse("$timescale 1 ns $end"),
            Ok(("", TimeScale("1 ns".to_string())))
        );
        assert_eq!(
            CommandParser::parse("$timescale 2 us $end"),
            Ok(("", TimeScale("2 us".to_string())))
        );
    }

    #[test]
    fn extra_whitespace() {
        assert_eq!(
            CommandParser::parse("$timescale  3    ms $end"),
            Ok(("", TimeScale("3 ms".to_string())))
        );
        assert_eq!(
            CommandParser::parse("   $timescale  3  ms  $end"),
            Ok(("", TimeScale("3 ms".to_string())))
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
            Ok(("", TimeScale("4 ns".to_string())))
        );
    }
}
