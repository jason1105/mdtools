use std::{fmt::Display, io::Write};

use crate::prelude::*;

static LINK_PATTERN: &str = r"\[.+?\](?P<link>\(.+?\))";

lazy_static! {
    pub static ref REG: Regex = Regex::new(LINK_PATTERN).unwrap();
}

fn format_footer_label<T>(label: T) -> String
where
    T: Display,
{
    format!("[{}]", label)
}

/// Subcommand `add_tag`
#[derive(Args, Debug)]
pub struct MkFootlinks {
    /// File path or directory path
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    path: PathBuf,
}

/// Implements the `RunCommand` trait.
impl RunCommand for MkFootlinks {
    fn run(&self) {
        self.mk_footlinks();
    }
}

/// Implements the command of mk-footlinks
impl MkFootlinks {
    /// Entry point of the command `add_tag()`.
    fn mk_footlinks(&self) {
        let Self { path } = self;

        // get files
        let files = file_utils::list_all_files(path);
        // add tags
        for file in files {
            println!("Updating file : {}", file.as_os_str().to_str().unwrap());
            do_command(&file);
        }
        println!("Complete.");
    }
}

pub fn do_command(file: &OsString) {
    let write_file = file.clone();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file)
        .unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut mdfile = MDFile::new();
    while let Ok(i) = reader.read_line(&mut line) {
        if i > 0 {
            mdfile.add_line(line.clone());
            // println!("read{}: {}", i, line);
            line.clear();
        } else {
            break;
        }
    }

    // for debug
    mdfile
        .footlinks
        .iter()
        .for_each(|link| println!("{}", link.to_string()));

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(write_file)
        .unwrap();
    mdfile.lines.iter().for_each(|line| {
        file.write(line.as_bytes());
    });
    if mdfile.footlinks.len() > 0 {
        file.write("\n\n".as_bytes());
        mdfile.footlinks.iter().for_each(|footlink| {
            file.write(footlink.to_string().as_bytes());
        });
    }
}

struct MDFile {
    lines: Vec<String>,
    footlinks: Vec<FootLink>,
    idx: u32,
}

impl MDFile {
    fn new() -> Self {
        Self {
            lines: vec![],
            footlinks: vec![],
            idx: 1,
        }
    }
    fn add_line(&mut self, mut line: String) {
        let mut position: Vec<(usize, usize, String)> = vec![]; // start, end, idx.  [abc](http://abc) -> (5, 16, "footlinks-1")

        for caps in REG.captures_iter(&line) {
            let mat = caps.name("link");
            println!("Making footer at line: {}", &line);
            println!(
                "start: {}, end: {}, link: {}",
                mat.unwrap().start(),
                mat.unwrap().end(),
                &caps["link"],
            );
            let start = mat.unwrap().start();
            let end = mat.unwrap().end(); // exclude index
            let label = format_footer_label(self.idx);
            position.push((start, end, label.clone()));
            self.footlinks
                .push(FootLink::new(&caps["link"], label.clone()));
            self.idx += 1;
        }

        position.iter().rev().for_each(|(start, end, label)| {
            let start = *start as usize;
            let end = *end as usize;
            line.replace_range(start..end, label);
        });
        println!("Adding line: {}", &line);
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

    fn to_string(&self) -> String {
        format!("{}: {}\n", &self.label, &self.link)
    }
}

#[test]
fn test_add_line() {
    let mut md = MDFile::new();
    let line = "sfasfsaf[abc](http://abc)..sdfsdfs.......sfsf[123](http://123)".into();
    md.add_line(line)
}
