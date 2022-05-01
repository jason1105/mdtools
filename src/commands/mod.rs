/// A trait used to define the command line interface of a program.
pub mod add_tag;
pub mod mk_footlinks;

pub trait RunCommand {
    fn run(&self);
}
