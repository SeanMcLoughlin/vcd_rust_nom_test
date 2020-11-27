use strum_macros::EnumString;

#[derive(Debug, Eq, PartialEq, EnumString, Clone, Copy)]
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

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct TimeScale(pub String);

impl From<&str> for TimeScale {
    fn from(s: &str) -> Self {
        TimeScale(String::from(s))
    }
}

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct Version(pub String);

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        Version(String::from(s))
    }
}

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct Date(pub String);

impl From<&str> for Date {
    fn from(s: &str) -> Self {
        Date(String::from(s))
    }
}
