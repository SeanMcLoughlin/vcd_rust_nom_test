use crate::error::VCDParserError;
use crate::types::CommandName;
use crate::vcd::VCD;
use nom::IResult;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod date;
mod helpers;
mod timescale;
mod version;

pub trait Parser<T> {
    fn parse(i: &str) -> IResult<&str, T>;
}

#[allow(dead_code)]
pub struct CommandParser;

pub fn parse<R>(buffer: BufReader<R>) -> Result<VCD, VCDParserError>
where
    R: std::io::Read,
{
    let mut vcd = VCD::default();
    let mut cur_cmd_buf = String::new();
    let mut current_command: CommandName = CommandName::Timescale;
    for line in buffer.lines() {
        for word in line
            .expect("Unable to read line")
            .split(|c: char| c.is_whitespace())
        {
            cur_cmd_buf += " ";
            cur_cmd_buf += word;
            if word.starts_with('$') {
                if word == "$end" {
                    vcd.write_cmd(current_command, &cur_cmd_buf[..]);
                    cur_cmd_buf = String::new();
                } else {
                    let word_wo_dollar = &word[1..];
                    // TODO: Handle unwrap
                    current_command = CommandName::from_str(word_wo_dollar).unwrap();
                }
            }
        }
    }
    Ok(vcd)
}
