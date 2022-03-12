pub mod file_utils;
pub mod add_tag;

pub trait RunCommand {
    fn run(&self);
}
