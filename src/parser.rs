use nom::IResult;

mod date;
mod timescale;
mod version;

trait Parser<T> {
    fn parse(i: &str) -> IResult<&str, T>;
}

#[allow(dead_code)]
struct CommandParser;
