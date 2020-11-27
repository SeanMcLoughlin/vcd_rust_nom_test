use crate::error::VCDParserError;
use crate::vcd::VCD;
use nom::IResult;
use std::io::{BufRead, BufReader};

mod date;
mod helpers;
mod timescale;
mod version;

trait Parser<T> {
    fn parse(i: &str) -> IResult<&str, T>;
}

#[allow(dead_code)]
struct CommandParser;

pub fn parse<R>(buffer: BufReader<R>) -> Result<VCD, VCDParserError>
where
    R: std::io::Read,
{
    for line in buffer.lines() {
        for _word in line
            .expect("Unable to read line")
            .split(|c: char| c.is_whitespace())
        {}
    }
    Ok(VCD::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn basic() {
        let contents = "$timescale 1 ps $end";
        let _ = parse(BufReader::new(contents.as_bytes())).unwrap();
    }
}
