use std::path;

use crate::prelude::*;

/// A trait used to define the command line interface of a program.
pub mod add_tag;
pub mod make_footlink;
pub mod publish;

pub trait RunCommand {
    fn run(&self) -> Result<()>;
}

pub trait ObsidianFile<T>
where
    T: AsRef<Path>,
{
    fn to_local_imgs(&self) -> Vec<T>;
    fn to_local_links(&self) -> Vec<T>;
}

pub trait HugoFile<T>
where
    T: AsRef<Path>,
{
    fn set_draft(&mut self, status: bool);
    fn home_dir(&self) -> Result<T>;
    fn post_dir(&self) -> Result<T>;
    fn img_dir(&self) -> Result<T>;
}

pub trait MarkdownFile {
    fn save(&self) -> Result<()>;
}

pub trait MDGit<T>
where
    T: AsRef<Path>,
{
    fn add_all(&self) -> Result<()>;
    fn commit_and_push(&self) -> Result<()>;
}
