use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self};
use std::fs::File;
use anyhow::{Context, Result};
use clap::Parser;
use ansi_term::Colour::{Cyan, Black};

#[derive(Parser)]
struct Cli {
    pattern: String,
    paths: Vec<std::path::PathBuf>,
}

fn find_matches(content: impl Iterator<Item = String>, pattern: &str, mut writer: impl std::io::Write) {
    for line in content {
        if line.contains(pattern) {
            let splitted: Vec<&str> = line.split(pattern).collect();
            for (i, s) in splitted.iter().enumerate() {
                if i != 0 {
                    write!(writer, "{}", Black.on(Cyan).paint(pattern)).unwrap();
                }

                write!(writer, "{}", s).unwrap();

                if i == splitted.len() - 1 {
                    writeln!(writer).unwrap();
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    for path in args.paths {
        let f = File::open(&path).with_context(|| format!("could not read file `{}`", &path.display()))?;
        let reader = BufReader::new(f);
        let content = reader.lines().map(|l| l.expect("could not parse line"));

        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout);

        find_matches(content, &args.pattern, &mut handle);
    }

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let content = vec![String::from("lorem ipsum"), String::from("dolor sit amet")];
    find_matches(content.into_iter(), "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
