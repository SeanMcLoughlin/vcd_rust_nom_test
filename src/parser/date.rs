use crate::parser::helpers::contents_as_string;
use crate::parser::CommandParser;
use crate::parser::Parser;
use crate::types::Date;

use nom::IResult;

impl Parser<Date> for CommandParser {
    fn parse(i: &str) -> IResult<&str, Date> {
        let (input, date) = contents_as_string(i, "$date")?;
        Ok((input, Date(date)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date() {
        assert_eq!(
            CommandParser::parse("$date September 24th, 2020 $end"),
            Ok(("", Date("September 24th, 2020".to_string())))
        );
    }
}
