/// Publish markdown file to gitee pages.
/// 1. Load md file. Change "draft" to false or add "draft" if it was lack.
/// 2. Copy md file and imgs to git repo used for pages.
/// 3. Commit git repo and push.
use crate::prelude::*;

/** Subcommand `publish`
```
 mdtools publish --mdfile "/c/my-projects/Lv-s-blog/blogs/Development/Rust 模拟 No. 018 有效回文.md" --pub-dir /c/my-projects/lv-wei/content/en/posts --img-dir /c/my-projects/lv-wei/static
```
*/
#[derive(Args, Debug)]
pub struct Publish {
    // publish
    // markdown file in Obsidian
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    mdfile: PathBuf,
    // directory in hugo where posts in it
    #[clap(short, long, parse(from_os_str), value_name = "DIR")]
    pub_dir: PathBuf,
    // directory in hugo where save all imgs for post
    #[clap(short, long, parse(from_os_str), value_name = "DIR")]
    resource_dir: PathBuf,
    // whether commit or not (when using git)
    #[clap(short, long)]
    commit: Option<bool>,
}

/// Implements the `RunCommand` trait.
impl RunCommand for Publish {
    fn run(&self) -> Result<()> {
        self.publish()
    }
}

/// Implements the command of Publish
impl Publish {
    /// Entry point of the command `publish()`.
    fn publish(&self) -> Result<()> {
        let Self {
            mdfile,
            pub_dir,
            resource_dir,
            commit,
        } = self;

        let mut s = MySetting::new(mdfile, pub_dir, resource_dir);
        s.prelude();
        // Obsidian
        s.set_draft(false);
        s.save(true)?;
        s.copy_to_hugo()?;
        // Hugo
        let img_dir = resource_dir.join(PathBuf::from("images/posts/"));
        if !img_dir.exists() {
            fs::create_dir_all(&img_dir).expect("Failed to create dir.");
        }
        s.localize_img().iter().for_each(|img| {
            // copy img from obsidian to hugo img dir
            // debug!("{}", img.as_os_str().to_str().unwrap());
            let _ = file_utils::copy(img, &&img_dir);
        });
        s.save(false)?;
        // Git
        if matches!(commit, Some(true)) {
            s.add_all()?;
        }

        Ok(())
    }
}

mod test {

    use std::path::PathBuf;

    use super::Publish;

    #[test]
    fn test_public() {
        let cmd = Publish {
            mdfile: PathBuf::from(r"C:\my-projects\Lv-s-blog\blogs\易\周易和风水.md"),
            pub_dir: PathBuf::from(r"C:\my-projects\hugo\zzo_site\content\en\posts"),
            resource_dir: PathBuf::from(r"C:\my-projects\hugo\zzo_site\static"),
            commit: None,
        };
        cmd.publish();
    }
}
