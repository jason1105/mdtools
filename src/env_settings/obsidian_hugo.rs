pub use crate::prelude::*;

pub struct MySetting<T>
where
    T: AsRef<Path>,
{
    pub obi_file: T, // obsidian file path
    post_dir: T,
    pub img_dir: T,
    pub lines: Vec<String>,
    real_imgs: HashMap<String, PathBuf>,
    hugo_file: PathBuf,
}

impl<T> MySetting<T>
where
    T: AsRef<Path>,
{
    pub fn new(path: T, pub_dir: T, img_dir: T) -> Self {
        let mut setting = MySetting {
            obi_file: path,
            post_dir: pub_dir,
            img_dir,
            lines: vec![],
            real_imgs: HashMap::default(),
            hugo_file: PathBuf::default(),
        };
        setting
    }

    pub fn prelude(&mut self) {
        self.lines = file_utils::read_file(&self.obi_file).expect("Failed to load file:");
        self.real_imgs = self.extract_img();
    }

    pub fn copy_to_hugo(&mut self) -> Result<()> {
        file_utils::copy(&self.obi_file, &self.post_dir)?;
        self.hugo_file = self
            .post_dir
            .as_ref()
            .join(PathBuf::from(&self.obi_file.as_ref().file_name().unwrap()));
        self.lines = file_utils::read_file(&self.hugo_file)?;
        Ok(())
    }

    pub fn copy_img_to_hugo(&self, img: &str) -> Result<String> {
        file_utils::copy(
            self.real_imgs.get(img).unwrap().as_ref(),
            self.img_dir.as_ref(),
        )?;
        Ok(self
            .img_dir
            .as_ref()
            .join(img)
            .to_str()
            .unwrap()
            .to_string())
    }

    pub fn load_file(&mut self) -> Result<()> {
        self.lines = file_utils::read_file(&self.obi_file)?;
        Ok(())
    }
}

impl<T> ObsidianFile<PathBuf> for MySetting<T>
where
    T: AsRef<Path>,
{
    /// Find out all image in Obsidian
    /// return <key: image name, key: image path>
    fn extract_img(&mut self) -> HashMap<String, PathBuf> {
        let ob_home = {
            let mut ret = None;
            let mut dir = self.obi_file.as_ref();
            while let Some(path) = dir.parent() {
                let mut pb = PathBuf::from(path);
                pb.push(".git");
                if pb.exists() {
                    ret = Some(path);
                }
                dir = dir.parent().unwrap();
            }
            ret
        };

        let mut map = HashMap::new();

        ob_home.and_then::<&Path, _>(|f| -> Option<_> {
            WalkDir::new(f)
                .into_iter()
                .filter(|f| f.as_ref().unwrap().file_type().is_file())
                .filter(|f| {
                    match Path::new(
                        f.as_ref()
                            .unwrap()
                            .file_name()
                            .to_str()
                            .unwrap()
                            .to_lowercase()
                            .as_str(),
                    )
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    {
                        "png" | "jpg" | "jpeg" | "bmp" | "gif" => true,
                        _ => false,
                    }
                })
                .for_each(|entry| {
                    map.insert(
                        entry
                            .as_ref()
                            .unwrap()
                            .file_name()
                            .to_str()
                            .unwrap()
                            .to_string(),
                        PathBuf::from(entry.unwrap().path()),
                    );
                });

            None
        });

        return map;
    }
}

