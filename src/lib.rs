//! `mdtools` is a library for managing markdown files.
pub mod commands;
pub mod env_settings;
pub mod file_utils;
pub mod git_utils;
mod prelude {
    pub use crate::commands::*;
    pub use crate::env_settings::*;
    pub use crate::file_utils;
    pub use crate::git_utils::MDGit;
    pub use clap::Args;
    pub use clap::{Parser, Subcommand};
    pub use lazy_static::lazy_static;
    pub use log::{debug, info, warn};
    pub use regex::Regex;
    pub use std::collections::HashMap;
    pub use std::collections::HashSet;
    pub use std::fs;
    pub use std::fs::OpenOptions;
    pub use std::io::BufRead;
    pub use std::path::Path;
    pub use std::process::Command;
    pub use std::process::Output;
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
    pub use walkdir::WalkDir;
}
