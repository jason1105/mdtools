use hugox::{
    add_tag::{do_add_tag, extract_tag_line},
    file_utils,
};
use lazy_static::lazy_static;
use std::{ffi::OsString, fs::OpenOptions, io::Read};

// "./tests/30daysOS-第02天 汇编语言和Makefile入门.md"
lazy_static! {
    pub static ref FILE: OsString =
        OsString::from("./tests/30daysOS-第02天 汇编语言和Makefile入门.md");
    pub static ref ORIGIN_FILE: OsString = OsString::from("./tests/file.md");
}

#[test]
fn test_extract_tag_line() {
    file_utils::copy(&ORIGIN_FILE.to_str().unwrap(), &FILE.to_str().unwrap()).unwrap();
    let (line, start, end) = extract_tag_line(&FILE).unwrap();
    assert_eq!(line, "tags: [学习笔记, 学习, 笔记]\n");
    assert_eq!(start, 37);
    assert_eq!(end, 74);
    //std::fs::remove_file(&FILE.to_str().unwrap()).unwrap();
    file_utils::remove_file(&FILE.to_str().unwrap()).unwrap();
}

#[test]
fn test_do_add_tag() {
    // prepare file for test
    file_utils::copy(&ORIGIN_FILE.to_str().unwrap(), &FILE.to_str().unwrap()).unwrap();

    let new_tags = vec![
        "这是一个测试字符串".to_string(),
        "这是另一个测试字符串".to_string(),
    ];

    do_add_tag(&new_tags, &FILE);

    let mut buf = String::new();
    let mut file_for_read = OpenOptions::new()
        .read(true)
        .open(&FILE.to_str().unwrap())
        .unwrap();
    file_for_read.read_to_string(&mut buf).unwrap();

    new_tags.iter().for_each(|tag| {
        assert!(buf.contains(tag));
    });

    println!("file: {:?}", file_for_read);

    //std::fs::remove_file(&FILE.to_str().unwrap()).unwrap();
    file_utils::remove_file(&FILE.to_str().unwrap()).unwrap();
}
