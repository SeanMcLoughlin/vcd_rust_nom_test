use crate::parser::CommandParser;
use crate::parser::Parser;
use crate::types::Date;
use nom::bytes::complete::{tag, take_until};
use nom::sequence::tuple;
use nom::IResult;

impl Parser<Date> for CommandParser {
    fn parse(i: &str) -> IResult<&str, Date> {
        let command = tag("$date");
        let date = take_until("$end");
        let end = tag("$end");
        let (input, (_, date, _)) = tuple((command, date, end))(i)?;
        Ok((input, Date(date.trim().to_string())))
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