impl<T> MarkdownFile for MySetting<T>
where
    T: AsRef<Path>,
{
    fn save(&self, is_obsidian: bool) -> Result<()> {
        let file = if is_obsidian {
            self.obi_file.as_ref()
        } else {
            self.hugo_file.as_ref()
        };

        let mut file = OpenOptions::new().write(true).truncate(true).open(file)?;

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
    /// update draft status
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
                *line = "draft: ".to_owned() + status.to_string().as_str() + "\n";
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

    /// translate imgs in Obsidian file to local img, return list including local img.
    /// ![[pasted img.png]] -> ![](/image/posts/"pasted img.png")
    /// copy "pasted img.png" to $HUGO_HOME/static/image/posts/
    fn localize_img(&mut self) -> Vec<&PathBuf> {
        let local_img = Regex::new(r"!\[\[(.*?)\]\]").unwrap();
        let map = &self.real_imgs;
        let real: Vec<&PathBuf> = self
            .lines
            .iter_mut()
            .filter_map(|line| {
                let mut real_imgs = vec![];
                let mut match_info: Vec<(usize, usize, &str)> = vec![]; // (start, end, img)

                let t_line = line.clone();
                debug!("find line: {}", line);
                for caps in local_img.captures_iter(t_line.as_str()) {
                    match_info.push((
                        caps.get(0).unwrap().start(),
                        caps.get(0).unwrap().end(),
                        caps.get(1).unwrap().as_str(),
                    ));
                    debug!("find a img: {}", caps.get(0).unwrap().as_str());
                    let img_name = caps.get(1).unwrap().as_str();
                }
                debug!("This will be replaced: {:?}", match_info);
                while let Some((start, end, img)) = match_info.pop() {
                    match map.get(img) {
                        Some(path) => {
                            // let hugo_img = self.copy_img_to_hugo(path.to_str().unwrap()).unwrap();
                            real_imgs.push(path);
                            line.replace_range(
                                start..end,
                                format!(
                                    "{}{}{}",
                                    "![](",
                                    "/images/posts/".to_owned()
                                        + img.to_string().replace(" ", "%20").as_ref(),
                                    ")"
                                )
                                .as_str(),
                            );
                        }
                        None => {}
                    }
                }
                // match_info.iter().rev().for_each(|(start, end, label)| {
                //     line.replace_range(*start..*end, label);
                // });
                Some(real_imgs)
            })
            .flat_map(|f| f)
            .collect();

        real
    }

    fn home_dir(&self) -> Result<PathBuf> {
        let mut dir = self.post_dir.as_ref();
        while let Some(path) = dir.parent() {
            let mut pb = PathBuf::from(path);
            pb.push("config.toml");
            if pb.exists() {
                return Ok(path.to_path_buf());
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
        fn run_cmd(path: &std::path::PathBuf) {
            if cfg!(target_os = "windows") {
                let add_cmd = Command::new("git")
                    .arg("add")
                    .arg(".")
                    .current_dir(&path)
                    .output();
                handle_cmd_output("git add", add_cmd, "git add command failed to start");

                let commit_cmd_output = Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg("\"auto commit\"")
                    .current_dir(&path)
                    .output();
                handle_cmd_output(
                    "git commit",
                    commit_cmd_output,
                    "git commit  command failed to start",
                );

                let push_cmd = Command::new("git").arg("push").current_dir(&path).output();
                handle_cmd_output("git push", push_cmd, "git push command failed to start");
            }
        }

        fn handle_cmd_output(cmd: &str, output: Result<Output>, msg: &str) {
            info!("Command \"{}\" have been called, output: {:?}", cmd, output);
            let output = output.expect(msg);

            match output.status.code() {
                Some(code) if code <= 1 => println!("Command \"{}\" success.", cmd),
                _ => {
                    eprintln!(
                        "Command \"{}\" failed: {}",
                        cmd,
                        String::from_utf8_lossy(&output.stderr)
                    );
                    // panic!()
                }
            }
        }

        let hugo_home = self.home_dir().expect("No hugo home be seen.");
        run_cmd(&hugo_home);

        Ok(())
    }

    fn commit_and_push(&self) -> Result<()> {
        todo!()
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
    #[test]
    fn test_to_imgs() {
        let mut setting = MySetting::new("c:/", "c:/", "c:/");
        let mut lines = vec![];
        lines.push("![[pasted image111.png]]".to_string());
        lines.push("![[pasted image222.png]] ![[pasted image333.png]]".to_string());
        lines.push("![[pasted image999.png]]".to_string());
        setting.lines = lines;
        setting.real_imgs = HashMap::new();
        setting.real_imgs.insert(
            "pasted image111.png".to_string(),
            PathBuf::from("c:/test/pasted image111.png"),
        );
        setting.real_imgs.insert(
            "pasted image222.png".to_string(),
            PathBuf::from("c:/test/pasted image222.png"),
        );
        setting.real_imgs.insert(
            "pasted image333.png".to_string(),
            PathBuf::from("c:/test/pasted image333.png"),
        );
        setting.real_imgs.insert(
            "pasted image999.png".to_string(),
            PathBuf::from("c:/test/pasted image999.png"),
        );
        setting.real_imgs.insert(
            "pasted image333.png".to_string(),
            PathBuf::from("c:/test/pasted image333.png"),
        );
        let vec = setting.localize_img();

        vec.iter()
            .for_each(|x| println!("{}", x.to_str().unwrap_or_default()));

        let a = setting
            .lines
            .iter()
            .map(|x| {
                println!("{}", x);
                x
            })
            .count();
    }

    #[test]
    fn test_real_imgs() {
        let mut setting = MySetting::new(
            PathBuf::from(
                "C:/my-projects/Lv-s-blog/blogs/English/100%20kilometers%20north%20of%20....md",
            ),
            PathBuf::from(
                "C:/my-projects/Lv-s-blog/blogs/English/100%20kilometers%20north%20of%20....md",
            ),
            PathBuf::from(
                "C:/my-projects/Lv-s-blog/blogs/English/100%20kilometers%20north%20of%20....md",
            ),
        );

        setting.real_imgs.iter().for_each(|(k, v)| {
            println!("key: {}, val = {}", k, v.as_path().to_str().unwrap());
        });
    }
}
