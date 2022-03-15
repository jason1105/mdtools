use std::{
    collections::BTreeSet,
    ffi::OsString,
    io::{BufReader, Error, ErrorKind, Result},
    path::PathBuf,
};

use clap::Args;
use regex::Regex;

use crate::{file_utils, RunCommand};

static TAG_LINE_PATTERN: &str = r"^\s*tags\s*:\s*[ ]*\[.*\]\s*";

/// Subcommand `add_tag`
#[derive(Args, Debug)]
pub struct AddTag {
    /// Tags to be added.
    #[clap(required = true)]
    tags: Vec<String>,
    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    path: PathBuf,
}

/// Implements the `RunCommand` trait.
impl RunCommand for AddTag {
    fn run(&self) {
        self.add_tag()
    }
}

/// Implements the command of add-tag
impl AddTag {
    /// Entry point of the command add_tag.
    fn add_tag(&self) {
        let Self { tags, path } = self;

        // get files
        let files = file_utils::list_all_files(path);
        // add tags
        for file in files {
            println!("Updating file : {}", file.as_os_str().to_str().unwrap());
            do_add_tag(&tags, &file);
        }
        println!("Complete.");
    }
}

/// Add tags to file.
pub fn do_add_tag(new_tags: &Vec<String>, file: &OsString) {
    let (line, start, end) = extract_tag_line(&file).unwrap();
    let new_line = extend_tag(line, new_tags);
    file_utils::replace_in_file(&file, start, end, new_line).unwrap();
}

/// Extend tag line with new tags.
///
/// # Examples
///
/// ```text
///     use hugox::add_tag::extend_tag;
///     let line = "tags: [a, b, c]";
///     let new_tags = vec!["d".to_string(), "e".to_string()];
///     let result = extend_tag(line.to_string(), &new_tags);
///     assert_eq!(result, "tags: [a, b, c, d, e]");
/// ```
fn extend_tag(old_tag_line: String, new_tags: &Vec<String>) -> String {
    let mut tags = old_tag_line
        .split_once("tags")
        .unwrap()
        .1
        .split_once(":")
        .unwrap()
        .1
        .split_once("[")
        .unwrap()
        .1
        .split_once("]")
        .unwrap()
        .0
        .split(",")
        .map(|s| s.trim().to_string())
        .collect::<BTreeSet<String>>();
    tags.extend(new_tags.iter().cloned());

    format!(
        "tags: [{}]",
        tags.into_iter().collect::<Vec<String>>().join(", ")
    )
}

/// Extract tag line from given file. As well as start and end position of the tag line.
///
/// For example, `note.md` included a frontmatter like this:
///
/// ```text
/// // note.md
/// ---
/// title: "30天自制操作系统"
/// tags: [学习笔记, 学习, 笔记]
/// draft: false
/// description: "30天自制操作系统"
/// source: "http://hrb.osask.jp/"
/// ---
/// ```
///
/// ```no_run
///     use hugox::add_tag::extract_tag_line;
///     use std::ffi::OsString;
///
///     let (line, start, end) = extract_tag_line(&OsString::from("note.md")).unwrap();
///     assert_eq!(line, "tags: [学习笔记, 学习, 笔记]\n");
///     assert_eq!(start, 16);
///     assert_eq!(end, 53);
/// ```
pub fn extract_tag_line(file: &OsString) -> Result<(String, usize, usize)> {
    use std::io::BufRead;

    use std::fs::OpenOptions;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file)
        .unwrap();

    let re = Regex::new(TAG_LINE_PATTERN).unwrap();

    let mut reader = BufReader::new(file);
    let (mut i, mut _j) = (0, 0);
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => return Err(Error::new(ErrorKind::Other, "No tag line found")),
            Ok(size) => {
                if re.is_match(&line) {
                    _j = i + size;
                    return Ok((String::from(line), i, _j));
                } else {
                    i += size;
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                return Err(e);
            }
        }
    }
}

#[test]
fn test_modify_tag() {
    let line = "tags: [a, b, c]";
    let new_tags = vec!["d".to_string(), "e".to_string()];
    let result = extend_tag(line.to_string(), &new_tags);
    assert_eq!(result, "tags: [a, b, c, d, e]");

    let re = Regex::new(TAG_LINE_PATTERN).unwrap();
    assert_eq!(re.is_match(line), true);
}

#[test]
fn test_regex() {
    let re = Regex::new(TAG_LINE_PATTERN).unwrap();
    let line = "tags: [a, b, c]";
    assert_eq!(re.is_match(line), true);
    let line = "  tags : [ a ,  b,  c  ] ";
    assert_eq!(re.is_match(line), true);
}
