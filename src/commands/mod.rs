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
    fn extract_img(&mut self) -> HashMap<String, PathBuf>;
}

pub trait HugoFile<T>
where
    T: AsRef<Path>,
{
    fn set_draft(&mut self, status: bool);
    fn home_dir(&self) -> Result<T>;
    fn post_dir(&self) -> Result<T>;
    fn img_dir(&self) -> Result<T>;
    fn localize_img(&mut self) -> Vec<&T>;
}

pub trait MarkdownFile {
    fn save(&self, is_obsidian: bool) -> Result<()>;
}
