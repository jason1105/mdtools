use lazy_static::lazy_static;
use mdtools::commands::add_tag::*;
use mdtools::file_utils;
use std::{ffi::OsString, fs::OpenOptions, io::Read};

lazy_static! {
    pub static ref TEST_FILE: OsString = OsString::from("./tests/test_result.md");
    pub static ref ORIGIN_FILE: OsString = OsString::from("./tests/file.md");
}

#[test]
fn test_extract_tag_line() {
    file_utils::copy(&ORIGIN_FILE.to_str().unwrap(), &TEST_FILE.to_str().unwrap()).unwrap();
    let (line, start, end) = extract_tag_line(&TEST_FILE).unwrap();
    assert_eq!(line, "tags: [web, book, note]\n");
    assert_eq!(start, 21);
    assert_eq!(end, 45);
    //std::fs::remove_file(&FILE.to_str().unwrap()).unwrap();
    file_utils::remove_file(&TEST_FILE.to_str().unwrap()).unwrap();
}

#[test]
fn test_do_add_tag() {
    // prepare file for test
    file_utils::copy(&ORIGIN_FILE.to_str().unwrap(), &TEST_FILE.to_str().unwrap()).unwrap();

    let new_tags = vec!["This_is_a_test_string".to_string(), "This_too".to_string()];

    add_new_tag(&new_tags, &TEST_FILE);

    let mut buf = String::new();
    let mut file_for_read = OpenOptions::new()
        .read(true)
        .open(&TEST_FILE.to_str().unwrap())
        .unwrap();
    file_for_read.read_to_string(&mut buf).unwrap();

    new_tags.iter().for_each(|tag| {
        assert!(buf.contains(tag));
    });

    println!("file: {:?}", file_for_read);

    //std::fs::remove_file(&FILE.to_str().unwrap()).unwrap();
    file_utils::remove_file(&TEST_FILE.to_str().unwrap()).unwrap();
}
