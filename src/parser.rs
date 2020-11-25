use nom::IResult;

mod command_definition_parser;
mod command_parser;

trait Parser<T> {
    fn parse(i: &str) -> IResult<&str, T>;
}
