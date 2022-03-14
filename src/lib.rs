pub mod add_tag;
pub mod file_utils;

/// A trait used to define the command line interface of a program.
pub trait RunCommand {
    fn run(&self);
}
