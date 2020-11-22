extern crate nom_vcd_rust_test;

use nom_vcd_rust_test::parse_timescale::*;
use std::fs;
use std::path::PathBuf;

fn get_test_file_path(file_name: &str) -> String {
    let mut test_file_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file_buf.push(format!("tests/golden_files/{}", file_name));
    test_file_buf.as_path().display().to_string()
}

fn get_file_contents(file_name: &str) -> String {
    fs::read_to_string(get_test_file_path(file_name)).expect("Failed to read file")
}
