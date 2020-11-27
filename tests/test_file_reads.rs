extern crate vcd_rust;

use std::fs;
use std::path::PathBuf;

fn get_test_file_path(file_name: &str) -> String {
    let mut test_file_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file_buf.push(format!("tests/golden_files/{}", file_name));
    test_file_buf.as_path().display().to_string()
}

#[allow(dead_code)]
fn get_file_contents(file_name: &str) -> String {
    fs::read_to_string(get_test_file_path(file_name)).expect("Failed to read file")
}

#[cfg(test)]
mod tests {
    use super::*;
    use vcd_rust::{types::*, vcd::VCDBuilder};

    #[test]
    fn test_preamble() {
        let act_vcd = vcd_rust::load_from_file(&get_test_file_path("preamble.vcd")[..]).unwrap();
        let exp_vcd = VCDBuilder::default()
            .date(Date(String::from("August 9th, 2020")))
            .version(Version(String::from("1.0")))
            .timescale(TimeScale(String::from("1 ps")))
            .build()
            .unwrap();
        assert_eq!(exp_vcd, act_vcd);
    }
}
