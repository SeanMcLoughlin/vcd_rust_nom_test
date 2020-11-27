use crate::parser::helpers::contents_as_string;
use crate::parser::CommandParser;
use crate::parser::Parser;
use crate::types::Version;
use nom::IResult;

impl Parser<Version> for CommandParser {
    fn parse(i: &str) -> IResult<&str, Version> {
        let (input, version) = contents_as_string(i, "$version")?;
        Ok((input, Version(version)))
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
