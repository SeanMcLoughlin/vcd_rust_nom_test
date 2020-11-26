use nom::IResult;

mod command_parser;

trait Parser<T> {
    fn parse(i: &str) -> IResult<&str, T>;
}
