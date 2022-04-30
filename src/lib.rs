//! `mdtools` is a library for managing markdown files.
pub mod commands;
pub mod file_utils;
mod prelude {
    pub use crate::commands::*;
    pub use crate::file_utils;
    pub use clap::Args;
    pub use clap::{Parser, Subcommand};
    pub use regex::Regex;
    pub use std::{
        collections::BTreeSet,
        ffi::OsString,
        io::{BufReader, Error, ErrorKind, Result},
        path::PathBuf,
    };
}
