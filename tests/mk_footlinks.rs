use lazy_static::lazy_static;
use mdtools::commands::mk_footlinks::*;
use mdtools::file_utils;
use std::ffi::OsString;

lazy_static! {
    pub static ref TEST_FILE: OsString = OsString::from("./tests/test_result.md");
    pub static ref ORIGIN_FILE: OsString = OsString::from("./tests/file.md");
}

#[test]
fn test_do_command() {
    file_utils::copy(&ORIGIN_FILE.to_str().unwrap(), &TEST_FILE.to_str().unwrap()).unwrap();
    do_command(&TEST_FILE);
    file_utils::remove_file(&TEST_FILE.to_str().unwrap()).unwrap();
}
