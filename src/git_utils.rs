use crate::prelude::*;
pub trait MDGit<T>
where
    T: AsRef<Path>,
{
    fn add_all(&self) -> Result<()>;
    fn commit_and_push(&self) -> Result<()>;
}

struct MyGit;

impl MDGit<PathBuf> for MyGit {
    fn add_all(&self) -> Result<()> {
        todo!()
    }

    fn commit_and_push(&self) -> Result<()> {
        todo!()
    }
}
