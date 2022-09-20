/// Publish markdown file to gitee pages.
/// 1. Load md file. Change "draft" to false or add "draft" if it was lack.
/// 2. Copy md file and imgs to git repo used for pages.
/// 3. Commit git repo and push.
/*

crate md::*;

let md = md::load(path);
let item = md.prelude.into_iter().find(|x| x.key == "draft");


*/
use crate::prelude::*;

/// Subcommand `publish`
#[derive(Args, Debug)]
pub struct Publish {
    // publish
    // markdown file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    mdfile: PathBuf,
    // directory in hugo where posts in it
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    pub_dir: PathBuf,
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    img_dir: PathBuf,
}

/// Implements the `RunCommand` trait.
impl RunCommand for Publish {
    fn run(&self) -> Result<()> {
        self.publish()
    }
}

struct MySetting<T>
where
    T: AsRef<Path>,
{
    path: T, // obsidian file path
    post_dir: T,
    img_dir: T,
    lines: Vec<String>,
}

impl<T> MySetting<T>
where
    T: AsRef<Path>,
{
    fn new(path: T, pub_dir: T, img_dir: T) -> Self {
        MySetting {
            path,
            post_dir: pub_dir,
            img_dir,
            lines: vec![],
        }
    }

    fn copy_to_hugo(&self) -> Result<()> {
        file_utils::copy(&self.path, &self.post_dir)
    }
}

impl<T> ObsidianFile<PathBuf> for MySetting<T>
where
    T: AsRef<Path>,
{
    fn to_local_imgs(&self) -> Vec<PathBuf> {
        todo!()
    }

    fn to_local_links(&self) -> Vec<PathBuf> {
        todo!()
    }
}

impl<T> MarkdownFile for MySetting<T>
where
    T: AsRef<Path>,
{
    fn save(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        self.lines.iter().for_each(|line| {
            if let Err(e) = file.write(line.as_bytes()) {
                panic!("{}", e);
            }
        });

        file.flush()?;

        Ok(())
    }
}

impl<T> HugoFile<PathBuf> for MySetting<T>
where
    T: AsRef<Path>,
{
    fn set_draft(&mut self, status: bool) {
        let mut in_front = None;
        let front_splitter = Regex::new(r"^\s*---\s*").unwrap();
        let draft = Regex::new(r"^\s*draft\s*:\s*\.*\s*").unwrap();
        let (mut start, mut end) = (0, 0);
        for (no, line) in &mut self.lines.iter_mut().enumerate() {
            if front_splitter.is_match(line) {
                in_front = match in_front {
                    Some(true) => {
                        end = no;
                        Some(false)
                    }
                    Some(false) => Some(false),
                    None => {
                        start = no;
                        Some(true)
                    }
                };
                if !in_front.unwrap() {
                    break;
                }
            }
            if in_front.unwrap_or(false) && draft.is_match(line) {
                *line = "draft: ".to_owned() + status.to_string().as_str();
                return;
            }
        }

        if end == 0 {
            self.lines.insert(0, "---".to_string());
            self.lines.insert(0, "---".to_string());
            self.lines
                .insert(1, "draft: ".to_owned() + status.to_string().as_str());
        } else {
            self.lines
                .insert(end, "draft: ".to_owned() + status.to_string().as_str());
        }
    }

    fn post_dir(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(&self.post_dir.as_ref()))
    }

    fn img_dir(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(&self.img_dir.as_ref()))
    }

    fn home_dir(&self) -> Result<PathBuf> {
        let mut dir = self.post_dir.as_ref();
        while let Some(path) = dir.parent() {
            let mut pb = PathBuf::from(path);
            pb.push("/config.toml");
            if pb.exists() {
                return Ok(pb);
            }
            dir = dir.parent().unwrap();
        }
        Err(std::io::Error::new(ErrorKind::AddrNotAvailable, "1234"))
    }
}

impl<T> MDGit<T> for MySetting<T>
where
    T: AsRef<Path>,
{
    fn add_all(&self) -> Result<()> {
        todo!()
    }

    fn commit_and_push(&self) -> Result<()> {
        todo!()
    }
}

/// Implements the command of add-tag
impl Publish {
    /// Entry point of the command `add_tag()`.
    fn publish(&self) -> Result<()> {
        let Self {
            mdfile,
            pub_dir,
            img_dir,
        } = self;

        let mut s = MySetting::new(mdfile, pub_dir, img_dir);

        s.set_draft(false);
        s.save()?;
        s.copy_to_hugo()?;
        s.to_local_imgs().iter().for_each(|img| {
            // copy to img_dir
        });
        s.add_all()?;
        s.commit_and_push()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_draft_false() {
        let mut setting = MySetting::new("c:/", "c:/", "c:/");
        let mut lines = vec![];
        lines.push("---".to_string());
        lines.push("title: Rust".to_string());
        lines.push("draft: true".to_string());
        lines.push("---".to_string());
        lines.push("".to_string());
        lines.push("# Introduce".to_string());
        setting.lines = lines;

        setting.set_draft(false);
        let count = setting
            .lines
            .iter()
            .map(|x| {
                println!("{}", x);
                x
            })
            .filter(|x| x.contains("draft: false"))
            .count();
        assert_eq!(1, count);
    }

    #[test]
    fn test_add_draft_false() {
        let mut setting = MySetting::new("c:/", "c:/", "c:/");
        let mut lines = vec![];
        lines.push("---".to_string());
        lines.push("title: Rust".to_string());
        lines.push("---".to_string());
        lines.push("".to_string());
        lines.push("# Introduce".to_string());
        setting.lines = lines;

        setting.set_draft(false);
        let count = setting
            .lines
            .iter()
            .map(|x| {
                println!("{}", x);
                x
            })
            .filter(|x| x.contains("draft: false"))
            .count();
        assert_eq!(1, count);
    }

    #[test]
    fn test_add_front_and_draft_false() {
        let mut setting = MySetting::new("c:/", "c:/", "c:/");
        let mut lines = vec![];
        lines.push("# Introduce".to_string());
        setting.lines = lines;

        setting.set_draft(false);
        let count = setting
            .lines
            .iter()
            .map(|x| {
                println!("{}", x);
                x
            })
            .filter(|x| x.contains("draft: false"))
            .count();
        assert_eq!(1, count);
    }
}
