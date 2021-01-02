use std::env;;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

const DAYS: u32 = 25;
const START_YEAR: u32 = 2015;
const END_YEAR: u32 = 2020;

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("missing path argument");
    let root = Path::new(&path);
    for year in START_YEAR..=END_YEAR {
        for day in 1..=DAYS {
            let path = create_dir(root, year, day);
            let readme = create_readme(&path);
            fill_readme(year, day, readme);
            process::exit(0);
            path.pop();
            path.push("input");
            fs::create_dir(&path)?;
            path.push("input.dat");
            let input = fs::OpenOptions::new().write(true).create_new(true).open(&path)?;
            path.pop();
            path.pop();
            path.push("src");
            fs::create_dir(&path)?;
            path.push("lib.rs");
            let source = fs::OpenOptions::new().write(true).create_new(true).open(&path)?;
            path.pop();
            path.pop();
            path.push("tests");
            fs::create_dir(&path)?;
            path.push("tests.rs");
            let tests = fs::OpenOptions::new().write(true).create_new(true).open(&path)?;
            path.pop();
        }
    }
    Ok(())
}

fn create_dir(root: &Path, year: u32, day: u32) -> PathBuf {
    let mut path = root.join(format!("aoc_{:4}", year));
    path.push(format!("day_{:02}", day));
    match fs::create_dir_all(&path) {
        Ok(_) => path,
        Err(e) => {
            eprintln!("failed to create directory '{}': {}", path.display(), e);
            process::exit(1);
        }
    }
}

fn create_readme(path: &Path) -> fs::File {
    let mut path = path.to_path_buf();
    path.push("README.md");
    match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("failed to create file '{}': {}", path.display(), e);
            process::exit(1);
        }
    }
}

fn fill_readme(year: u32, day: u32, readme: fs::File) {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    async {
        let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    };
}
