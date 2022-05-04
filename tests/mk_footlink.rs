use lazy_static::lazy_static;
use mdtools::commands::make_footlink::*;
use mdtools::file_utils;
use std::ffi::OsString;

lazy_static! {
    pub static ref TEST_FILE: OsString = OsString::from("./tests/test_result.md");
    pub static ref ORIGIN_FILE: OsString = OsString::from("./tests/file.md");
}

/// Only run but no verity.
#[test]
fn test_do_command() {
    file_utils::copy(&ORIGIN_FILE.to_str().unwrap(), &TEST_FILE.to_str().unwrap()).unwrap();
    let mut file = MDFile::new(TEST_FILE.as_os_str());
    file.make_footlinks().unwrap();
    file_utils::remove_file(&TEST_FILE.to_str().unwrap()).unwrap();
}
