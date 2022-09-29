use std::vec;

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
        s.prelude();
        // Obsidian
        s.set_draft(false);
        s.save(true)?;
        s.copy_to_hugo()?;
        // Hugo
        // let path = PathBuf::from("_");
        // s.path = &path;
        // s.load_file();
        s.localize_img().iter().for_each(|img| {
            // copy to hugo img dir

            println!("{}", img.as_os_str().to_str().unwrap());
            file_utils::copy(img, &&self.img_dir);
        });
        s.save(false)?;
        // copy imgs from obsidian
        // Git
        s.add_all()?;
        s.commit_and_push()?;

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
            img_dir: PathBuf::from(r"C:\my-projects\hugo\zzo_site\static\images\posts"),
        };
        cmd.publish();
    }
}
