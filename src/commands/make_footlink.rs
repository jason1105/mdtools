use std::path::Path;

use crate::prelude::*;

// Pattern for links
static LINK_PATTERN: &str = r"(?P<title>\[.+?\])(?P<link>\(.+?\))";
static FOOT_LINK_PATTERN: &str = r"(?P<title>\[.+?\]) ?\[(?P<label>.+?)]";

lazy_static! {
    pub static ref REG_NORMAL_LINK: Regex = Regex::new(LINK_PATTERN).unwrap();
    pub static ref REG_FOOT_LINK: Regex = Regex::new(FOOT_LINK_PATTERN).unwrap();
    pub static ref FOOT_LABELS: HashSet<usize> = HashSet::new();
}

/// Subcommand `mk-footlinks`
#[derive(Args, Debug)]
pub struct MakeFootlink {
    /// File path or directory path
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    path: PathBuf,
}

/// Implements the `RunCommand` trait for mk-footlinks
impl RunCommand for MakeFootlink {
    fn run(&self) -> Result<()> {
        let Self { path } = self;

        // get files
        let files = file_utils::list_all_files(path);

        // makes foot links
        for file in files {
            info!("Updating file : {}", file.as_os_str().to_str().unwrap());
            let mut mdfile = MDFile::new(file);
            mdfile.make_footlinks()?;
        }

        Ok(())
    }
}

/// Format a label for foot link
fn format_footer_label<T>(label: T) -> String
where
    T: Display,
{
    format!("[{}]", label)
}

pub struct MDFile<T>
where
    T: AsRef<Path>,
{
    path: T,
    lines: Vec<String>,
    footlinks: Vec<FootLink>,
    foot_label_idx: u32,
    has_pervious_footlink: bool,
}

impl<T> MDFile<T>
where
    T: AsRef<Path>,
{
    pub fn new(path: T) -> Self {
        Self {
            path,
            lines: vec![],
            footlinks: vec![],
            foot_label_idx: 1,
            has_pervious_footlink: false,
        }
    }

    /// Main function
    pub fn make_footlinks(&mut self) -> Result<()> {
        self.init_foot_label_idx()?;
        self.prepare_content()?;
        self.flush_content()
    }

    fn init_foot_label_idx(&mut self) -> Result<()> {
        let file = OpenOptions::new().read(true).open(&self.path)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        let mut numbers: Vec<i32> = vec![];

        while let Ok(i) = reader.read_line(&mut line) {
            if i > 0 {
                for caps in REG_FOOT_LINK.captures_iter(&line) {
                    let label = &caps["label"].to_string();
                    match label.parse::<i32>() {
                        Ok(i) => numbers.push(i),
                        Err(_) => (),
                    }
                }
            } else {
                break;
            }
        }

        numbers.sort();
        match numbers.pop() {
            Some(max) if max > 0 => {
                self.foot_label_idx = (max + 1) as u32;
                self.has_pervious_footlink = true
            }
            Some(_) => (),
            None => (),
        }

        Ok(())
    }

    fn prepare_content(&mut self) -> Result<()> {
        let file = OpenOptions::new().read(true).open(&self.path)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        while let Ok(i) = reader.read_line(&mut line) {
            if i > 0 {
                self.add_line(line.clone());
                line.clear();
            } else {
                break;
            }
        }

        Ok(())
    }

    fn flush_content(&mut self) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        self.lines.iter().for_each(|line| {
            if let Err(e) = file.write(line.as_bytes()) {
                panic!("{}", e);
            }
        });

        if !self.footlinks.is_empty() {
            if !self.has_pervious_footlink {
                file.write_all("\n\n---\n".as_bytes())?
            }

            self.footlinks.iter().for_each(|footlink| {
                if let Err(e) = file.write_all("\n".as_bytes()) {
                    panic!("{}", e);
                } else if let Err(e) = file.write(footlink.to_string().as_bytes()) {
                    panic!("{}", e);
                }
            });
            file.flush()?;
        }

        Ok(())
    }

    fn add_line(&mut self, mut line: String) {
        let mut position: Vec<(usize, usize, String)> = vec![]; // start, end, idx.  [abc](http://abc) -> (5, 16, "footlinks-1")

        for caps in REG_NORMAL_LINK.captures_iter(&line) {
            let link = caps.name("link");

            println!(
                "Found link: {}\x1b[33m{}\x1b[0m",
                &caps["title"], &caps["link"]
            );

            info!(
                "Found link: start: {}, end: {}, link: {}",
                link.unwrap().start(),
                link.unwrap().end(),
                &caps["link"],
            );

            let start = link.unwrap().start();
            let end = link.unwrap().end(); // exclude index
            let label = format_footer_label(self.foot_label_idx);
            position.push((start, end, label.clone()));
            self.footlinks
                .push(FootLink::new(&caps["link"], label.clone()));
            self.foot_label_idx += 1;
        }

        position.iter().rev().for_each(|(start, end, label)| {
            let start = *start as usize;
            let end = *end as usize;
            line.replace_range(start..end, label);
        });
        self.lines.push(line);
    }
}

struct FootLink {
    link: String,
    label: String,
}

impl FootLink {
    fn new(link: &str, label: String) -> Self {
        Self {
            link: (&link[1..link.len() - 1]).to_string(),
            label,
        }
    }
}

impl Display for FootLink {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{}: {}", &self.label, &self.link))
    }
}

#[test]
fn test_add_line() {
    let mut md = MDFile::new("");
    let line = "sfasfsaf[abc](http://abc)..sdfsdfs.......sfsf[123](http://123)".into();
    md.add_line(line)
}
