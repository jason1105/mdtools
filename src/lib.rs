//! `mdtools` is a library for managing markdown files.
pub mod commands;
pub mod file_utils;
mod prelude {
    pub use crate::commands::*;
    pub use crate::file_utils;
    pub use clap::Args;
    pub use clap::{Parser, Subcommand};
    pub use lazy_static::lazy_static;
    pub use log::{debug, info, warn};
    pub use regex::Regex;
    pub use std::collections::HashSet;
    pub use std::fs::OpenOptions;
    pub use std::io::BufRead;
    pub use std::{
        collections::BTreeSet,
        ffi::OsString,
        io::{BufReader, Error, ErrorKind, Result},
        path::PathBuf,
    };
    pub use std::{
        fmt::{Display, Formatter},
        io::Write,
    };
}
