use std::{
    collections::{BTreeSet, HashSet},
    ffi::OsString,
    fs::File,
    io::{BufReader, BufWriter, Error, ErrorKind, Read, Result, Write},
    path::PathBuf,
};

use clap::Args;
use regex::Regex;

use crate::RunCommand;

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

impl RunCommand for AddTag {
    fn run(&self) {
        self.add_tag()
    }
}

impl AddTag {
    fn add_tag(&self) {
        println!("{:?}, {:?}", self.tags, self.path);
        let Self { tags, path } = self;

        // get files
        let files = list_matched_files(path);
        // add tags
        for file in files {
            do_add_tag(&tags, &file);
        }
    }
}

pub fn do_add_tag(new_tags: &Vec<String>, file: &OsString) {
    let (line, start, end) = extract_tag_line(&file).unwrap();
    let new_line = modify_tag(line, new_tags);
    replace_in_file(&file, start, end, new_line);
}

/// list all file in given path
fn list_matched_files(path: &PathBuf) -> Vec<OsString> {
    use walkdir::WalkDir;
    let mut paths = vec![];
    for entry in WalkDir::new(path) {
        println!("{}", entry.as_ref().unwrap().path().display());
        paths.push(entry.unwrap().path().as_os_str().to_os_string());
    }
    paths
}

/// # Examples
/// ```no_run
///     let line = "tags: [a, b, c]";
///     let new_tags = vec!["d".to_string(), "e".to_string()];
///     let result = modify_tag(line.to_string(), &new_tags);
///     assert_eq!(result, "tags: [a, b, c, d, e]");
/// ```
fn modify_tag(line: String, new_tags: &Vec<String>) -> String {
    let mut tags = line
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
    // tags.extend_from_slice(self.tags.as_slice());
    tags.extend(new_tags.iter().cloned());

    format!(
        "tags: [{}]",
        tags.into_iter().collect::<Vec<String>>().join(", ")
    )
}

// return (line, start_position, end_position)
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
    let (mut i, mut j) = (0, 0);
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => return Err(Error::new(ErrorKind::Other, "No tag line found")),
            Ok(size) => {
                if re.is_match(&line) {
                    j = i + size;
                    return Ok((String::from(line), i, j));
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

fn replace_in_file(file: &OsString, start: usize, end: usize, content: String) -> Result<()> {
    use std::fs::OpenOptions;

    let file1 = file.clone();
    let mut buf = String::new();
    {
        let file_for_read = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file1)?;

        let mut reader = BufReader::new(file_for_read);
        reader.read_to_string(&mut buf)?;
    }

    let file_for_write = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file)?;
    let mut writer = BufWriter::new(file_for_write);

    let buf = buf.replace(&buf[start..end], &format!("\n{}\n", content));
    writer.write_all(buf.as_bytes())?;

    Ok(())
}

#[test]
fn test_modify_tag() {
    let line = "tags: [a, b, c]";
    let new_tags = vec!["d".to_string(), "e".to_string()];
    let result = modify_tag(line.to_string(), &new_tags);
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
