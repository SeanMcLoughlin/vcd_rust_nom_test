use strum_macros::EnumString;

#[derive(Debug, Eq, PartialEq, EnumString)]
pub enum CommandName {
    #[strum(serialize = "date")]
    Date,
    #[strum(serialize = "enddefinitions")]
    EndDefinitions,
    #[strum(serialize = "scope")]
    Scope,
    #[strum(serialize = "timescale")]
    Timescale,
    #[strum(serialize = "upscope")]
    UpScope,
    #[strum(serialize = "var")]
    Var,
    #[strum(serialize = "version")]
    Version,
    #[strum(serialize = "dumpall")]
    DumpAll,
    #[strum(serialize = "dumpoff")]
    DumpOff,
    #[strum(serialize = "dumpon")]
    DumpOn,
    #[strum(serialize = "dumpvars")]
    DumpVars,
    #[strum(serialize = "end")]
    End,
}

#[derive(Debug, Eq, PartialEq, EnumString)]
pub enum TimeUnit {
    #[strum(serialize = "ms")]
    MS,
    #[strum(serialize = "us")]
    US,
    #[strum(serialize = "ns")]
    NS,
    #[strum(serialize = "ps")]
    PS,
}

pub trait Command {}

#[derive(Debug, Eq, PartialEq)]
pub struct TimeScale(pub usize, pub TimeUnit);

impl Command for TimeScale {}