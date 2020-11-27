extern crate nom;
extern crate strum;
extern crate strum_macros;
#[macro_use]
extern crate derive_builder;

mod error;
mod parser;
pub mod types;
pub mod vcd;

use crate::parser::parse;
use error::VCDParserError;
use std::fs::File;
use std::io::BufReader;
use vcd::VCD;

pub fn load_from_file(file_name: &str) -> Result<VCD, VCDParserError> {
    parse(BufReader::new(File::open(file_name)?))
}

pub fn load_from_contents(file_contents: &str) -> Result<VCD, VCDParserError> {
    parse(BufReader::new(file_contents.as_bytes()))
}
