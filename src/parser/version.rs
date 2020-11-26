use crate::parser::CommandParser;
use crate::parser::Parser;
use crate::types::Version;
use nom::bytes::complete::{tag, take_until};
use nom::sequence::tuple;
use nom::IResult;

impl Parser<Version> for CommandParser {
    fn parse(i: &str) -> IResult<&str, Version> {
        let command = tag("$version");
        let version = take_until("$end");
        let end = tag("$end");
        let (input, (_, version, _)) = tuple((command, version, end))(i)?;
        Ok((input, Version(version.trim().to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            CommandParser::parse("$version won point oh $end"),
            Ok(("", Version("won point oh".to_string())))
        );

        assert_eq!(
            CommandParser::parse("$version Version 2.3 $end"),
            Ok(("", Version("Version 2.3".to_string())))
        );
    }
}
