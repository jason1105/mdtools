use std::io::{Read, Result, Write};
use std::{ffi::OsString, fs::OpenOptions, path::Path};

pub fn copy<T: AsRef<Path>>(src: T, dst: T) -> Result<()> {
    let mut buf = String::new();

    let mut file_for_read = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(src)
        .unwrap();
    file_for_read.read_to_string(&mut buf)?;

    let mut file_for_write = OpenOptions::new()
        .write(true)
        .create(true)
        .open(dst)
        .unwrap();
    file_for_write.write_all(buf.as_bytes())?;

    Ok(())
}
