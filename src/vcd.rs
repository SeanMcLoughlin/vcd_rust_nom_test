use crate::parser::{CommandParser, Parser};
use crate::types::{CommandName, Date, TimeScale, Version};

#[derive(Builder, Default, Eq, PartialEq, Debug)]
pub struct VCD {
    #[builder(setter(into), default)]
    pub date: Date,
    #[builder(setter(into), default)]
    pub version: Version,
    #[builder(setter(into), default)]
    pub timescale: TimeScale,
}

impl VCD {
    pub fn write_cmd(&mut self, command: CommandName, contents: &str) {
        // TODO: Fix duplicate code
        // TODO: Remove unwrap
        // TODO: Clean up tuple .1 reference
        match command {
            CommandName::Timescale => self.timescale = CommandParser::parse(contents).unwrap().1,
            CommandName::Date => self.date = CommandParser::parse(contents).unwrap().1,
            CommandName::Version => self.version = CommandParser::parse(contents).unwrap().1,
            _ => {}
        }
    }
}
