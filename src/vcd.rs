use crate::types::{Date, TimeScale, Version};

#[derive(Builder, Default, Eq, PartialEq, Debug)]
pub struct VCD {
    pub date: Date,
    pub version: Version,
    pub timescale: TimeScale,
}
