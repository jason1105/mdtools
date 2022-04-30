//! A lib for basic file operations.

use std::ffi::OsString;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::path::PathBuf;
use std::{fs::OpenOptions, path::Path};

pub fn copy<T: AsRef<Path>>(src: T, dst: T) -> Result<()> {
    let mut buf = String::new();

    let mut file_for_read = OpenOptions::new().read(true).open(src).unwrap();
    file_for_read.read_to_string(&mut buf)?;

    let mut file_for_write = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dst)
        .unwrap();
    file_for_write.write_all(buf.as_bytes())?;

    Ok(())
}

pub fn remove_file<T: AsRef<Path>>(file: T) -> Result<()> {
    if Path::new(file.as_ref()).exists() {
        // to avoid permission denied, don't unwrap
        let _ = std::fs::remove_file(file.as_ref());
    }

    Ok(())
}

/// Replace content in file specified by position of start and end with new content.
pub fn replace_in_file(
    file: &OsString,
    start: usize,
    end: usize,
    new_content: String,
) -> Result<()> {
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

    let buf = buf.replace(&buf[start..end], &format!("{}\n", new_content));
    writer.write_all(buf.as_bytes())?;

    Ok(())
}

/// List all file in given path
pub fn list_all_files(dir: &PathBuf) -> Vec<OsString> {
    use walkdir::WalkDir;
    let mut paths = vec![];
    WalkDir::new(dir)
        .into_iter()
        .filter(|f| f.as_ref().unwrap().file_type().is_file())
        .for_each(|entry| {
            paths.push(entry.unwrap().path().as_os_str().to_os_string());
        });
    paths
}
